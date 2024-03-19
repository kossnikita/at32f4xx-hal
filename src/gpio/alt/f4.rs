use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

#[cfg(feature = "tmr1")]
pub mod tmr1 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA8<2>,
        ],

        <Ch1N> default: PushPull for [
            PA7<2>,

            PB13<2>,

        ],

        <Ch2> default: PushPull for [
            PA9<2>,
        ],

        <Ch2N> default: PushPull for [
            PB0<2>,

            PB14<2>,
        ],

        <Ch3> default: PushPull for [
            PA10<2>,
        ],

        <Ch3N> default: PushPull for [
            PB1<2>,

            PB15<2>,
        ],

        <Ch4> default: PushPull for [
            PA11<2>,

        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<2>,

            PB12<2>,
        ],

        <Ext, PushPull> for [
            PA0<5>,

            PA12<2>,
        ],
    }

    use crate::pac::TMR1 as TMR;

    impl TmrCPin<0> for TMR {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TmrCPin<1> for TMR {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TmrCPin<2> for TMR {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TmrCPin<3> for TMR {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TmrNCPin<0> for TMR {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TmrNCPin<1> for TMR {
        type ChN<Otype> = Ch2N<Otype>;
    }
    impl TmrNCPin<2> for TMR {
        type ChN<Otype> = Ch3N<Otype>;
    }
    impl TmrBkin for TMR {
        type Bkin = Bkin;
    }
    impl TmrExt for TMR {
        type Ext = Ext;
    }
}

#[cfg(feature = "tmr2")]
pub mod tmr2 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA0<1>,

            PA15<1>,

        ],

        <Ch2> default: PushPull for [
            PA1<1>,

            PB3<1>,

        ],

        <Ch3> default: PushPull for [
            PA2<1>,

            PB10<1>,
        ],

        <Ch4> default: PushPull for [
            PA3<1>,

            PB11<1>,
        ],
    }

    pin! {
        <Ext, PushPull> for [
            PA0<1>,

            PA15<1>,

        ],
    }

    use crate::pac::TMR2 as TMR;

    impl TmrCPin<0> for TMR {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TmrCPin<1> for TMR {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TmrCPin<2> for TMR {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TmrCPin<3> for TMR {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TmrExt for TMR {
        type Ext = Ext;
    }
}

#[cfg(feature = "tmr3")]
pub mod tmr3 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA6<1>,

            PB4<1>,

            PC6<1>,
        ],

        <Ch2> default: PushPull for [
            PA7<1>,

            PB5<1>,

            PC7<1>,
        ],

        <Ch3> default: PushPull for [
            PB0<1>,

            PC8<1>,
        ],

        <Ch4> default: PushPull for [
            PB1<1>,

            PC9<1>,
        ],
    }

    pin! {
        <Ext, PushPull> for [
            PD2<1>,
        ],
    }

    use crate::pac::TMR3 as TMR;

    impl TmrCPin<0> for TMR {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TmrCPin<1> for TMR {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TmrCPin<2> for TMR {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TmrCPin<3> for TMR {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TmrExt for TMR {
        type Ext = Ext;
    }
}

#[cfg(feature = "tmr4")]
pub mod tmr4 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PB6<1>,
        ],

        <Ch2> default: PushPull for [
            PB7<1>,
        ],

        <Ch3> default: PushPull for [
            PB8<1>,
        ],

        <Ch4> default: PushPull for [
            PB9<1>,
        ],
    }

    use crate::pac::TMR4 as TMR;

    impl TmrCPin<0> for TMR {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TmrCPin<1> for TMR {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TmrCPin<2> for TMR {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TmrCPin<3> for TMR {
        type Ch<Otype> = Ch4<Otype>;
    }
}

#[cfg(feature = "tmr5")]
pub mod tmr5 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA0<1>,

            PF4<1>,
        ],

        <Ch2> default: PushPull for [
            PA1<1>,

            PF5<1>,
        ],

        <Ch3> default: PushPull for [
            PA2<1>,
        ],

        <Ch4> default: PushPull for [
            PA3<1>,

        ],
    }

    use crate::pac::TMR5 as TMR;

    impl TmrCPin<0> for TMR {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TmrCPin<1> for TMR {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TmrCPin<2> for TMR {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TmrCPin<3> for TMR {
        type Ch<Otype> = Ch4<Otype>;
    }
}

#[cfg(feature = "tmr9")]
pub mod tmr9 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA2<1>,

            PB14<1>,
        ],

        <Ch2> default: PushPull for [
            PA3<1>,

            PB15<1>,
        ],

    }

    use crate::pac::TMR9 as TMR;

    impl TmrCPin<0> for TMR {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TmrCPin<1> for TMR {
        type Ch<Otype> = Ch2<Otype>;
    }
}

pub mod usart1 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA8<1>,
        ],

        <Cts, PushPull> for [
            PA11<1>,
        ],

        <Rts, PushPull> for [
            PA12<1>,
        ],
    }

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            PA10<1>,

            PB7<1>,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PA9<1>,

            PB6<1>,
        ],
    }

    use crate::pac::USART1 as USART;
    impl SerialAsync for USART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialSync for USART {
        type Ck = Ck;
    }
    impl SerialRs232 for USART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

