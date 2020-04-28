#![allow(unsafe_code)]
#![feature(alloc_error_handler)]

#![no_main]
#![no_std]

// Halt on panic
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{
    prelude::*,
    stm32,
    gpio,
    fmc::{
        FmcSdramExt,
        FmcSdramTiming, FmcSdramTimingNs,
        FmcSdramConfig, FmcSdramBank,
        SDRAM2_BASE_ADDRESS,
    },
};

extern crate alloc;
extern crate alloc_cortex_m;
use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
#[alloc_error_handler]
fn alloc_error(_: core::alloc::Layout) -> ! {
    panic!("Out of memory! :(");
}



#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LEDs. On the STM32F429I-DISC[O1] they are connected to pin PG13/14.
        let gpiog = dp.GPIOG.split();
        let mut led1 = gpiog.pg13.into_push_pull_output();
        let mut led2 = gpiog.pg14.into_push_pull_output();

        // Set up the system clock. We want to run at 168MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();
        
        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        // Setup sdram
        let gpiob = dp.GPIOB.split();
        let gpioc = dp.GPIOC.split();
        let gpiod = dp.GPIOD.split();
        let gpioe = dp.GPIOE.split();
        let gpiof = dp.GPIOF.split();
        let sdram_pins = (
            gpiog.pg8.into_alternate_af12().set_speed(gpio::Speed::Medium),  // Sdclk
            // sdcke0, sdne0 not needed
            gpiob.pb5.into_alternate_af12().set_speed(gpio::Speed::Medium),  // Sdcke1
            gpiob.pb6.into_alternate_af12().set_speed(gpio::Speed::Medium),  // Sdne1
            
            gpiog.pg4.into_alternate_af12().set_speed(gpio::Speed::Medium),  // Ba0
            gpiog.pg5.into_alternate_af12().set_speed(gpio::Speed::Medium),  // Ba1
            
            gpiof.pf11.into_alternate_af12().set_speed(gpio::Speed::Medium), // Sdnras
            gpiog.pg15.into_alternate_af12().set_speed(gpio::Speed::Medium), // Sdncas
            gpioc.pc0.into_alternate_af12().set_speed(gpio::Speed::Medium),  // Sdnwe
            
            gpioe.pe0.into_alternate_af12().set_speed(gpio::Speed::Medium),  // Nbl0
            gpioe.pe1.into_alternate_af12().set_speed(gpio::Speed::Medium),  // Nbl1
            
            // A0-12
            gpiof.pf0.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiof.pf1.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiof.pf2.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiof.pf3.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiof.pf4.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiof.pf5.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiof.pf12.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiof.pf13.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiof.pf14.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiof.pf15.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiog.pg0.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiog.pg1.into_alternate_af12().set_speed(gpio::Speed::Medium),
            
            // D0-15
            gpiod.pd14.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiod.pd15.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiod.pd0.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiod.pd1.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpioe.pe7.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpioe.pe8.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpioe.pe9.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpioe.pe10.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpioe.pe11.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpioe.pe12.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpioe.pe13.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpioe.pe14.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpioe.pe15.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiod.pd8.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiod.pd9.into_alternate_af12().set_speed(gpio::Speed::Medium),
            gpiod.pd10.into_alternate_af12().set_speed(gpio::Speed::Medium),
        );
        let config = FmcSdramConfig {
            bank: FmcSdramBank::BANK2,
            memory_width: stm32::fmc::sdcr::MWID_A::BITS16,
            num_rows: stm32::fmc::sdcr::NR_A::BITS12,
            num_columns: stm32::fmc::sdcr::NC_A::BITS8,
            cas_latency: stm32::fmc::sdcr::CAS_A::CLOCKS3,
            ..Default::default()
        };
        let trcd = 18;
        let trp  = 18;
        let tras = 42; // 50
        let trc  = 60; // 80
        let trfc = 60; // 80
        let trdl = 12;
        let txsr = trfc;
        let tmrd = 7;
        let timing = FmcSdramTiming::from_ns(
            &clocks,
            &config,
            FmcSdramTimingNs {
                rcd: trcd,
                rp:  trp,
                wr:  *[trdl, tras - trcd, trc - trcd - trp].iter().enumerate().max().unwrap().1,
                rc:  trc,
                ras: tras,
                xsr: txsr,
                mrd: tmrd,
                
                // 64ms refresh rate
                refresh: 64_000_000
            }
        );
        let _sdram = dp.FMC.setup_sdram(
            sdram_pins,
            &mut delay,
            config,
            timing
        );
        
        // Setup the heap allocator
        unsafe { ALLOCATOR.init(SDRAM2_BASE_ADDRESS, 8 * 1024 * 1024) }
        
        // Allocate new vector
        let mut my_vec: Vec<u32> = Vec::new();
        let mut my_val = 0u32;
        
        loop {
            // Green LED
            led1.set_high().unwrap();
            led2.set_low().unwrap();
            
            // Fill sdram with c
            for c in my_val..my_val+1_000_000 {
                my_vec.push(c);
            }
            
            // Red LED
            led1.set_low().unwrap();
            led2.set_high().unwrap();
            
            // Check ram
            for &c in &my_vec {
                assert!(c == my_val, "Sdram is faulty");
                my_val = my_val.wrapping_add(1);
            }

            // Clear the vector
            my_vec.clear();
        }
    }

    loop {}
}
