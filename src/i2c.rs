use core::ops::Deref;
use cortex_m::interrupt::free;

use crate::stm32::i2c1;

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::I2C3;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::stm32::{I2C1, I2C2, RCC};

use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioa::PA8;

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiob::PB11;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
use crate::gpio::gpiob::PB3;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
use crate::gpio::gpiob::PB4;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiob::{PB10, PB6, PB7, PB8, PB9};

#[cfg(any(feature = "stm32f446"))]
use crate::gpio::gpioc::PC12;
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioc::PC9;

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpiof::{PF0, PF1};

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
use crate::gpio::gpioh::{PH4, PH5, PH7, PH8};

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
use crate::gpio::AF9;
use crate::gpio::{AlternateOD, AF4};

use crate::rcc::Clocks;
use crate::time::{Hertz, KiloHertz, U32Ext};

/// I2C abstraction
pub struct I2c<I2C, PINS> {
    i2c: I2C,
    pins: PINS,
}

pub trait Pins<I2c> {}
pub trait PinScl<I2c> {}
pub trait PinSda<I2c> {}

impl<I2c, SCL, SDA> Pins<I2c> for (SCL, SDA)
where
    SCL: PinScl<I2c>,
    SDA: PinSda<I2c>,
{
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinScl<I2C1> for PB6<AlternateOD<AF4>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinSda<I2C1> for PB7<AlternateOD<AF4>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinScl<I2C1> for PB8<AlternateOD<AF4>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinSda<I2C1> for PB9<AlternateOD<AF4>> {}

#[cfg(any(feature = "stm32f446"))]
impl PinSda<I2C2> for PB3<AlternateOD<AF4>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinSda<I2C2> for PB3<AlternateOD<AF9>> {}
#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinSda<I2C2> for PB9<AlternateOD<AF9>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinScl<I2C2> for PB10<AlternateOD<AF4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinSda<I2C2> for PB11<AlternateOD<AF4>> {}
#[cfg(any(feature = "stm32f446"))]
impl PinSda<I2C2> for PC12<AlternateOD<AF4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinScl<I2C2> for PF1<AlternateOD<AF4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinSda<I2C2> for PF0<AlternateOD<AF4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinScl<I2C2> for PH4<AlternateOD<AF4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinSda<I2C2> for PH5<AlternateOD<AF4>> {}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinScl<I2C3> for PA8<AlternateOD<AF4>> {}
#[cfg(any(feature = "stm32f446"))]
impl PinSda<I2C3> for PB4<AlternateOD<AF4>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinSda<I2C3> for PB4<AlternateOD<AF9>> {}
#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
impl PinSda<I2C3> for PB8<AlternateOD<AF9>> {}
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinSda<I2C3> for PC9<AlternateOD<AF4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinScl<I2C3> for PH7<AlternateOD<AF4>> {}
#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl PinSda<I2C3> for PH8<AlternateOD<AF4>> {}

#[derive(Debug)]
pub enum Error {
    OVERRUN,
    NACK,
    TIMEOUT
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl<PINS> I2c<I2C1, PINS> {
    pub fn i2c1(i2c: I2C1, pins: PINS, speed: KiloHertz, clocks: Clocks) -> Self
    where
        PINS: Pins<I2C1>,
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for I2C1
        rcc.apb1enr.modify(|_, w| w.i2c1en().set_bit());

        // Reset I2C1
        rcc.apb1rstr.modify(|_, w| w.i2c1rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.i2c1rst().clear_bit());

        let i2c = I2c { i2c, pins };
        i2c.i2c_init(speed, clocks.pclk1());
        i2c
    }
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl<PINS> I2c<I2C2, PINS> {
    pub fn i2c2(i2c: I2C2, pins: PINS, speed: KiloHertz, clocks: Clocks) -> Self
    where
        PINS: Pins<I2C2>,
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for I2C2
        rcc.apb1enr.modify(|_, w| w.i2c2en().set_bit());

        // Reset I2C2
        rcc.apb1rstr.modify(|_, w| w.i2c2rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.i2c2rst().clear_bit());

        let i2c = I2c { i2c, pins };
        i2c.i2c_init(speed, clocks.pclk1());
        i2c
    }
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
impl<PINS> I2c<I2C3, PINS> {
    pub fn i2c3(i2c: I2C3, pins: PINS, speed: KiloHertz, clocks: Clocks) -> Self
    where
        PINS: Pins<I2C3>,
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for I2C3
        rcc.apb1enr.modify(|_, w| w.i2c3en().set_bit());

        // Reset I2C3
        rcc.apb1rstr.modify(|_, w| w.i2c3rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.i2c3rst().clear_bit());

        let i2c = I2c { i2c, pins };
        i2c.i2c_init(speed, clocks.pclk1());
        i2c
    }
}

impl<I2C, PINS> I2c<I2C, PINS>
where
    I2C: Deref<Target = i2c1::RegisterBlock>,
{
    fn i2c_init(&self, speed: KiloHertz, pclk: Hertz) {
        let speed: Hertz = speed.into();

        // Software reset
        self.i2c.cr1.modify(|_, w| w.swrst().set_bit());
        { let mut delay = 10u32; while delay>0 {delay-=1;} }
        self.i2c.cr1.modify(|_, w| w.swrst().clear_bit());

        // Make sure the I2C unit is disabled so we can configure it
        self.i2c.cr1.modify(|_, w| w.pe().clear_bit());

        // Calculate settings for I2C speed modes
        let clock = pclk.0;
        let freq = clock / 1_000_000;
        assert!(freq >= 2 && freq <= 50);

        // Configure bus frequency into I2C peripheral
        self.i2c.cr2.modify(|_,w| unsafe { w.freq().bits(freq as u8) });

        let trise = if speed <= 100.khz().into() {
            freq + 1
        } else {
            (freq * 300) / 1000 + 1
        };

        // Configure correct rise times
        self.i2c.trise.write(|w| w.trise().bits(trise as u8));

        // I2C clock control calculation
        if speed <= 100.khz().into() {
            let ccr = {
                let ccr = clock / (speed.0 * 2);
                if ccr < 4 {
                    4
                } else {
                    ccr
                }
            };

            // Set clock to standard mode with appropriate parameters for selected speed
            self.i2c.ccr.modify(|_,w| unsafe {
                w.f_s().clear_bit()
                 .duty().clear_bit()
                 .ccr().bits(ccr as u16)
            });
        } else {
            const DUTYCYCLE: u8 = 0;
            if DUTYCYCLE == 0 {
                let ccr = clock / (speed.0 * 3);
                let ccr = if ccr < 1 { 1 } else { ccr };

                // Set clock to fast mode with appropriate parameters for selected speed (2:1 duty cycle)
                self.i2c.ccr.modify(|_,w| unsafe {
                    w.f_s().set_bit()
                     .duty().clear_bit()
                     .ccr().bits(ccr as u16)
                });
            } else {
                let ccr = clock / (speed.0 * 25);
                let ccr = if ccr < 1 { 1 } else { ccr };

                // Set clock to fast mode with appropriate parameters for selected speed (16:9 duty cycle)
                self.i2c.ccr.modify(|_,w| unsafe {
                    w.f_s().set_bit()
                     .duty().set_bit()
                     .ccr().bits(ccr as u16)
                });
            }
        }

        // Enable the I2C processing
        self.i2c.cr1.modify(|_, w| w.pe().set_bit());
    }

    pub fn release(self) -> (I2C, PINS) {
        (self.i2c, self.pins)
    }
}

trait I2cCommon {
    fn wait_while_busy(&self) -> Result<(), Error>;
    fn wait_until_safe(&self) -> Result<(), Error>;
    fn recover(&self);
    fn wait_for_sr1_field(&self, field: impl Fn(&i2c1::SR1) -> bool) -> Result<(), Error>;
}

impl<I2C, PINS> I2cCommon for I2c<I2C, PINS>
where
    I2C: Deref<Target = i2c1::RegisterBlock>,
{

    fn wait_while_busy(&self) -> Result<(), Error> {
        // Wait while busy
        while !self.i2c.sr2.read().busy().bit_is_clear() {
            // TODO timeout
        }
        // Clear NACK error
        if self.i2c.sr1.read().af().bit() {
            self.i2c.sr1.modify(|_, w| w.af().clear_bit());
        }

        self.wait_until_safe()?;
        Ok(())
    }

    fn wait_until_safe(&self) -> Result<(), Error> {
        // Clear errors
        {
            let sr1 = self.i2c.sr1.read();
            if sr1.timeout().bit() || sr1.pecerr().bit() || sr1.ovr().bit() || sr1.berr().bit() {
                self.i2c.sr1.modify(|_,w|
                    w.timeout().clear_bit()
                     .pecerr().clear_bit()
                     .ovr().clear_bit()
                     .berr().clear_bit()
                );
            }
        }
        // wait until i2c is free
        let mut timeout = 100_000u32;
        while {
            let cr1 = self.i2c.cr1.read();
            cr1.stop().bit() || cr1.start().bit() || cr1.pec().bit()
        } {
            if timeout==0 {
                self.i2c.cr1.read();
                self.recover();
                return Err(Error::TIMEOUT);
            } else {
                timeout-=1;
            }
        }

        Ok(())
    }

    fn recover(&self) {
        // I2C recovery
        if self.i2c.sr1.read().sb().bit() { self.i2c.dr.write(|w| unsafe { w.bits(0) }); }
        self.i2c.sr2.read();
        self.i2c.sr1.write(|w| unsafe { w.bits(0) });
        if self.i2c.sr2.read().msl().bit() {
            self.i2c.dr.write(|w| unsafe { w.bits(0) });
            self.i2c.dr.write(|w| unsafe { w.bits(0) });
            self.i2c.cr1.modify(|_, w| w.stop().set_bit());
            self.i2c.dr.write(|w| unsafe { w.bits(0) });
        } else {
            self.i2c.dr.read();
            self.i2c.dr.read();
            self.i2c.cr1.modify(|_, w| w.ack().clear_bit());
            self.i2c.cr1.modify(|_, w| w.stop().set_bit());
            self.i2c.dr.read();
        }
        self.i2c.dr.read();
    }

    fn wait_for_sr1_field(&self, field: impl Fn(&i2c1::SR1) -> bool) -> Result<(), Error> {
//    some day...
//    fn wait_for_sr1_field(&self, field: impl Fn(&stm32f4::generic::R<u32, stm32f4::generic::Reg<u32, stm32f4::stm32f429::i2c1::_SR1>>) -> bool) -> Result<(), Error> {
        // Wait until START condition was generated (or NACK)
        let mut timeout = 100_000u32;
        while {
            let sr1 = self.i2c.sr1.read();
            !(field(&self.i2c.sr1) || sr1.af().bit()) && timeout > 0
//            some day
//            !(field(&sr1) || sr1.af().bit()) && timeout > 0
        } {
            timeout-=1;
        }
        self.i2c.sr1.read();
        // handle nack
        if self.i2c.sr1.read().af().bit() {
            // clear ADDR flag by reading SR2
            self.i2c.sr2.read();
            // clear start condition
            if self.i2c.sr1.read().sb().bit() { self.i2c.dr.write(|w| unsafe { w.bits(0) }); }
            self.i2c.sr2.read();
            // send stop
            self.i2c.cr1.modify(|_,w| w.stop().set_bit());
            self.i2c.dr.write(|w| unsafe { w.bits(0) });
            return Err(Error::NACK);
        }
        // handle timeout
        if timeout==0 {
            self.i2c.sr2.read();
            self.recover();
            return Err(Error::TIMEOUT);
        }
        Ok(())
    }
}

impl<I2C, PINS> WriteRead for I2c<I2C, PINS>
where
    I2C: Deref<Target = i2c1::RegisterBlock>,
{
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.write(addr, bytes)?;
        self.read(addr, buffer)?;

        Ok(())
    }
}

