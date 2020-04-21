//! FSMC interface (ram and stuff..)

use core::mem;
use cortex_m::asm;
use stm32f4::ResetValue;
use embedded_hal::blocking::delay::DelayUs;

use crate::rcc::Clocks;
use crate::stm32;
use crate::stm32::FMC;
use crate::stm32::fmc::SDCMR;
use crate::stm32::fmc::sdcr::{CAS_A, MWID_A, NR_A, NC_A, RPIPE_A, SDCLK_A, NB_A, RBURST_A};
use crate::stm32::fmc::sdcmr::MODE_AW;

pub trait FmcBusyWait {
    fn busy_wait(&self) -> &Self;
}
impl FmcBusyWait for FMC {
    fn busy_wait(&self) -> &Self {
        asm::dsb();
        while self.sdsr.read().busy().is_busy() {}
        self
    }
}


pub const SDRAM1_BASE_ADDRESS:usize = 0xc000_0000;
pub const SDRAM2_BASE_ADDRESS:usize = 0xd000_0000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FmcSdramBank {
    BANK1,
    BANK2,
    BOTH_BANKS,
}

pub mod fmc_sdram_mode_cmd {
    pub const BURST_LENGTH_1:u16 = 0;
    pub const BURST_LENGTH_2:u16 = 1;
    pub const BURST_LENGTH_4:u16 = 2;
    pub const BURST_LENGTH_8:u16 = 3;
    pub const BURST_TYPE_SEQUENTIAL:u16  = 0 << 3;
    pub const BURST_TYPE_INTERLEAVED:u16 = 1 << 3;
    pub const CAS_LATENCY_1:u16 = 1 << 4;
    pub const CAS_LATENCY_2:u16 = 2 << 4;
    pub const CAS_LATENCY_3:u16 = 3 << 4;
    pub const OPERATING_MODE_STANDARD:u16 = 0 << 7;
    pub const OPERATING_MODE_TEST1:u16    = 1 << 7;
    pub const OPERATING_MODE_TEST2:u16    = 2 << 8;
    pub const OPERATING_MODE_TEST3:u16    = 3 << 8;
    pub const WRITEBURST_MODE_PROGRAMMED:u16 = 0 << 9;
    pub const WRITEBURST_MODE_SINGLE:u16     = 1 << 9;
}
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum FmcSdramBurstLength {
    Burst1 = fmc_sdram_mode_cmd::BURST_LENGTH_1,
    Burst2 = fmc_sdram_mode_cmd::BURST_LENGTH_2,
    Burst4 = fmc_sdram_mode_cmd::BURST_LENGTH_4,
    Burst8 = fmc_sdram_mode_cmd::BURST_LENGTH_8,
}


pub struct FmcSdramConfig {
    pub bank: FmcSdramBank,
    pub sdclk: SDCLK_A,
    pub memory_width: MWID_A,
    pub num_rows: NR_A,
    pub num_columns: NC_A,
    pub cas_latency: CAS_A,
    pub rpipe: RPIPE_A,
    pub nb: NB_A,
    pub read_burst: bool,
    pub burst_length: FmcSdramBurstLength,
    pub write_burst: bool,
}
impl Default for FmcSdramConfig {
    fn default() -> Self {
        FmcSdramConfig {
            bank: FmcSdramBank::BANK1,
            sdclk: SDCLK_A::DIV2,
            memory_width: MWID_A::BITS16,
            num_rows: NR_A::BITS12,
            num_columns: NC_A::BITS8,
            cas_latency: CAS_A::CLOCKS3,
            rpipe: RPIPE_A::CLOCKS1,
            nb: NB_A::NB4,
            read_burst: false,
            burst_length: FmcSdramBurstLength::Burst2,
            write_burst: false,
        }
    }
}
//impl FmcSdramConfig {
//    pub fn new() -> Self { Default::default() }
//}

pub struct FmcSdramTimingNs {
    /// RCD Delay
    pub rcd: u32,
    /// RP Delay
    pub rp: u32,
    /// Write Recovery Time
    pub wr: u32,
    /// Row Cycle Delay
    pub rc: u32,
    /// Self Refresh TIme
    pub ras: u32,
    /// Exit Self Refresh Time
    pub xsr: u32,
    /// Load to Active delay
    pub mrd: u32,
    
    /// Refresh counter value
    pub refresh: u32,
}
pub struct FmcSdramTiming {
    /// RCD Delay
    pub rcd: u8,
    /// RP Delay
    pub rp: u8,
    /// Write Recovery Time
    pub wr: u8,
    /// Row Cycle Delay
    pub rc: u8,
    /// Self Refresh TIme
    pub ras: u8,
    /// Exit Self Refresh Time
    pub xsr: u8,
    /// Load to Active delay
    pub mrd: u8,
    
