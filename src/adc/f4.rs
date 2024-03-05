use super::*;

macro_rules! adc_pins {
    ($($pin:ty => ($adc:ident, $chan:expr)),+ $(,)*) => {
        $(
            impl Channel<pac::$adc> for $pin {
                type ID = u8;
                fn channel() -> u8 { $chan }
            }
        )+
    };
}

adc_pins!(
    gpio::PA0<Analog> => (ADC1, 0),
    gpio::PA1<Analog> => (ADC1, 1),
    gpio::PA2<Analog> => (ADC1, 2),
    gpio::PA3<Analog> => (ADC1, 3),
    gpio::PA4<Analog> => (ADC1, 4),
    gpio::PA5<Analog> => (ADC1, 5),
    gpio::PA6<Analog> => (ADC1, 6),
    gpio::PA7<Analog> => (ADC1, 7),
    gpio::PB0<Analog> => (ADC1, 8),
    gpio::PB1<Analog> => (ADC1, 9),
    gpio::PB2<Analog> => (ADC1, 10),
    gpio::PB12<Analog> => (ADC1, 11),
    gpio::PB13<Analog> => (ADC1, 12),
    gpio::PB14<Analog> => (ADC1, 13),
    gpio::PB15<Analog> => (ADC1, 14),
    Temperature => (ADC1, 16),
    Vref => (ADC1, 17),
);
