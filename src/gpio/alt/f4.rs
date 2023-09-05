use super::*;
use crate::gpio::{self, NoPin, OpenDrain, PushPull};

pub mod tim1 {
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

    use crate::pac::TIM1 as TIM;

    impl TimCPin<0> for TIM {
        type Ch<Otype> = Ch1<Otype>;
    }
    impl TimCPin<1> for TIM {
        type Ch<Otype> = Ch2<Otype>;
    }
    impl TimCPin<2> for TIM {
        type Ch<Otype> = Ch3<Otype>;
    }
    impl TimCPin<3> for TIM {
        type Ch<Otype> = Ch4<Otype>;
    }
    impl TimNCPin<0> for TIM {
        type ChN<Otype> = Ch1N<Otype>;
    }
    impl TimNCPin<1> for TIM {
        type ChN<Otype> = Ch2N<Otype>;
    }
    impl TimNCPin<2> for TIM {
        type ChN<Otype> = Ch3N<Otype>;
    }
    impl TimBkin for TIM {
        type Bkin = Bkin;
    }
    impl TimEtr for TIM {
        type Etr = Etr;
    }
}