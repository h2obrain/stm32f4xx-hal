//! FSMC interface (ram and stuff..)

use core::mem;
use cortex_m::asm;
use embedded_hal::blocking::delay::DelayUs;
use stm32f4::ResetValue;

use crate::pin_defs::*;
use crate::rcc::Clocks;
use crate::stm32;
use crate::stm32::fmc::sdcmr::MODE_AW;
use crate::stm32::fmc::sdcr::{CAS_A, MWID_A, NB_A, NC_A, NR_A, RBURST_A, RPIPE_A, SDCLK_A};
use crate::stm32::fmc::SDCMR;
use crate::stm32::FMC;

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

pub const SDRAM1_BASE_ADDRESS: usize = 0xc000_0000;
pub const SDRAM2_BASE_ADDRESS: usize = 0xd000_0000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FmcSdramBank {
    BANK1,
    BANK2,
    BOTH_BANKS,
}

pub mod fmc_sdram_mode_cmd {
    pub const BURST_LENGTH_1: u16 = 0;
    pub const BURST_LENGTH_2: u16 = 1;
    pub const BURST_LENGTH_4: u16 = 2;
    pub const BURST_LENGTH_8: u16 = 3;
    pub const BURST_TYPE_SEQUENTIAL: u16 = 0 << 3;
    pub const BURST_TYPE_INTERLEAVED: u16 = 1 << 3;
    pub const CAS_LATENCY_1: u16 = 1 << 4;
    pub const CAS_LATENCY_2: u16 = 2 << 4;
    pub const CAS_LATENCY_3: u16 = 3 << 4;
    pub const OPERATING_MODE_STANDARD: u16 = 0 << 7;
    pub const OPERATING_MODE_TEST1: u16 = 1 << 7;
    pub const OPERATING_MODE_TEST2: u16 = 2 << 8;
    pub const OPERATING_MODE_TEST3: u16 = 3 << 8;
    pub const WRITEBURST_MODE_PROGRAMMED: u16 = 0 << 9;
    pub const WRITEBURST_MODE_SINGLE: u16 = 1 << 9;
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
    fn ns_to_clk(clocks: &Clocks, config: &FmcSdramConfig, ns: u32, sub: u32, min: u32) -> u64 {
        let nclk = (
            ns as u64 * clocks.hclk().0 as u64
                / match config.sdclk {
                    SDCLK_A::DIV2 => 2,
                    SDCLK_A::DIV3 => 3,
                    _ => 0,
                }
                + 1_000_000_000
                - 1
            // round up
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
        timings_ns: FmcSdramTimingNs,
    ) -> FmcSdramTiming {
        // Set the refresh counter to refresh_time / num_rows * memclk - 20
        let refresh_counter: u64 =
            FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.refresh, 0, 0)
                / match config.num_rows {
                    NR_A::BITS11 => 1 << 11,
                    NR_A::BITS12 => 1 << 12,
                    NR_A::BITS13 => 1 << 13,
                } as u64;
        let refresh_counter = if refresh_counter > 61 {
            refresh_counter - 20
        } else {
            41
        } as u16;

        FmcSdramTiming {
            rcd: FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.rcd, 1, 0) as u8,
            rp: FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.rp, 1, 0) as u8,
            wr: FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.wr, 1, 0) as u8,
            rc: FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.rc, 1, 0) as u8,
            ras: FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.ras, 1, 0) as u8,
            xsr: FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.xsr, 1, 0) as u8,
            mrd: FmcSdramTiming::ns_to_clk(clocks, config, timings_ns.mrd, 1, 0) as u8,
            refresh_counter,
        }
    }
}

pub trait FmcSdramExt<PINS> {
    fn setup_sdram(
        self: &Self,
        pins: PINS,
        delay: &mut dyn DelayUs<u32>,
        config: FmcSdramConfig,
        timing: FmcSdramTiming,
    ) -> FmcSdram<PINS>
    where
        PINS: Pins<FMC>;
}
impl<PINS> FmcSdramExt<PINS> for FMC {
    fn setup_sdram(
        &self, //: &Self,
        pins: PINS,
        delay: &mut dyn DelayUs<u32>,
        config: FmcSdramConfig,
        timing: FmcSdramTiming,
    ) -> FmcSdram<PINS>
    where
        PINS: Pins<FMC>,
    {
        FmcSdram::new(self, pins, delay, config, timing)
    }
}