    /// Refresh counter value
    pub refresh_counter: u16,
}
impl Default for FmcSdramTiming {
    fn default() -> Self {
        FmcSdramTiming {
            rcd: 2 - 1,
            rp: 2 - 1,
            wr: 2 - 1,
            rc: 7 - 1,
            ras: 4 - 1,
            xsr: 7 - 1,
            mrd: 2 - 1,
            refresh_counter: 1241,
        }
    }  
}
impl FmcSdramTiming {
    fn ns_to_clk(
        clocks: &Clocks,
        config: &FmcSdramConfig,
        ns: u32,
        sub: u32,
        min: u32,
    ) -> u64 {
        let nclk = (
            ns as u64 *
            clocks.hclk().0 as u64 / 
            match config.sdclk {
                SDCLK_A::DIV2 => 2,
                SDCLK_A::DIV3 => 3,
                _ => 0,
            } +
            1_000_000_000 - 1 // round up
        ) / 1_000_000_000;
        
        if nclk > (min + sub) as u64 {
            nclk - sub as u64
        } else {
            min as u64
        }
    }
    pub fn from_ns(
        clocks: &Clocks,
        config: &FmcSdramConfig,
        timings_ns: FmcSdramTimingNs
    ) -> FmcSdramTiming {
        // Set the refresh counter to refresh_time / num_rows * memclk - 20
        let refresh_counter:u64 =
                FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.refresh, 0, 0) /
                match config.num_rows {
                    NR_A::BITS11 => 1 << 11,
                    NR_A::BITS12 => 1 << 12,
                    NR_A::BITS13 => 1 << 13,
                } as u64;
        let refresh_counter = if refresh_counter > 61 { refresh_counter - 20 } else { 41 } as u16;
        
        FmcSdramTiming {
            rcd: FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.rcd, 1, 0) as u8,
            rp:  FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.rp, 1, 0) as u8,
            wr:  FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.wr, 1, 0) as u8,
            rc:  FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.rc, 1, 0) as u8,
            ras: FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.ras, 1, 0) as u8,
            xsr: FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.xsr, 1, 0) as u8,
            mrd: FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.mrd, 1, 0) as u8,
            refresh_counter,
        }
    }
}

pub trait FmcSdram {
    fn setup_sdram(
        &self,
        delay: &mut dyn DelayUs<u32>,
        config: FmcSdramConfig,
        timing: FmcSdramTiming
    ) -> &Self;
    fn sdram_command(
        &self,
        bank: FmcSdramBank,
        cmd: MODE_AW,
        autorefresh: u8,
        modereg: u16
    ) -> &Self;
}