impl<I2C, PINS> Write for I2c<I2C, PINS>
where
    I2C: Deref<Target = i2c1::RegisterBlock>,
{
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        // Wait until not busy (maybe move to after start?)
//        self.wait_while_busy()?;
//        self.i2c.sr1.modify(|_, w| w.af().clear_bit()); // clear NACK
        self.wait_until_safe()?;

        // Send a START condition
        self.i2c.cr1.modify(|_, w| w.start().set_bit());

        // Wait for start condition
        self.wait_for_sr1_field(|sr1| sr1.read().sb().bit())?;

        // Also wait until signalled we're master and everything is waiting for us
        while {
            let sr2 = self.i2c.sr2.read();
            sr2.msl().bit_is_clear() && sr2.busy().bit_is_clear()
        } {}

        // Set up current address, we're trying to talk to
        self.i2c
            .dr
            .write(|w| unsafe { w.bits(u32::from(addr) << 1) });

        // Wait until address was sent
        self.wait_for_sr1_field(|sr1| sr1.read().addr().bit())?;

        // Clear condition by reading SR2
        self.i2c.sr2.read();

        // Send bytes
        for c in bytes {
            // Wait until we're ready for sending
            self.wait_for_sr1_field(|sr1| sr1.read().tx_e().bit())?;

            // Push out a byte of data
            self.i2c.dr.write(|w| unsafe { w.bits(u32::from(*c)) });

            // Wait until byte is transferred
            self.wait_for_sr1_field(|sr1| sr1.read().btf().bit())?;
        }

        // Fallthrough is success
        Ok(())
    }
}