#[allow(dead_code)]
pub struct FmcSdram</*FMC,*/ PINS> {
    //    fmc: FMC, // FIXME: we actually do not want to take fmc here :(
    pins: PINS,
}

impl<PINS> FmcSdram</*FMC,*/ PINS> {
    fn new(
        fmc: &FMC,
        pins: PINS,
        delay: &mut dyn DelayUs<u32>,
        config: FmcSdramConfig,
        timing: FmcSdramTiming,
    ) -> Self
    where
        PINS: Pins<FMC>,
    {
        let sdram = FmcSdram { /*fmc,*/ pins };

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
                    fmc.sdcr1.write(
                        |w| {
                            w.rpipe()
                                .variant(config.rpipe)
                                .sdclk()
                                .variant(config.sdclk)
                                .nb()
                                .variant(config.nb)
                                .cas()
                                .variant(config.cas_latency)
                                .mwid()
                                .variant(config.memory_width)
                                .nr()
                                .variant(config.num_rows)
                                .nc()
                                .variant(config.num_columns)
                                .rburst()
                                .variant(if config.read_burst {
                                    RBURST_A::ENABLED
                                } else {
                                    RBURST_A::DISABLED
                                })
                                .wp()
                                .disabled()
                        }, // disable write protection
                    );
                }
                FmcSdramBank::BANK2 => {
                    fmc.sdcr1.modify(|_, w| {
                        w.rpipe()
                            .variant(config.rpipe)
                            .sdclk()
                            .variant(config.sdclk)
                            .rburst()
                            .variant(if config.read_burst {
                                RBURST_A::ENABLED
                            } else {
                                RBURST_A::DISABLED
                            })
                    });
                }
            }
            match config.bank {
                FmcSdramBank::BANK2 | FmcSdramBank::BOTH_BANKS => {
                    fmc.sdcr2.write(
                        |w| {
                            w.nb()
                                .variant(config.nb)
                                .cas()
                                .variant(config.cas_latency)
                                .mwid()
                                .variant(config.memory_width)
                                .nr()
                                .variant(config.num_rows)
                                .nc()
                                .variant(config.num_columns)
                                .wp()
                                .disabled()
                        }, // disable write protection
                    );
                }
                FmcSdramBank::BANK1 => {}
            }

            // Setup timings. trp and trc have to be set in sdtr1
            match config.bank {
                FmcSdramBank::BANK1 | FmcSdramBank::BOTH_BANKS => {
                    fmc.sdtr1.write(|w| {
                        w.trcd()
                            .bits(timing.rcd)
                            .trp()
                            .bits(timing.rp)
                            .twr()
                            .bits(timing.wr)
                            .trc()
                            .bits(timing.rc)
                            .tras()
                            .bits(timing.ras)
                            .txsr()
                            .bits(timing.xsr)
                            .tmrd()
                            .bits(timing.mrd)
                    });
                }
                FmcSdramBank::BANK2 => {
                    fmc.sdtr1
                        .modify(|_, w| w.trp().bits(timing.rp).trc().bits(timing.rc));
                }
            }
            match config.bank {
                FmcSdramBank::BANK2 | FmcSdramBank::BOTH_BANKS => {
                    fmc.sdtr2.write(|w| {
                        w.trcd()
                            .bits(timing.rcd)
                            .twr()
                            .bits(timing.wr)
                            .tras()
                            .bits(timing.ras)
                            .txsr()
                            .bits(timing.xsr)
                            .tmrd()
                            .bits(timing.mrd)
                    });
                }
                FmcSdramBank::BANK1 => {}
            }

            let _sdcr1 = fmc.sdcr1.read().bits();
            let _sdcr2 = fmc.sdcr2.read().bits();
            let _sdtr1 = fmc.sdtr1.read().bits();
            let _sdtr2 = fmc.sdtr2.read().bits();

            // Enable sdram clocks
            sdram.sdram_command(fmc, config.bank, MODE_AW::CLOCKCONFIGURATIONENABLE, 0, 0);
            delay.delay_us(500); // sleep >>100us (sdram dependent)
            fmc.busy_wait();

            // Issue "Precharge all" command
            sdram.sdram_command(fmc, config.bank, MODE_AW::PALL, 0, 0);
            fmc.busy_wait();

            // Set number of autorefresh command that should be issued to 8 (sdram dependant)
            sdram.sdram_command(fmc, config.bank, MODE_AW::AUTOREFRESHCOMMAND, 8, 0);
            fmc.busy_wait();

            // Load the mode register
            sdram.sdram_command(
                fmc,
                config.bank,
                MODE_AW::LOADMODEREGISTER,
                0,
                config.burst_length as u16
                    | fmc_sdram_mode_cmd::BURST_TYPE_SEQUENTIAL
                    | match config.cas_latency {
                        CAS_A::CLOCKS1 => fmc_sdram_mode_cmd::CAS_LATENCY_1,
                        CAS_A::CLOCKS2 => fmc_sdram_mode_cmd::CAS_LATENCY_2,
                        CAS_A::CLOCKS3 => fmc_sdram_mode_cmd::CAS_LATENCY_3,
                    }
                    | fmc_sdram_mode_cmd::OPERATING_MODE_STANDARD
                    | if config.write_burst {
                        fmc_sdram_mode_cmd::WRITEBURST_MODE_PROGRAMMED
                    } else {
                        fmc_sdram_mode_cmd::WRITEBURST_MODE_SINGLE
                    },
            );
            fmc.busy_wait();

            // Set the refresh counter
            fmc.sdrtr.write(|w| w.count().bits(timing.refresh_counter));

            // doing something completely unrelated :)
            //            fmc.bcr1.modify(|_,w| w.muxen().enabled()); // Why?
            //            fmc.bcr1.write(|w| unsafe { w.bits(BCR::reset_value()) }); // hö?
        });

        sdram
    }

    fn sdram_command(
        &self,
        fmc: &FMC,
        bank: FmcSdramBank,
        cmd: MODE_AW,
        autorefresh: u8,
        modereg: u16,
    ) {
        cortex_m::interrupt::free(|_| {
            let sdcmr_init = mem::MaybeUninit::<SDCMR>::zeroed();
            unsafe {
                sdcmr_init
                    .as_ptr()
                    .as_ref()
                    .unwrap()
                    .write(|w| w.bits(SDCMR::reset_value()))
            }
            let sdcmr_tmp = unsafe { sdcmr_init.assume_init() };

            match bank {
                FmcSdramBank::BANK1 => sdcmr_tmp.modify(|_, w| w.ctb1().issued()),
                FmcSdramBank::BANK2 => sdcmr_tmp.modify(|_, w| w.ctb2().issued()),
                FmcSdramBank::BOTH_BANKS => {
                    sdcmr_tmp.modify(|_, w| w.ctb1().issued().ctb2().issued())
                }
            };

            sdcmr_tmp.modify(|_, w| {
                w.nrfs()
                    .bits(autorefresh)
                    .mrd()
                    .bits(modereg)
                    .mode()
                    .variant(cmd)
            });

            // Wait for the next chance to talk to the controller
            fmc.busy_wait();

            /* Send the next command */
            let sdcmr_tmp = sdcmr_tmp.read().bits();
            fmc.sdcmr.write(|w| unsafe { w.bits(sdcmr_tmp) });
        });

        //        self
    }
}

