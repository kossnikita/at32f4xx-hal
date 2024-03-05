use crate::crm::{Enable, Reset};
use crate::{
    gpio::{self, Analog},
    pac,
    signature::{VrefCal, VDDA_CALIB},
};
use core::fmt;

mod f4;

/// Vref internal signal, used for calibration
pub struct Vref;

/// Vbat internal signal, used for monitoring the battery (if used)
pub struct Vbat;

/// Core temperature internal signal
pub struct Temperature;

/// Contains types related to ADC configuration
pub mod config {
    /// The place in the sequence a given channel should be captured
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
    #[repr(u8)]
    pub enum Sequence {
        /// 1
        One = 0,
        /// 2
        Two = 1,
        /// 3
        Three = 2,
        /// 4
        Four = 3,
        /// 5
        Five = 4,
        /// 6
        Six = 5,
        /// 7
        Seven = 6,
        /// 8
        Eight = 7,
        /// 9
        Nine = 8,
        /// 10
        Ten = 9,
        /// 11
        Eleven = 10,
        /// 12
        Twelve = 11,
        /// 13
        Thirteen = 12,
        /// 14
        Fourteen = 13,
        /// 15
        Fifteen = 14,
        /// 16
        Sixteen = 15,
    }

    impl From<Sequence> for u8 {
        fn from(s: Sequence) -> u8 {
            s as _
        }
    }

    impl From<u8> for Sequence {
        fn from(bits: u8) -> Self {
            match bits {
                0 => Sequence::One,
                1 => Sequence::Two,
                2 => Sequence::Three,
                3 => Sequence::Four,
                4 => Sequence::Five,
                5 => Sequence::Six,
                6 => Sequence::Seven,
                7 => Sequence::Eight,
                8 => Sequence::Nine,
                9 => Sequence::Ten,
                10 => Sequence::Eleven,
                11 => Sequence::Twelve,
                12 => Sequence::Thirteen,
                13 => Sequence::Fourteen,
                14 => Sequence::Fifteen,
                15 => Sequence::Sixteen,
                _ => unreachable!(),
            }
        }
    }

    /// The number of cycles to sample a given channel for
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum SampleTime {
        /// 3 cycles
        Cycles_3 = 0,
        /// 15 cycles
        Cycles_15 = 1,
        /// 28 cycles
        Cycles_28 = 2,
        /// 56 cycles
        Cycles_56 = 3,
        /// 84 cycles
        Cycles_84 = 4,
        /// 112 cycles
        Cycles_112 = 5,
        /// 144 cycles
        Cycles_144 = 6,
        /// 480 cycles
        Cycles_480 = 7,
    }

    impl From<u8> for SampleTime {
        fn from(f: u8) -> SampleTime {
            match f {
                0 => SampleTime::Cycles_3,
                1 => SampleTime::Cycles_15,
                2 => SampleTime::Cycles_28,
                3 => SampleTime::Cycles_56,
                4 => SampleTime::Cycles_84,
                5 => SampleTime::Cycles_112,
                6 => SampleTime::Cycles_144,
                7 => SampleTime::Cycles_480,
                _ => unimplemented!(),
            }
        }
    }

    impl From<SampleTime> for u8 {
        fn from(l: SampleTime) -> u8 {
            l as _
        }
    }