impl<I2C, PINS> Read for I2c<I2C, PINS>
where
    I2C: Deref<Target = i2c1::RegisterBlock>,
{
    type Error = Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        if let Some((_last, buffer)) = buffer.split_last_mut() {
            // Wait until not busy (maybe move to after start?)
//            self.wait_while_busy()?;
//            self.i2c.sr1.modify(|_, w| w.af().clear_bit()); // clear NACK
            self.wait_until_safe()?;

            // Send a START condition and set ACK bit
            self.i2c
                .cr1
                .modify(|_, w| w.start().set_bit().ack().set_bit());

            // Wait until START condition was generated
            self.wait_for_sr1_field(|sr1| sr1.read().sb().bit())?;

            // Also wait until signalled we're master and everything is waiting for us
            while {
                let sr2 = self.i2c.sr2.read();
                sr2.msl().bit_is_clear() && sr2.busy().bit_is_clear()
            } {}

            // Set up current address, we're trying to talk to
            self.i2c
                .dr
                .write(|w| unsafe { w.bits((u32::from(addr) << 1) + 1) });

            // Wait until address was sent
            self.wait_for_sr1_field(|sr1| sr1.read().addr().bit())?;

            // Initialize transfer
            // NOTE: The I2C input buffer has 2-bytes, which causes confusion
            let mut rest = buffer.len();
            match rest {
                1 => {
                    // Disable ack
                    self.i2c.cr1.modify(|_, w| w.ack().clear_bit());
                    // Disable all active IRQs around ADDR clearing and STOP programming because the EV6_3
                    // software sequence must complete before the current byte end of transfer
                    free(|_| {
                        // Clear address
                        self.i2c.sr1.read();
                        self.i2c.sr2.read();
                        // Prepare to send STOP after next byte
                        self.i2c.cr1.modify(|_, w| w.stop().set_bit());
                    });
                },
                2 => {
                    // This may cause trouble? has it to be set before start?
                    self.i2c.cr1.modify(|_, w| w.pos().set_bit());
                    // EV6_1: The acknowledge disable should be done just after EV6,
                    // that is after ADDR is cleared, so disable all active IRQs around ADDR clearing and
                    // ACK clearing
                    free(|_| {
                        // Clear address
                        self.i2c.sr1.read();
                        self.i2c.sr2.read();
                        // Prepare to send STOP after next byte
                        self.i2c.cr1.modify(|_, w| w.ack().clear_bit());
                    });
                },
                _ => {
                    // Enable ack
                    self.i2c.cr1.modify(|_, w| w.ack().set_bit());
                    // Clear address
                    self.i2c.sr1.read();
                    self.i2c.sr2.read();
                },
            };

            // Receive bytes into buffer
            for c in buffer {
                match rest {
                    1 => {
                        // Wait until receive buffer empty read
                        self.wait_for_sr1_field(|sr1| sr1.read().rx_ne().bit())?;
                        // Read next byte
                        *c = self.i2c.dr.read().bits() as u8;
                        rest -= 1;
                    },
                    2 => {
                        // Wait until byte transfer finished
                        self.wait_for_sr1_field(|sr1| sr1.read().btf().bit())?;
                        free(|_| {
                            // Prepare to send STOP after next byte
                            self.i2c.cr1.modify(|_, w| w.stop().set_bit());
                            // Read next byte
                            *c = self.i2c.dr.read().bits() as u8;
                            rest -= 1;
                        });
                        // Read next byte
                        *c = self.i2c.dr.read().bits() as u8;
                        rest -= 1;
                    },
                    3 => { // ???!
                        // Wait until byte transfer finished
                        self.wait_for_sr1_field(|sr1| sr1.read().btf().bit())?;
                        // Disable ack
                        self.i2c.cr1.modify(|_, w| w.ack().clear_bit());
                        // Read next byte
                        *c = self.i2c.dr.read().bits() as u8;
                        rest -= 1;
                        // goto case 2 (to finish after 1 byte.. hysterical coders, meh!)
                    },
                    _ => {
                        // Wait until byte transfer finished
                        self.wait_for_sr1_field(|sr1| sr1.read().btf().bit())?;
                        // Read next byte
                        *c = self.i2c.dr.read().bits() as u8;
                        rest -= 1;
                    },
                }
            }

            // Wait a bit
            self.wait_until_safe()?;

            // Clear some field
            self.i2c.cr1.modify(|_, w| w.pos().clear_bit());

            // Handle NACK
            if self.i2c.sr1.read().af().bit() {
                self.i2c.sr1.modify(|_,w| w.af().clear_bit());
                return Err(Error::NACK);
            }

            // Fallthrough is success
            Ok(())
        } else {
            Err(Error::OVERRUN)
        }
    }
}