// TODO How could this be implemented? With features? Each possible combination seems really bad (check the cube-parse output, there is more than visible here!). Can the user define Pins<Fmc> somehow?

// more or less autogenerated stuff
pub trait Pins<Fmc> {}

/// 8/16/32bit SDRam Bank0/1
impl<
        Fmc,
        SDCLK,
        //    SDCKE0, SDNE0,
        SDCKE1,
        SDNE1,
        BA1,
        BA0,
        SDNRAS,
        SDNCAS,
        SDNWE,
        NBL0,
        NBL1,
        //    NBL2,NBL3,
        A0,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11, //A12,
        D0,
        D1,
        D2,
        D3,
        D4,
        D5,
        D6,
        D7,
        D8,
        D9,
        D10,
        D11,
        D12,
        D13,
        D14,
        D15,
        //    D16,D17,D18,D19,D20,D21,D22,D23,D24,D25,D26,D27,D28,D29,D30,D31
    > Pins<Fmc>
    for (
        SDCLK,
        //    SDCKE0, SDNE0,
        SDCKE1,
        SDNE1,
        BA0,
        BA1,
        SDNRAS,
        SDNCAS,
        SDNWE,
        NBL0,
        NBL1,
        //    NBL2,NBL3,
        A0,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11, //A12,
        D0,
        D1,
        D2,
        D3,
        D4,
        D5,
        D6,
        D7,
        D8,
        D9,
        D10,
        D11,
        D12,
        D13,
        D14,
        D15,
        //    D16,D17,D18,D19,D20,D21,D22,D23,D24,D25,D26,D27,D28,D29,D30,D31
    )
