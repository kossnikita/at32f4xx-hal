//! Device electronic signature
//!
//! (stored in flash memory)

/// This is the test voltage, in millivolts of the calibration done at the factory
pub const VDDA_CALIB: u32 = 3300;

macro_rules! define_ptr_type {
    ($name: ident, $ptr: expr) => {
        impl $name {
            fn ptr() -> *const Self {
                $ptr as *const _
            }

            /// Returns a wrapped reference to the value in flash memory
            pub fn get() -> &'static Self {
                unsafe { &*Self::ptr() }
            }
        }
    };
}

#[cfg(any(feature = "at32f421"))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
#[repr(u32)]
pub enum Mcu {
    AT32F421C8T7 = 0x50020100,
    AT32F421K8T7 = 0x50020101,
    AT32F421K8U7 = 0x50020102,
    AT32F421K8U7_4 = 0x50020103,
    AT32F421F8U7 = 0x50020104,
    AT32F421F8P7 = 0x50020105,
    AT32F421C6T7 = 0x50020086,
    AT32F421K6T7 = 0x50020087,
    AT32F421K6U7 = 0x50020088,
    AT32F421K6U7_4 = 0x50020089,
    AT32F421F6U7 = 0x5002008A,
    AT32F421F6P7 = 0x5002008B,
    AT32F421C4T7 = 0x5001000C,
    AT32F421K4T7 = 0x5001000D,
    AT32F421K4U7 = 0x5001000E,
    AT32F421K4U7_4 = 0x5001000F,
    AT32F421F4U7 = 0x50010010,
    AT32F421F4P7 = 0x50010011,
    AT32F421G8U7 = 0x50020112,
    AT32F421G6U7 = 0x50020093,
    AT32F421G4U7 = 0x50010014,
}

impl From<u32> for Mcu {
    fn from(pid: u32) -> Self {
        match pid {
            0x50020100 => Mcu::AT32F421C8T7,
            0x50020101 => Mcu::AT32F421K8T7,
            0x50020102 => Mcu::AT32F421K8U7,
            0x50020103 => Mcu::AT32F421K8U7_4,
            0x50020104 => Mcu::AT32F421F8U7,
            0x50020105 => Mcu::AT32F421F8P7,
            0x50020086 => Mcu::AT32F421C6T7,
            0x50020087 => Mcu::AT32F421K6T7,
            0x50020088 => Mcu::AT32F421K6U7,
            0x50020089 => Mcu::AT32F421K6U7_4,
            0x5002008A => Mcu::AT32F421F6U7,
            0x5002008B => Mcu::AT32F421F6P7,
            0x5001000C => Mcu::AT32F421C4T7,
            0x5001000D => Mcu::AT32F421K4T7,
            0x5001000E => Mcu::AT32F421K4U7,
            0x5001000F => Mcu::AT32F421K4U7_4,
            0x50010010 => Mcu::AT32F421F4U7,
            0x50010011 => Mcu::AT32F421F4P7,
            0x50020112 => Mcu::AT32F421G8U7,
            0x50020093 => Mcu::AT32F421G6U7,
            0x50010014 => Mcu::AT32F421G4U7,
            _ => unimplemented!(),
        }
    }
}

impl From<Mcu> for u32 {
    fn from(m: Mcu) -> u32 {
        m as _
    }
}

/// Uniqure Device ID register
#[derive(Hash, Debug)]
#[repr(C)]
pub struct IDCode(u32);
define_ptr_type!(IDCode, 0xE004_2000);

impl IDCode {
    /// PID information
    pub fn pid(&self) -> u32 {
        self.0
    }

    /// MCU information
    pub fn mcu(&self) -> Mcu {
        Mcu::from(self.0)
    }
}

/// Size of integrated flash
#[derive(Debug)]
#[repr(C)]
pub struct FlashSize(u16);

#[cfg(any())]
define_ptr_type!(FlashSize, 0x1FFF_7A22);

#[cfg(any(feature = "at32f421"))]
impl FlashSize {
    pub fn from_pid(pid: u32) -> Self {
        let size = match pid {
            0x50020100 => 64,
            0x50020101 => 64,
            0x50020102 => 64,
            0x50020103 => 64,
            0x50020104 => 64,
            0x50020105 => 64,
            0x50020112 => 64,
            0x50020086 => 32,
            0x50020087 => 32,
            0x50020088 => 32,
            0x50020089 => 32,
            0x5002008A => 32,
            0x5002008B => 32,
            0x50020093 => 32,
            0x5001000C => 16,
            0x5001000D => 16,
            0x5001000E => 16,
            0x5001000F => 16,
            0x50010010 => 16,
            0x50010011 => 16,
            0x50010014 => 16,
            _ => unimplemented!(),
        };
        FlashSize(size)
    }

    pub fn get() -> Self {
        let pid = IDCode::get().pid();
        Self::from_pid(pid)
    }
}

impl FlashSize {
    /// Read flash size in kilobytes
    pub fn kilo_bytes(&self) -> u16 {
        self.0
    }

    /// Read flash size in bytes
    pub fn bytes(&self) -> usize {
        usize::from(self.kilo_bytes()) * 1024
    }
}

#[cfg(any())]
/// ADC VREF calibration value is stored in at the factory
#[derive(Debug)]
#[repr(C)]
pub struct VrefCal(u16);

#[cfg(any())]
define_ptr_type!(VrefCal, 0x1FFF_7A2A);

#[cfg(any())]
impl VrefCal {
    /// Read calibration value
    pub fn read(&self) -> u16 {
        self.0
    }
}

#[cfg(any())]
/// A temperature reading taken at 30°C stored at the factory
#[derive(Debug)]
#[repr(C)]
pub struct VtempCal30(u16);

#[cfg(any())]
define_ptr_type!(VtempCal30, 0x1FFF_7A2C);

#[cfg(any())]
impl VtempCal30 {
    /// Read calibration value
    pub fn read(&self) -> u16 {
        self.0
    }
}

#[cfg(any())]
/// A temperature reading taken at 110°C stored at the factory
#[derive(Debug)]
#[repr(C)]
pub struct VtempCal110(u16);
#[cfg(any())]
define_ptr_type!(VtempCal110, 0x1FFF_7A2E);

#[cfg(any())]
impl VtempCal110 {
    /// Read calibration value
    pub fn read(&self) -> u16 {
        self.0
    }
}