    #[cfg(any(
        feature = "at32a403a",
        feature = "at32f403",
        feature = "at32f403a",
        feature = "at32f407",
        feature = "at32f413",
        feature = "at32f415",
        feature = "at32f421",
        feature = "at32f425",
        feature = "at32wb415",
    ))]
    /// Clock config for the ADC
    /// Check the datasheet for the maximum speed the ADC supports
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum Clock {
        /// PCLK2 (APB2) divided by 2
        Pclk2_div_2 = 0,
        /// PCLK2 (APB2) divided by 4
        Pclk2_div_4 = 1,
        /// PCLK2 (APB2) divided by 6
        Pclk2_div_6 = 2,
        /// PCLK2 (APB2) divided by 8
        Pclk2_div_8 = 3,
        /// PCLK2 (APB2) divided by 2 (duplicate)
        Pclk2_div_2_duplicate = 4,
        /// PCLK2 (APB2) divided by 12
        Pclk2_div_12 = 5,
        /// PCLK2 (APB2) divided by 8 (duplicate)
        Pclk2_div_8_duplicate = 6,
        /// PCLK2 (APB2) divided by 16
        Pclk2_div_16 = 7,
    }

    #[cfg(any(
        feature = "at32f402",
        feature = "at32f405",
        feature = "at32f423",
        feature = "at32f435",
        feature = "at32f437",
    ))]
    /// Clock config for the ADC
    /// Check the datasheet for the maximum speed the ADC supports
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum Clock {
        /// HCLK divided by 2
        Hclk_div_2 = 0,
        /// HCLK divided by 4
        Hclk_div_4 = 1,
        /// HCLK divided by 6
        Hclk_div_6 = 2,
        /// HCLK divided by 8
        Hclk_div_8 = 3,
    }

    impl From<Clock> for u8 {
        fn from(c: Clock) -> u8 {
            c as _
        }
    }

    /// Resolution to sample at
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum Resolution {
        /// 12-bit
        Twelve = 0,
        /// 10-bit
        Ten = 1,
        /// 8-bit
        Eight = 2,
        /// 6-bit
        Six = 3,
    }
    impl From<Resolution> for u8 {
        fn from(r: Resolution) -> u8 {
            r as _
        }
    }

    /// Possible external triggers the ADC can listen to
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum ExternalTrigger {
        /// TIM1 compare channel 1
        Tim_1_cc_1 = 0b0000,
        /// TIM1 compare channel 2
        Tim_1_cc_2 = 0b0001,
        /// TIM1 compare channel 3
        Tim_1_cc_3 = 0b0010,
        /// TIM2 compare channel 2
        Tim_2_cc_2 = 0b0011,
        /// TIM2 compare channel 3
        Tim_2_cc_3 = 0b0100,
        /// TIM2 compare channel 4
        Tim_2_cc_4 = 0b0101,
        /// TIM2 trigger out
        Tim_2_trgo = 0b0110,
        /// TIM3 compare channel 1
        Tim_3_cc_1 = 0b0111,
        /// TIM3 trigger out
        Tim_3_trgo = 0b1000,
        /// TIM4 compare channel 4
        Tim_4_cc_4 = 0b1001,
        /// TIM5 compare channel 1
        Tim_5_cc_1 = 0b1010,
        /// TIM5 compare channel 2
        Tim_5_cc_2 = 0b1011,
        /// TIM5 compare channel 3
        Tim_5_cc_3 = 0b1100,
        /// External interrupt line 11
        Exti_11 = 0b1111,
    }
    impl From<ExternalTrigger> for u8 {
        fn from(et: ExternalTrigger) -> u8 {
            et as _
        }
    }

    /// Possible trigger modes
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum TriggerMode {
        /// Don't listen to external trigger
        Disabled = 0,
        /// Listen for rising edges of external trigger
        RisingEdge = 1,
    }
    impl From<TriggerMode> for bool {
        fn from(tm: TriggerMode) -> bool {
            tm != TriggerMode::Disabled
        }
    }

    /// Data register alignment
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Align {
        /// Right align output data
        Right,
        /// Left align output data
        Left,
    }
    impl From<Align> for bool {
        fn from(a: Align) -> bool {
            match a {
                Align::Right => false,
                Align::Left => true,
            }
        }
    }

    /// Sequence mode enable/disable
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum SequenceMode {
        /// Sequence mode disabled
        Disabled,
        /// Sequence mode enabled
        Enabled,
    }
    impl From<SequenceMode> for bool {
        fn from(s: SequenceMode) -> bool {
            match s {
                SequenceMode::Disabled => false,
                SequenceMode::Enabled => true,
            }
        }
    }

    /// Continuous mode enable/disable
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Continuous {
        /// Single mode, continuous disabled
        Single,
        /// Continuous mode enabled
        Continuous,
    }
    impl From<Continuous> for bool {
        fn from(c: Continuous) -> bool {
            match c {
                Continuous::Single => false,
                Continuous::Continuous => true,
            }
        }
    }

    /// DMA mode
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Dma {
        /// No DMA, disabled
        Disabled,
        /// Single DMA, DMA will be disabled after each conversion sequence
        Single,
        /// Continuous DMA, DMA will remain enabled after conversion
        Continuous,
    }

    /// End-of-conversion interrupt enabled/disabled
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Eoc {
        /// End-of-conversion interrupt disabled
        Disabled,
        /// End-of-conversion interrupt enabled per conversion
        Conversion,
        /// End-of-conversion interrupt enabled per sequence
        Sequence,
    }

    /// Configuration for the adc.
    /// There are some additional parameters on the adc peripheral that can be
    /// added here when needed but this covers several basic usecases.
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct AdcConfig {
        pub(crate) clock: Clock,
        pub(crate) resolution: Resolution,
        pub(crate) align: Align,
        pub(crate) sequence: SequenceMode,
        pub(crate) external_trigger: (TriggerMode, ExternalTrigger),
        pub(crate) continuous: Continuous,
        pub(crate) dma: Dma,
        pub(crate) end_of_conversion_interrupt: Eoc,
        pub(crate) default_sample_time: SampleTime,
        pub(crate) vdda: Option<u32>,
    }

    impl AdcConfig {
        /// change the clock field
        pub fn clock(mut self, clock: Clock) -> Self {
            self.clock = clock;
            self
        }
        /// change the resolution field
        pub fn resolution(mut self, resolution: Resolution) -> Self {
            self.resolution = resolution;
            self
        }
        /// change the align field
        pub fn align(mut self, align: Align) -> Self {
            self.align = align;
            self
        }
        /// change the scan field
        pub fn sequence(mut self, sequence: SequenceMode) -> Self {
            self.sequence = sequence;
            self
        }
        /// change the external_trigger field
        pub fn external_trigger(
            mut self,
            trigger_mode: TriggerMode,
            trigger: ExternalTrigger,
        ) -> Self {
            self.external_trigger = (trigger_mode, trigger);
            self
        }
        /// change the continuous field
        pub fn continuous(mut self, continuous: Continuous) -> Self {
            self.continuous = continuous;
            self
        }
        /// change the dma field
        pub fn dma(mut self, dma: Dma) -> Self {
            self.dma = dma;
            self
        }
        /// change the end_of_conversion_interrupt field
        pub fn end_of_conversion_interrupt(mut self, end_of_conversion_interrupt: Eoc) -> Self {
            self.end_of_conversion_interrupt = end_of_conversion_interrupt;
            self
        }
        /// change the default_sample_time field
        pub fn default_sample_time(mut self, default_sample_time: SampleTime) -> Self {
            self.default_sample_time = default_sample_time;
            self
        }

        /// Specify the reference voltage for the ADC.
        ///
        /// # Args
        /// * `vdda_mv` - The ADC reference voltage in millivolts.
        pub fn reference_voltage(mut self, vdda_mv: u32) -> Self {
            self.vdda = Some(vdda_mv);
            self
        }
    }

    impl Default for AdcConfig {
        fn default() -> Self {
            Self {
                clock: Clock::Pclk2_div_2,
                resolution: Resolution::Twelve,
                align: Align::Right,
                sequence: SequenceMode::Disabled,
                external_trigger: (TriggerMode::Disabled, ExternalTrigger::Tim_1_cc_1),
                continuous: Continuous::Single,
                dma: Dma::Disabled,
                end_of_conversion_interrupt: Eoc::Disabled,
                default_sample_time: SampleTime::Cycles_480,
                vdda: None,
            }
        }
    }
}