pub mod usart2 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PA4<1>,
        ],

        <Cts, PushPull> for [
            PA0<1>,
        ],

        <Rts, PushPull> for [
            PA1<1>,
        ],
    }

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            PA3<1>,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PA2<1>,
        ],
    }

    use crate::pac::USART2 as USART;
    impl SerialAsync for USART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialSync for USART {
        type Ck = Ck;
    }
    impl SerialRs232 for USART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

#[cfg(feature = "usart3")]
pub mod usart3 {
    use super::*;

    pin! {
        <Ck, PushPull> for [
            PB12<1>,
        ],

        <Cts, PushPull> for [
            PB13<1>,
        ],

        <Rts, PushPull> for [
            PB14<1>,
        ],
    }

    pin! {
        <Rx> default: PushPull for no:NoPin, [
            PB11<1>,
        ],

        <Tx> default: PushPull for no:NoPin, [
            PB10<1>,
        ],
    }

    use crate::pac::USART3 as USART;
    impl SerialAsync for USART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
    impl SerialSync for USART {
        type Ck = Ck;
    }
    impl SerialRs232 for USART {
        type Cts = Cts;
        type Rts = Rts;
    }
}

#[cfg(feature = "uart4")]
pub mod uart4 {
    use super::*;
    pin! {
        <Rx> default: PushPull for no:NoPin, [
            PC11<0>,
            PF5<1>,
        ],
        <Tx> default: PushPull for no:NoPin, [
            PC10<0>,
            PF4<1>,
        ],
    }

    use crate::pac::UART4 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
}

#[cfg(feature = "uart5")]
pub mod uart5 {
    use super::*;
    pin! {
        <Rx> default: PushPull for no:NoPin, [
            PD2<0>,
        ],
        <Tx> default: PushPull for no:NoPin, [
            PC12<0>,
        ],
    }

    use crate::pac::UART5 as UART;
    impl SerialAsync for UART {
        type Rx<Otype> = Rx<Otype>;
        type Tx<Otype> = Tx<Otype>;
    }
}

pub mod i2c1 {
    use super::*;
    use crate::pac::I2C1 as I2C;

    pin! {
        <Scl, OpenDrain> for [
            PA9<4>,

            PB6<2>,

            PB8<2>,

            PF1<2>,
        ],

        <Sda, OpenDrain> for [
            PA10<4>,

            PB7<2>,

            PB9<2>,

            PF0<2>,
        ],

        <Smba, OpenDrain> for [
            PA11<4>,

            PB5<3>,
        ],
    }

    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

pub mod i2c2 {
    use super::*;
    use crate::pac::I2C2 as I2C;

    pin! {
        <Scl, OpenDrain> for [
            PA0<4>,

            PA8<7>,

            PA11<5>,

            PB10<1>,

            PB13<5>,

            PF6<0>,
        ],

        <Sda, OpenDrain> for [
            PA1<4>,

            PA12<5>,

            PB4<7>,

            PB11<1>,

            PB14<5>,

            PF7<0>,
        ],

        <Smba, OpenDrain> for [
            PA9<7>,

            PB12<7>,
        ],
    }

    impl I2cCommon for I2C {
        type Scl = Scl;
        type Sda = Sda;
        type Smba = Smba;
    }
}

