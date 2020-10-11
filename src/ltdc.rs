//! LTDC interface (lcd/tft-display controller 8/16/32-bit parallel display interface)

use core::mem;
use cortex_m::asm;
use embedded_hal::blocking::delay::DelayUs;
use stm32f4::ResetValue;

use crate::pin_defs::*;
use crate::rcc::Clocks;
use crate::stm32;
//use crate::stm32::fmc::sdcmr::MODE_AW;
//use crate::stm32::fmc::sdcr::{CAS_A, MWID_A, NB_A, NC_A, NR_A, RBURST_A, RPIPE_A, SDCLK_A};
//use crate::stm32::fmc::SDCMR;
use crate::stm32::LTDC;

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))] mod pins {
    use crate::pin_defs::*;
    /// Ltdc
    pub trait Pins<Ltdc> {}
    impl<Ltdc, 
        B0, B1, B2, B3, B4, B5, B6, B7, CLK, DE, G0, G1, 
        G2, G3, G4, G5, G6, G7, HSYNC, R0, R1, R2, R3, R4, 
        R5, R6, R7, VSYNC>
    Pins<Ltdc>
    for (
        B0, B1, B2, B3, B4, B5, B6, B7, CLK, DE, G0, G1, 
        G2, G3, G4, G5, G6, G7, HSYNC, R0, R1, R2, R3, R4, 
        R5, R6, R7, VSYNC
    )
    where
        B0: PinB0<Ltdc>,
        B1: PinB1<Ltdc>,
        B2: PinB2<Ltdc>,
        B3: PinB3<Ltdc>,
        B4: PinB4<Ltdc>,
        B5: PinB5<Ltdc>,
        B6: PinB6<Ltdc>,
        B7: PinB7<Ltdc>,
        CLK: PinClk<Ltdc>,
        DE: PinDe<Ltdc>,
        G0: PinG0<Ltdc>,
        G1: PinG1<Ltdc>,
        G2: PinG2<Ltdc>,
        G3: PinG3<Ltdc>,
        G4: PinG4<Ltdc>,
        G5: PinG5<Ltdc>,
        G6: PinG6<Ltdc>,
        G7: PinG7<Ltdc>,
        HSYNC: PinHsync<Ltdc>,
        R0: PinR0<Ltdc>,
        R1: PinR1<Ltdc>,
        R2: PinR2<Ltdc>,
        R3: PinR3<Ltdc>,
        R4: PinR4<Ltdc>,
        R5: PinR5<Ltdc>,
        R6: PinR6<Ltdc>,
        R7: PinR7<Ltdc>,
        VSYNC: PinVsync<Ltdc>
    {}

}