where
    SDCLK: PinSdclk<Fmc>,
    //    SDCKE0: PinSdcke0<Fmc>, // bank0 clock enable
    //    SDNE0: PinSdne0<Fmc>, // bank0 not enable
    SDCKE1: PinSdcke1<Fmc>, // bank1 clock enable
    SDNE1: PinSdne1<Fmc>,   // bank1 not enable

    BA0: PinBa0<Fmc>,
    BA1: PinBa1<Fmc>,

    SDNRAS: PinSdnras<Fmc>,
    SDNCAS: PinSdncas<Fmc>,
    SDNWE: PinSdnwe<Fmc>,

    NBL0: PinNbl0<Fmc>,
    NBL1: PinNbl1<Fmc>,
    //    NBL2: PinNbl2<Fmc>, // optional, not needed?
    //    NBL3: PinNbl3<Fmc>, // optional, not needed?
    A0: PinA0<Fmc>,
    A1: PinA1<Fmc>,
    A2: PinA2<Fmc>,
    A3: PinA3<Fmc>,
    A4: PinA4<Fmc>,
    A5: PinA5<Fmc>,
    A6: PinA6<Fmc>,
    A7: PinA7<Fmc>,
    A8: PinA8<Fmc>,
    A9: PinA9<Fmc>,
    A10: PinA10<Fmc>,
    A11: PinA11<Fmc>, // only for 12/13 address bits
    //    A12: PinA12<Fmc>, // only for 13 address bits
    D0: PinD0<Fmc>,
    D1: PinD1<Fmc>,
    D2: PinD2<Fmc>,
    D3: PinD3<Fmc>,
    D4: PinD4<Fmc>,
    D5: PinD5<Fmc>,
    D6: PinD6<Fmc>,
    D7: PinD7<Fmc>,

    D8: PinD8<Fmc>,   // only for 16/32bit memory width
    D9: PinD9<Fmc>,   // only for 16/32bit memory width
    D10: PinD10<Fmc>, // only for 16/32bit memory width
    D11: PinD11<Fmc>, // only for 16/32bit memory width
    D12: PinD12<Fmc>, // only for 16/32bit memory width
    D13: PinD13<Fmc>, // only for 16/32bit memory width
    D14: PinD14<Fmc>, // only for 16/32bit memory width
    D15: PinD15<Fmc>, // only for 16/32bit memory width

                      //    D16: PinD16<Fmc>, // only for 32bit memory width
                      //    D17: PinD17<Fmc>, // only for 32bit memory width
                      //    D18: PinD18<Fmc>, // only for 32bit memory width
                      //    D19: PinD19<Fmc>, // only for 32bit memory width
                      //    D20: PinD20<Fmc>, // only for 32bit memory width
                      //    D21: PinD21<Fmc>, // only for 32bit memory width
                      //    D22: PinD22<Fmc>, // only for 32bit memory width
                      //    D23: PinD23<Fmc>, // only for 32bit memory width
                      //    D24: PinD24<Fmc>, // only for 32bit memory width
                      //    D25: PinD25<Fmc>, // only for 32bit memory width
                      //    D26: PinD26<Fmc>, // only for 32bit memory width
                      //    D27: PinD27<Fmc>, // only for 32bit memory width
                      //    D28: PinD28<Fmc>, // only for 32bit memory width
                      //    D29: PinD29<Fmc>, // only for 32bit memory width
                      //    D30: PinD30<Fmc>, // only for 32bit memory width
                      //    D31: PinD31<Fmc>, // only for 32bit memory width
{
}