/// Analog to Digital Converter
#[derive(Clone, Copy)]
pub struct Adc<ADC> {
    /// Current config of the ADC, kept up to date by the various set methods
    config: config::AdcConfig,
    /// The adc peripheral
    adc_reg: ADC,
    /// VDDA in millivolts calculated from the factory calibration and vrefint
    calibrated_vdda: u32,
    /// Exclusive limit for the sample value possible for the configured resolution.
    max_sample: u32,
}
impl<ADC> fmt::Debug for Adc<ADC> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Adc: {{ calibrated_vdda: {:?}, max_sample: {:?}, config: {:?}, ... }}",
            self.calibrated_vdda, self.max_sample, self.config
        )
    }
}

macro_rules! adc {
    // Note that only ADC1 supports measurement of VREF, VBAT, and the internal temperature sensor.
    (additionals: ADC1 => ($common_type:ident)) => {
        /// Calculates the system VDDA by sampling the internal VREF channel and comparing
        /// the result with the value stored at the factory.
        pub fn calibrate(&mut self) {
            self.enable();

            let vref_en = self.temperature_and_vref_enabled();
            if !vref_en {
                self.enable_temperature_and_vref();
            }

            let vref_cal = VrefCal::get().read();
            let vref_samp = self.read(&mut Vref).unwrap(); //This can't actually fail, it's just in a result to satisfy hal trait

            self.calibrated_vdda = (VDDA_CALIB * u32::from(vref_cal)) / u32::from(vref_samp);
            if !vref_en {
                self.disable_temperature_and_vref();
            }
        }

        /// Enables the vbat internal channel
        pub fn enable_vbat(&self) {
            unsafe {
                let common = &(*pac::$common_type::ptr());
                common.ccr.modify(|_, w| w.vbate().set_bit());
            }
        }

        /// Enables the vbat internal channel
        pub fn disable_vbat(&self) {
            unsafe {
                let common = &(*pac::$common_type::ptr());
                common.ccr.modify(|_, w| w.vbate().clear_bit());
            }
        }

        /// Enables the temp and vref internal channels.
        /// They can't work while vbat is also enabled so this method also disables vbat.
        pub fn enable_temperature_and_vref(&mut self) {
            //VBAT prevents TS and VREF from being sampled
            self.disable_vbat();
            unsafe {
                let common = &(*pac::$common_type::ptr());
                common.ccr.modify(|_, w| w.tsvrefe().set_bit());
            }
        }

        /// Disables the temp and vref internal channels
        pub fn disable_temperature_and_vref(&mut self) {
            unsafe {
                let common = &(*pac::$common_type::ptr());
                common.ccr.modify(|_, w| w.tsvrefe().clear_bit());
            }
        }

        /// Returns if the temp and vref internal channels are enabled
        pub fn temperature_and_vref_enabled(&mut self) -> bool {
            unsafe {
                let common = &(*pac::$common_type::ptr());
                common.ccr.read().tsvrefe().bit_is_set()
            }
        }
    };

    // Provide a stub implementation for ADCs that do not have a means of sampling VREF.
    (additionals: $adc_type:ident => ($common_type:ident)) => {
        fn calibrate(&mut self) {}
    };

    ($($adc_type:ident => ($constructor_fn_name:ident, $common_type:ident, $en_bit: expr)),+ $(,)*) => {
        $(

            impl Adc<pac::$adc_type> {
                adc!(additionals: $adc_type => ($common_type));

                pub fn $constructor_fn_name(adc: pac::$adc_type, reset: bool, config: config::AdcConfig) -> Adc<pac::$adc_type> {
                    unsafe {
                        // All ADCs share the same reset interface.
                        // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                        let crm = &(*pac::CRM::ptr());

                        //Enable the clock
                        pac::$adc_type::enable(crm);

                        if reset {
                            //Reset the peripheral(s)
                            pac::$adc_type::reset(crm);
                        }
                    }

                    let mut s = Self {
                        config,
                        adc_reg: adc,
                        calibrated_vdda: VDDA_CALIB,
                        max_sample: 0,
                    };

                    //Probably unnecessary to disable the ADC in most cases but it shouldn't do any harm either
                    s.disable();
                    s.apply_config(config);

                    s.enable();
                    s.calibrate();

                    // If the user specified a VDDA, use that over the internally determined value.
                    if let Some(vdda) = s.config.vdda {
                        s.calibrated_vdda = vdda;
                    }

                    s
                }

                /// Applies all fields in AdcConfig
                pub fn apply_config(&mut self, config: config::AdcConfig) {
                    self.set_clock(config.clock);
                    // self.set_resolution(config.resolution);
                    self.set_align(config.align);
                    self.set_sequence(config.sequence);
                    // self.set_external_trigger(config.external_trigger);
                    // self.set_continuous(config.continuous);
                    // self.set_dma(config.dma);
                    // self.set_end_of_conversion_interrupt(config.end_of_conversion_interrupt);
                    // self.set_default_sample_time(config.default_sample_time);

                    if let Some(vdda) = config.vdda {
                        self.calibrated_vdda = vdda;
                    }
                }

                /// Returns if the adc is enabled
                pub fn is_enabled(&self) -> bool {
                    self.adc_reg.ctrl2().read().adcen().is_enabled()
                }

                /// Enables the adc
                pub fn enable(&mut self) {
                    self.adc_reg.ctrl2().modify(|_, w| w.adcen().enable());
                }

                /// Disables the adc
                /// # Note
                /// The ADC in the f4 has few restrictions on what can be configured while the ADC
                /// is enabled. If any bugs are found where some settings aren't "sticking" try disabling
                /// the ADC before changing them. The reference manual for the chip I'm using only states
                /// that the sequence registers are locked when they are being converted.
                pub fn disable(&mut self) {
                    self.adc_reg.ctrl2().modify(|_, w| w.adcen().disable());
                }

                /// Starts conversion sequence. Waits for the hardware to indicate it's actually started.
                pub fn start_conversion(&mut self) {
                    self.enable();
                    self.clear_end_of_conversion_flag();
                    //Start conversion
                    self.adc_reg.ctrl2().modify(|_, w| w.ocswtrg().triggered());

                    while self.adc_reg.sts().read().occe().is_not_complete() {}
                }

                #[cfg(any(
                    feature = "at32a403a",
                    feature = "at32f403",
                    feature = "at32f403a",
                    feature = "at32f407",
                    feature = "at32f413",
                    feature = "at32f415",
                    feature = "at32f421",
                    feature = "at32f425",
                    feature = "at32wb415",
                ))]
                /// Sets the clock for the adc
                pub fn set_clock(&mut self, clock: config::Clock) {
                    self.config.clock = clock;
                    unsafe {
                        let crm = &(*pac::CRM::ptr());
                        crm.cfg().modify(|_, w| w.adcdiv1_0().bits(clock as _));
                        crm.cfg().modify(|_, w| w.adcdiv2().bit((clock as u8) & 0b100 != 0));
                    }
                }

                #[cfg(any(
                    feature = "at32f402",
                    feature = "at32f405",
                    feature = "at32f423",
                    feature = "at32f435",
                    feature = "at32f437",
                ))]
                /// Sets the clock for the adc
                pub fn set_clock(&mut self, clock: config::Clock) {
                    self.adc_reg.cctrl().modify(|_, w| w.adcdiv().bits(clock as _));
                }

                #[cfg(any(
                    feature = "at32f402",
                    feature = "at32f405",
                    feature = "at32f423",
                    feature = "at32f435",
                    feature = "at32f437",
                ))]
                /// Sets the sampling resolution
                pub fn set_resolution(&mut self, resolution: config::Resolution) {
                    self.max_sample = match resolution {
                        config::Resolution::Twelve => (1 << 12),
                        config::Resolution::Ten => (1 << 10),
                        config::Resolution::Eight => (1 << 8),
                        config::Resolution::Six => (1 << 6),
                    };
                    self.config.resolution = resolution;
                    self.adc_reg.ctrl1().modify(|_, w| w.res().bits(resolution as _));
                }

                /// Sets the DR register alignment to left or right
                pub fn set_align(&mut self, align: config::Align) {
                    self.config.align = align;
                    self.adc_reg.ctrl2().modify(|_, w| w.dtalign().bit(align.into()));
                }

                /// Enables and disables sequence (scan) mode
                pub fn set_sequence(&mut self, sequence: config::SequenceMode) {
                    self.config.sequence = sequence;
                    self.adc_reg.ctrl1().modify(|_, w| w.sqen().bit(sequence.into()));
                }

                /// Resets the end-of-conversion flag
                pub fn clear_end_of_conversion_flag(&mut self) {
                    self.adc_reg.sts().modify(|_, w| w.occe().clear());
                }

                /// Reset the sequence
                pub fn reset_sequence(&mut self) {
                    //The reset state is One conversion selected
                    self.adc_reg.osq1().modify(|_, w| w.oclen().bits(config::Sequence::One.into()));
                }

                /// Returns the address of the ADC data register. Primarily useful for configuring DMA.
                pub fn data_register_address(&mut self) -> u32 {
                    self.adc_reg.odt().as_ptr() as u32
                }

                /// Configure a channel for sampling.
                /// It will make sure the sequence is at least as long as the `sequence` provided.
                /// # Arguments
                /// * `channel` - channel to configure
                /// * `sequence` - where in the sequence to sample the channel. Also called rank in some STM docs/code
                /// * `sample_time` - how long to sample for. See datasheet and ref manual to work out how long you need\
                /// to sample for at a given ADC clock frequency
                pub fn configure_channel<CHANNEL>(&mut self, _channel: &CHANNEL, sequence: config::Sequence, sample_time: config::SampleTime)
                where
                    CHANNEL: Channel<pac::$adc_type, ID=u8>
                {
                    //Check the sequence is long enough
                    self.adc_reg.osq1().modify(|r, w| {
                        let prev: config::Sequence = r.oclen().bits().into();
                        if prev < sequence {
                            w.oclen().bits(sequence.into())
                        } else {
                            w
                        }
                    });

                    let channel = CHANNEL::channel();

                    //Set the channel in the right sequence field
                    match sequence {
                        config::Sequence::One      => self.adc_reg.osq3().modify(|_, w| unsafe {w.osn1().bits(channel) }),
                        config::Sequence::Two      => self.adc_reg.osq3().modify(|_, w| unsafe {w.osn2().bits(channel) }),
                        config::Sequence::Three    => self.adc_reg.osq3().modify(|_, w| unsafe {w.osn3().bits(channel) }),
                        config::Sequence::Four     => self.adc_reg.osq3().modify(|_, w| unsafe {w.osn4().bits(channel) }),
                        config::Sequence::Five     => self.adc_reg.osq3().modify(|_, w| unsafe {w.osn5().bits(channel) }),
                        config::Sequence::Six      => self.adc_reg.osq3().modify(|_, w| unsafe {w.osn6().bits(channel) }),
                        config::Sequence::Seven    => self.adc_reg.osq2().modify(|_, w| unsafe {w.osn7().bits(channel) }),
                        config::Sequence::Eight    => self.adc_reg.osq2().modify(|_, w| unsafe {w.osn8().bits(channel) }),
                        config::Sequence::Nine     => self.adc_reg.osq2().modify(|_, w| unsafe {w.osn9().bits(channel) }),
                        config::Sequence::Ten      => self.adc_reg.osq2().modify(|_, w| unsafe {w.osn10().bits(channel) }),
                        config::Sequence::Eleven   => self.adc_reg.osq2().modify(|_, w| unsafe {w.osn11().bits(channel) }),
                        config::Sequence::Twelve   => self.adc_reg.osq2().modify(|_, w| unsafe {w.osn12().bits(channel) }),
                        config::Sequence::Thirteen => self.adc_reg.osq1().modify(|_, w| unsafe {w.osn13().bits(channel) }),
                        config::Sequence::Fourteen => self.adc_reg.osq1().modify(|_, w| unsafe {w.osn14().bits(channel) }),
                        config::Sequence::Fifteen  => self.adc_reg.osq1().modify(|_, w| unsafe {w.osn15().bits(channel) }),
                        config::Sequence::Sixteen  => self.adc_reg.osq1().modify(|_, w| unsafe {w.osn16().bits(channel) }),
                    }

                    fn replace_bits(mut v: u32, offset: u32, width: u32, value: u32) -> u32 {
                        let mask = !(((1 << width) -1) << (offset * width));
                        v &= mask;
                        v |= value << (offset * width);
                        v
                    }

                    //Set the sample time for the channel
                    let st = sample_time as u32;
                    let ch = channel as u32;
                    match channel {
                        0..=9   => self.adc_reg.spt2().modify(|r, w| unsafe { w.bits(replace_bits(r.bits(), ch, 3, st)) }),
                        10..=18 => self.adc_reg.spt1().modify(|r, w| unsafe { w.bits(replace_bits(r.bits(), ch-10, 3, st)) }),
                        _ => unimplemented!(),
                    }
                }

                /// Returns the current sample stored in the ADC data register
                pub fn current_sample(&self) -> u16 {
                    self.adc_reg.odt().read().odt().bits()
                }

                /// Block until the conversion is completed
                /// # Panics
                /// Will panic if there is no conversion started and the end-of-conversion bit is not set
                pub fn wait_for_conversion_sequence(&self) {
                    if self.adc_reg.sts().read().occs().is_idle() && self.adc_reg.sts().read().occe().is_not_complete() {
                        panic!("Waiting for end-of-conversion but no conversion started");
                    }
                    while self.adc_reg.sts().read().occe().is_not_complete() {}
                    //Clear the conversion started flag
                    self.adc_reg.sts().modify(|_, w| w.occs().clear());
                }

                /// Synchronously convert a single sample
                /// Note that it reconfigures the adc sequence and doesn't restore it
                pub fn convert<PIN>(&mut self, pin: &PIN, sample_time: config::SampleTime) -> u16
                where
                    PIN: Channel<pac::$adc_type, ID=u8>
                {
                    self.adc_reg.ctrl2().modify(|_, w| w
                        .ocdmaen().clear_bit() //Disable dma
                        .rpen().disable() //Disable continuous mode
                        .octen().bit(config::TriggerMode::Disabled.into()) //Disable trigger
                        // .eocs().clear_bit() //EOC is set at the end of the sequence
                    );
                    self.adc_reg.ctrl1().modify(|_, w| w
                        .sqen().disable() //Disable scan mode
                        .cceien().disable() //Disable end of conversion interrupt
                    );

                    self.reset_sequence();
                    self.configure_channel(pin, config::Sequence::One, sample_time);
                    self.start_conversion();

                    //Wait for the sequence to complete
                    self.wait_for_conversion_sequence();

                    let result = self.current_sample();

                    //Reset the config
                    self.apply_config(self.config);

                    result
                }

            }

            impl Adc<pac::$adc_type> {
                fn read<PIN>(&mut self, pin: &mut PIN) -> nb::Result<u16, ()>
                    where PIN: Channel<pac::$adc_type, ID=u8>,
                {
                    let enabled = self.is_enabled();
                    if !enabled {
                        self.enable();
                    }

                    let sample = self.convert(pin, self.config.default_sample_time);

                    if !enabled {
                        self.disable();
                    }

                    Ok(sample)
                }
            }

        )+
    };
}

pub trait Channel<ADC> {
    /// Channel ID type
    ///
    /// A type used to identify this ADC channel. For example, if the ADC has eight channels, this
    /// might be a `u8`. If the ADC has multiple banks of channels, it could be a tuple, like
    /// `(u8: bank_id, u8: channel_id)`.
    type ID;

    /// Get the specific ID that identifies this channel, for example `0_u8` for the first ADC
    /// channel, if Self::ID is u8.
    fn channel() -> Self::ID;

    // `channel` is a function due to [this reported
    // issue](https://github.com/rust-lang/rust/issues/54973). Something about blanket impls
    // combined with `type ID; const CHANNEL: Self::ID;` causes problems.
    //const CHANNEL: Self::ID;
}

adc!(ADC1 => (adc1, ADC_COMMON, 8));
