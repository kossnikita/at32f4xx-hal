use super::*;
use crate::gpio::{self, NoPin, PushPull};

#[cfg(feature = "tmr1")]
pub mod tmr1 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA8<1>,

            #[cfg(feature = "f403a-peripheral")]
            PE9<1>,

            #[cfg(feature = "f415-peripheral")]
            PC6<1>,
        ],

        <Ch1N> default: PushPull for [
            PA7<1>,

            PB13<1>,
        ],

        <Ch2> default: PushPull for [
            PA9<1>,

        ],

        <Ch2N> default: PushPull for [
            PB0<1>,

            PB14<1>,

        ],

        <Ch3> default: PushPull for [
            PA10<1>,

        ],

        <Ch3N> default: PushPull for [
            PB1<1>,

            PB15<1>,

        ],

        <Ch4> default: PushPull for [
            PA11<1>,

        ],
    }

    pin! {
        <Bkin, PushPull> for [
            PA6<1>,

            PB12<1>,
        ],

        <Ext, PushPull> for [
            PA12<1>,
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