impl FmcSdram for FMC {
    fn setup_sdram(
        &self,
        delay: &mut dyn DelayUs<u32>,
        config: FmcSdramConfig,
        timing: FmcSdramTiming
    ) -> &Self {
        cortex_m::interrupt::free(|_| {
        
            // Enable fmc clock
            let rcc = unsafe { &*stm32::RCC::ptr() };
            if !rcc.ahb3enr.read().fmcen().is_enabled() {
                // enable FMC (peripheral clock)
                rcc.ahb3enr.modify(|_, w| w.fmcen().enabled());
                // give FMC_CLK time to start
                let ok = rcc.ahb3enr.read().fmcen().is_enabled();
                assert!(ok);
    
                // reset the FMC
                rcc.ahb3rstr.modify(|_, w| w.fmcrst().set_bit());
                rcc.ahb3rstr.modify(|_, w| w.fmcrst().clear_bit());
    
                // verify the clock configuration is valid
                // TBD
            }

            // Configure sdram. sdclk, rburst, rpipe have to be set in sdcr1 (also for bank2)
            match config.bank {
                FmcSdramBank::BANK1 | FmcSdramBank::BOTH_BANKS => {
                    self.sdcr1.write(|w|
                        w.rpipe().variant(config.rpipe)
                         .sdclk().variant(config.sdclk)
                         .nb().variant(config.nb)
                         .cas().variant(config.cas_latency)
                         .mwid().variant(config.memory_width)
                         .nr().variant(config.num_rows)
                         .nc().variant(config.num_columns)
                         .rburst().variant(if config.read_burst  { RBURST_A::ENABLED } else { RBURST_A::DISABLED })
                         .wp().disabled() // disable write protection
                    );
                },
                FmcSdramBank::BANK2 => {
                    self.sdcr1.modify(|_,w|
                        w.rpipe().variant(config.rpipe)
                         .sdclk().variant(config.sdclk)
                         .rburst().variant(if config.read_burst  { RBURST_A::ENABLED } else { RBURST_A::DISABLED })
                    );
                },
            }
            match config.bank {
                FmcSdramBank::BANK2 | FmcSdramBank::BOTH_BANKS => {
                    self.sdcr2.write(|w|
                        w.nb().variant(config.nb)
                         .cas().variant(config.cas_latency)
                         .mwid().variant(config.memory_width)
                         .nr().variant(config.num_rows)
                         .nc().variant(config.num_columns)
                         .wp().disabled() // disable write protection
                    );
                },
                FmcSdramBank::BANK1 => {},
            }
                    
            // Setup timings. trp and trc have to be set in sdtr1
            match config.bank {
                FmcSdramBank::BANK1 | FmcSdramBank::BOTH_BANKS => {
                    self.sdtr1.write(|w|
                        w.trcd().bits(timing.rcd)
                         .trp().bits(timing.rp)
                         .twr().bits(timing.wr)
                         .trc().bits(timing.rc)
                         .tras().bits(timing.ras)
                         .txsr().bits(timing.xsr)
                         .tmrd().bits(timing.mrd)
                    );
                },
                FmcSdramBank::BANK2 => {
                    self.sdtr1.modify(|_,w|
                        w.trp().bits(timing.rp)
                         .trc().bits(timing.rc)
                    );
                },
            }
            match config.bank {
                FmcSdramBank::BANK2 | FmcSdramBank::BOTH_BANKS => {
                    self.sdtr2.write(|w|
                        w.trcd().bits(timing.rcd)
                         .twr().bits(timing.wr)
                         .tras().bits(timing.ras)
                         .txsr().bits(timing.xsr)
                         .tmrd().bits(timing.mrd)
                    );
                },
                FmcSdramBank::BANK1 => {},
            }
            
            let _sdcr1 = self.sdcr1.read().bits();
            let _sdcr2 = self.sdcr2.read().bits();
            let _sdtr1 = self.sdtr1.read().bits();
            let _sdtr2 = self.sdtr2.read().bits();
            
            // Enable sdram clocks
            self.sdram_command(config.bank, MODE_AW::CLOCKCONFIGURATIONENABLE, 0, 0);
            delay.delay_us(500); // sleep >>100us (sdram dependent)
            self.busy_wait();
    
            // Issue "Precharge all" command
            self.sdram_command(config.bank, MODE_AW::PALL, 0, 0);
            self.busy_wait();
    
            // Set number of autorefresh command that should be issued to 8 (sdram dependant)
            self.sdram_command(config.bank, MODE_AW::AUTOREFRESHCOMMAND, 8, 0);
            self.busy_wait();
            
            // Load the mode register
            self.sdram_command(
                config.bank,
                MODE_AW::LOADMODEREGISTER,
                0,
                config.burst_length as u16 |
                fmc_sdram_mode_cmd::BURST_TYPE_SEQUENTIAL |
                match config.cas_latency {
                    CAS_A::CLOCKS1 => fmc_sdram_mode_cmd::CAS_LATENCY_1,
                    CAS_A::CLOCKS2 => fmc_sdram_mode_cmd::CAS_LATENCY_2,
                    CAS_A::CLOCKS3 => fmc_sdram_mode_cmd::CAS_LATENCY_3,
                } |
                fmc_sdram_mode_cmd::OPERATING_MODE_STANDARD |
                if config.write_burst {
                    fmc_sdram_mode_cmd::WRITEBURST_MODE_PROGRAMMED
                } else {
                    fmc_sdram_mode_cmd::WRITEBURST_MODE_SINGLE
                });
            self.busy_wait();
            
            // Set the refresh counter
            self.sdrtr.write(|w| w.count().bits(timing.refresh_counter));
            
            // doing something completely unrelated :)
//            self.bcr1.modify(|_,w| w.muxen().enabled()); // Why?
//            self.bcr1.write(|w| unsafe { w.bits(BCR::reset_value()) }); // hö?
        });
        
        self
    }
    
    fn sdram_command(
        &self,
        bank: FmcSdramBank,
        cmd: MODE_AW,
        autorefresh: u8,
        modereg: u16
    ) -> &Self {
        cortex_m::interrupt::free(|_| {
            
            let sdcmr_init = mem::MaybeUninit::<SDCMR>::zeroed();
            unsafe { sdcmr_init.as_ptr().as_ref().unwrap().write(|w| w.bits(SDCMR::reset_value())) }
            let sdcmr_tmp = unsafe { sdcmr_init.assume_init() };
            
            match bank {
                FmcSdramBank::BANK1 => { sdcmr_tmp.modify(|_,w| w.ctb1().issued()) }
                FmcSdramBank::BANK2 => { sdcmr_tmp.modify(|_,w| w.ctb2().issued()) }
                FmcSdramBank::BOTH_BANKS => {
                    sdcmr_tmp.modify(|_,w|
                        w.ctb1().issued()
                         .ctb2().issued()
                    )
                }
            };
            
            sdcmr_tmp.modify(|_,w|
                w.nrfs().bits(autorefresh)
                 .mrd().bits(modereg)
                 .mode().variant(cmd)
            );
            
            // Wait for the next chance to talk to the controller
            self.busy_wait();
            
            /* Send the next command */
            let sdcmr_tmp = sdcmr_tmp.read().bits();
            self.sdcmr.write(|w| unsafe { w.bits(sdcmr_tmp) });
            
        });
        
        self
    }
}

