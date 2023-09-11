use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

pub mod tmr1 {
    use super::*;

    pin! {
        <Ch1> default: PushPull for [
            PA8<1>,

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

        <Etr, PushPull> for [
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
    impl TmrEtr for TMR {
        type Etr = Etr;
    }
}
