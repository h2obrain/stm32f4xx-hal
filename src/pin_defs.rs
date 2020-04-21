// Uses

use crate::gpio::Alternate;

macro_rules! dev_uses {
    ($($DEV:ident),+) => {
        $(
            use crate::stm32::$DEV;
        )+
    }
}
macro_rules! gpio_af_uses {
    ($($AF:ident),+) => {
        use crate::gpio::{$($AF)+};
    }
}
macro_rules! gpio_uses {
    ($($GPIO:ident => {
        $($PINS:ident),+
    }),+) => {
        $(
            use crate::gpio::$GPIO::{$($PINS),+};
        )+
    }
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
dev_uses! {
    FMC
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
gpio_af_uses! {
    AF12
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439"
))]
gpio_uses! {
    gpiof => {PF6,PF7,PF8,PF9,PF10},
    gpiog => {PG6,PG11}
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
gpio_uses! {
    gpiob => {PB5,PB6,PB7},
    gpioc => {PC0,PC2,PC3},
    gpiod => {PD0,PD1,PD3,PD4,PD5,PD6,PD7,PD8,PD9,PD10,PD11,PD12,PD13,PD14,PD15},
    gpioe => {PE0,PE1,PE2,PE3,PE4,PE5,PE6,PE7,PE8,PE9,PE10,PE11,PE12,PE13,PE14,PE15},
    gpiof => {PF0,PF1,PF2,PF3,PF4,PF5,PF11,PF12,PF13,PF14,PF15},
    gpiog => {PG0,PG1,PG2,PG3,PG4,PG5,PG7,PG8,PG9,PG10,PG12,PG13,PG14,PG15}
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
gpio_uses! {
    gpioh => {PH2,PH3,PH5,PH6,PH7,PH8,PH9,PH10,PH11,PH12,PH13,PH14,PH15},
    gpioi => {PI0,PI1,PI2,PI3,PI4,PI5,PI6,PI7,PI9,PI10}
}

#[cfg(any(feature = "stm32f446", feature = "stm32f469", feature = "stm32f479"))]
gpio_uses! {
    gpioa => {PA7},
    gpioc => {PC4,PC5}
}

// Traits

macro_rules! io_traits {
    ($($STEM:ident => {
        $($IO:ident),+
    }),+) => {
        $(
            $(
                pub trait $IO<$STEM> {}
            )+
        )+
    }
}
#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439"
))]
io_traits! {
   Fmc => {PinCd,PinInt2,PinIntr,PinNce2,PinNce41,PinNce42,PinNiord,PinNiowr,PinNreg}
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446"
))]
io_traits! {
   Fmc => {PinInt3,PinNce3}
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
io_traits! {
   Fmc => {PinA0,PinA1,PinA2,PinA3,PinA4,PinA5,PinA6,PinA7,PinA8,PinA9,PinA10,PinA11,PinA12,PinA13,PinA14,PinA15,PinA16,PinA17,PinA18,PinA19,PinA20,PinA21,PinA22,PinA23,PinA24,PinA25,PinBa0,PinBa1,PinClk,PinD0,PinD1,PinD2,PinD3,PinD4,PinD5,PinD6,PinD7,PinD8,PinD9,PinD10,PinD11,PinD12,PinD13,PinD14,PinD15,PinDa0,PinDa1,PinDa2,PinDa3,PinDa4,PinDa5,PinDa6,PinDa7,PinDa8,PinDa9,PinDa10,PinDa11,PinDa12,PinDa13,PinDa14,PinDa15,PinNbl0,PinNbl1,PinNe1,PinNe2,PinNe3,PinNe4,PinNl,PinNoe,PinNwait,PinNwe,PinSdcke0,PinSdcke1,PinSdclk,PinSdncas,PinSdne0,PinSdne1,PinSdnras,PinSdnwe}
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
io_traits! {
   Fmc => {PinAle,PinCle,PinD16,PinD17,PinD18,PinD19,PinD20,PinD21,PinD22,PinD23,PinD24,PinD25,PinD26,PinD27,PinD28,PinD29,PinD30,PinD31,PinNbl2,PinNbl3}
}

#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
io_traits! {
   Fmc => {PinInt,PinNce}
}

// Implementations

macro_rules! pins {
    ($($PIN:ident => {
        $($AF:ty: $TRAIT:ty),+
    }),+) => {
        $(
            $(
                impl $TRAIT for $PIN<Alternate<$AF>> {}
            )+
        )+
    }
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439"
))]
pins! {
    PD7  => {AF12: PinNce2<FMC>},
    PF6  => {AF12: PinNiord<FMC>},
    PF7  => {AF12: PinNreg<FMC>},
    PF8  => {AF12: PinNiowr<FMC>},
    PF9  => {AF12: PinCd<FMC>},
    PF10 => {AF12: PinIntr<FMC>},
    PG6  => {AF12: PinInt2<FMC>},
    PG10 => {AF12: PinNce41<FMC>},
    PG11 => {AF12: PinNce42<FMC>}
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446"
))]
pins! {
    PG7  => {AF12: PinInt3<FMC>},
    PG9  => {AF12: PinNce3<FMC>}
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    PB5  => {AF12: PinSdcke1<FMC>},
    PB6  => {AF12: PinSdne1<FMC>},
    PB7  => {AF12: PinNl<FMC>},
    PC0  => {AF12: PinSdnwe<FMC>},
    PC2  => {AF12: PinSdne0<FMC>},
    PC3  => {AF12: PinSdcke0<FMC>},
    PD0  => {AF12: PinD2<FMC>},
    PD0  => {AF12: PinDa2<FMC>},
    PD1  => {AF12: PinD3<FMC>},
    PD1  => {AF12: PinDa3<FMC>},
    PD3  => {AF12: PinClk<FMC>},
    PD4  => {AF12: PinNoe<FMC>},
    PD5  => {AF12: PinNwe<FMC>},
    PD6  => {AF12: PinNwait<FMC>},
    PD7  => {AF12: PinNe1<FMC>},
    PD8  => {AF12: PinD13<FMC>},
    PD8  => {AF12: PinDa13<FMC>},
    PD9  => {AF12: PinD14<FMC>},
    PD9  => {AF12: PinDa14<FMC>},
    PD10 => {AF12: PinD15<FMC>},
    PD10 => {AF12: PinDa15<FMC>},
    PD11 => {AF12: PinA16<FMC>},
    PD12 => {AF12: PinA17<FMC>},
    PD13 => {AF12: PinA18<FMC>},
    PD14 => {AF12: PinD0<FMC>},
    PD14 => {AF12: PinDa0<FMC>},
    PD15 => {AF12: PinD1<FMC>},
    PD15 => {AF12: PinDa1<FMC>},
    PE0  => {AF12: PinNbl0<FMC>},
    PE1  => {AF12: PinNbl1<FMC>},
    PE2  => {AF12: PinA23<FMC>},
    PE3  => {AF12: PinA19<FMC>},
    PE4  => {AF12: PinA20<FMC>},
    PE5  => {AF12: PinA21<FMC>},
    PE6  => {AF12: PinA22<FMC>},
    PE7  => {AF12: PinD4<FMC>},
    PE7  => {AF12: PinDa4<FMC>},
    PE8  => {AF12: PinD5<FMC>},
    PE8  => {AF12: PinDa5<FMC>},
    PE9  => {AF12: PinD6<FMC>},
    PE9  => {AF12: PinDa6<FMC>},
    PE10 => {AF12: PinD7<FMC>},
    PE10 => {AF12: PinDa7<FMC>},
    PE11 => {AF12: PinD8<FMC>},
    PE11 => {AF12: PinDa8<FMC>},
    PE12 => {AF12: PinD9<FMC>},
    PE12 => {AF12: PinDa9<FMC>},
    PE13 => {AF12: PinD10<FMC>},
    PE13 => {AF12: PinDa10<FMC>},
    PE14 => {AF12: PinD11<FMC>},
    PE14 => {AF12: PinDa11<FMC>},
    PE15 => {AF12: PinD12<FMC>},
    PE15 => {AF12: PinDa12<FMC>},
    PF0  => {AF12: PinA0<FMC>},
    PF1  => {AF12: PinA1<FMC>},
    PF2  => {AF12: PinA2<FMC>},
    PF3  => {AF12: PinA3<FMC>},
    PF4  => {AF12: PinA4<FMC>},
    PF5  => {AF12: PinA5<FMC>},
    PF11 => {AF12: PinSdnras<FMC>},
    PF12 => {AF12: PinA6<FMC>},
    PF13 => {AF12: PinA7<FMC>},
    PF14 => {AF12: PinA8<FMC>},
    PF15 => {AF12: PinA9<FMC>},
    PG0  => {AF12: PinA10<FMC>},
    PG1  => {AF12: PinA11<FMC>},
    PG2  => {AF12: PinA12<FMC>},
    PG3  => {AF12: PinA13<FMC>},
    PG4  => {AF12: PinA14<FMC>},
    PG4  => {AF12: PinBa0<FMC>},
    PG5  => {AF12: PinA15<FMC>},
    PG5  => {AF12: PinBa1<FMC>},
    PG8  => {AF12: PinSdclk<FMC>},
    PG9  => {AF12: PinNe2<FMC>},
    PG10 => {AF12: PinNe3<FMC>},
    PG12 => {AF12: PinNe4<FMC>},
    PG13 => {AF12: PinA24<FMC>},
    PG14 => {AF12: PinA25<FMC>},
    PG15 => {AF12: PinSdncas<FMC>}
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    PD11 => {AF12: PinCle<FMC>},
    PD12 => {AF12: PinAle<FMC>},
    PH2  => {AF12: PinSdcke0<FMC>},
    PH3  => {AF12: PinSdne0<FMC>},
    PH5  => {AF12: PinSdnwe<FMC>},
    PH6  => {AF12: PinSdne1<FMC>},
    PH7  => {AF12: PinSdcke1<FMC>},
    PH8  => {AF12: PinD16<FMC>},
    PH9  => {AF12: PinD17<FMC>},
    PH10 => {AF12: PinD18<FMC>},
    PH11 => {AF12: PinD19<FMC>},
    PH12 => {AF12: PinD20<FMC>},
    PH13 => {AF12: PinD21<FMC>},
    PH14 => {AF12: PinD22<FMC>},
    PH15 => {AF12: PinD23<FMC>},
    PI0  => {AF12: PinD24<FMC>},
    PI1  => {AF12: PinD25<FMC>},
    PI2  => {AF12: PinD26<FMC>},
    PI3  => {AF12: PinD27<FMC>},
    PI4  => {AF12: PinNbl2<FMC>},
    PI5  => {AF12: PinNbl3<FMC>},
    PI6  => {AF12: PinD28<FMC>},
    PI7  => {AF12: PinD29<FMC>},
    PI9  => {AF12: PinD30<FMC>},
    PI10 => {AF12: PinD31<FMC>}
}

#[cfg(any(feature = "stm32f446", feature = "stm32f469", feature = "stm32f479"))]
pins! {
    PA7  => {AF12: PinSdnwe<FMC>},
    PC4  => {AF12: PinSdne0<FMC>},
    PC5  => {AF12: PinSdcke0<FMC>}
}

#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
pins! {
    PG7  => {AF12: PinInt<FMC>},
    PG9  => {AF12: PinNce<FMC>}
}
