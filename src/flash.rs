use embedded_storage::nor_flash::{
    ErrorType, MultiwriteNorFlash, NorFlash, NorFlashError, NorFlashErrorKind, ReadNorFlash,
};

use crate::pac::FLASH;
use crate::signature::FlashSize;
use core::{ptr, slice};

/// Flash erase/program error
#[derive(Debug, Clone, Copy)]
pub enum Error {
    Programming,
    WriteProtection,
}

impl Error {
    fn read(flash: &FLASH) -> Option<Self> {
        let sts = flash.sts.read();
        if sts.prgmerr().bit() {
            Some(Error::Programming)
        } else if sts.epperr().bit() {
            Some(Error::WriteProtection)
        } else {
            None
        }
    }
}

impl NorFlashError for Error {
    fn kind(&self) -> NorFlashErrorKind {
        NorFlashErrorKind::Other
    }
}

#[allow(clippy::len_without_is_empty)]
pub trait FlashExt {
    /// Memory-mapped address
    fn address(&self) -> usize;
    /// Size in bytes
    fn len(&self) -> usize;
    /// Returns a read-only view of flash memory
    fn read(&self) -> &[u8] {
        let ptr = self.address() as *const _;
        unsafe { slice::from_raw_parts(ptr, self.len()) }
    }
    /// Unlock flash for erasing/programming until this method's
    /// result is dropped
    fn unlocked(&mut self) -> UnlockedFlash;
    // Returns true if flash is in dual bank organization
    fn dual_bank(&self) -> bool;
    /// Returns flash memory sector of a given offset. Returns none if offset is out of range.
    fn sector(&self, offset: usize) -> Option<FlashSector>;
}

impl FlashExt for FLASH {
    fn address(&self) -> usize {
        0x0800_0000
    }

    fn len(&self) -> usize {
        FlashSize::get().bytes()
    }

    fn unlocked(&mut self) -> UnlockedFlash {
        unlock(self);
        UnlockedFlash { flash: self }
    }

    fn dual_bank(&self) -> bool {
        match self.len() / 1024 {
            // 1 MB devices depend on configuration
            1024 => {
                if cfg!(any(feature = "at32f403", feature = "at32f407",)) {
                    true
                } else {
                    false
                }
            }
            // 2 MB devices are always dual bank
            2048 => true,
            // All other devices are single bank
            _ => false,
        }
    }

    fn sector(&self, offset: usize) -> Option<FlashSector> {
        flash_sectors(self.len(), self.dual_bank()).find(|s| s.contains(offset))
    }
}

/// Read-only flash
///
/// # Examples
///
/// ```
/// use stm32f4xx_hal::pac::Peripherals;
/// use stm32f4xx_hal::flash::LockedFlash;
/// use embedded_storage::nor_flash::ReadNorFlash;
///
/// let dp = Peripherals::take().unwrap();
/// let mut flash = LockedFlash::new(dp.FLASH);
/// println!("Flash capacity: {}", ReadNorFlash::capacity(&flash));
///
/// let mut buf = [0u8; 64];
/// ReadNorFlash::read(&mut flash, 0x0, &mut buf).unwrap();
/// println!("First 64 bytes of flash memory: {:?}", buf);
/// ```
pub struct LockedFlash {
    flash: FLASH,
}

impl LockedFlash {
    pub fn new(flash: FLASH) -> Self {
        Self { flash }
    }
}

impl FlashExt for LockedFlash {
    fn address(&self) -> usize {
        self.flash.address()
    }

    fn len(&self) -> usize {
        self.flash.len()
    }

    fn unlocked(&mut self) -> UnlockedFlash {
        self.flash.unlocked()
    }

    fn dual_bank(&self) -> bool {
        self.flash.dual_bank()
    }

    fn sector(&self, offset: usize) -> Option<FlashSector> {
        self.flash.sector(offset)
    }
}

/// Result of `FlashExt::unlocked()`
///
/// # Examples
///
/// ```
/// use stm32f4xx_hal::pac::Peripherals;
/// use stm32f4xx_hal::flash::{FlashExt, LockedFlash, UnlockedFlash};
/// use embedded_storage::nor_flash::NorFlash;
///
/// let dp = Peripherals::take().unwrap();
/// let mut flash = LockedFlash::new(dp.FLASH);
///
/// // Unlock flash for writing
/// let mut unlocked_flash = flash.unlocked();
///
/// // Erase the second 128 KB sector.
/// NorFlash::erase(&mut unlocked_flash, 128 * 1024, 256 * 1024).unwrap();
///
/// // Write some data at the start of the second 128 KB sector.
/// let buf = [0u8; 64];
/// NorFlash::write(&mut unlocked_flash, 128 * 1024, &buf).unwrap();
///
/// // Lock flash by dropping
/// drop(unlocked_flash);
/// ```
pub struct UnlockedFlash<'a> {
    flash: &'a mut FLASH,
}

/// Automatically lock flash erase/program when leaving scope
impl Drop for UnlockedFlash<'_> {
    fn drop(&mut self) {
        lock(self.flash);
    }
}

impl UnlockedFlash<'_> {
    /// Erase a flash page
    ///
    /// Refer to the reference manual to see which page corresponds
    /// to which memory address.
    pub fn erase(&mut self, page: u8) -> Result<(), Error> {
        self.flash
            .addr
            .write(|w| unsafe { w.fa().bits(page.into()) });
        self.flash
            .ctrl
            .modify(|_, w| w.secers().set_bit().erstr().set_bit());
        self.wait_ready();
        self.ok()
    }

    /// Program bytes with offset into flash memory,
    /// aligned to 128-bit rows
    pub fn program<'a, I>(&mut self, mut offset: usize, mut bytes: I) -> Result<(), Error>
    where
        I: Iterator<Item = &'a u8>,
    {
        let ptr = self.flash.address() as *mut u8;
        let mut bytes_written = 1;
        while bytes_written > 0 {
            bytes_written = 0;
            let amount = 16 - (offset % 16);

            #[rustfmt::skip]
            #[allow(unused_unsafe)]
            self.flash.ctrl.modify(|_, w| unsafe {
                w
                    // programming
                    .fprgm().set_bit()
            });
            for _ in 0..amount {
                match bytes.next() {
                    Some(byte) => {
                        unsafe {
                            ptr::write_volatile(ptr.add(offset), *byte);
                        }
                        offset += 1;
                        bytes_written += 1;
                    }
                    None => break,
                }
            }
            self.wait_ready();
            self.ok()?;
        }

        Ok(())
    }

    fn ok(&self) -> Result<(), Error> {
        Error::read(self.flash).map(Err).unwrap_or(Ok(()))
    }

    fn wait_ready(&self) {
        while self.flash.sts.read().obf().bit() {}
    }
}

const UNLOCK_KEY1: u32 = 0x45670123;
const UNLOCK_KEY2: u32 = 0xCDEF89AB;

#[allow(unused_unsafe)]
fn unlock(flash: &FLASH) {
    flash
        .unlock
        .write(|w| unsafe { w.ukval().bits(UNLOCK_KEY1) });
    flash
        .unlock
        .write(|w| unsafe { w.ukval().bits(UNLOCK_KEY2) });
    assert!(!flash.ctrl.read().oplk().bit())
}

fn lock(flash: &FLASH) {
    flash.ctrl.modify(|_, w| w.oplk().set_bit());
}

/// Flash memory sector
pub struct FlashSector {
    /// Sector number
    pub number: u8,
    /// Offset from base memory address
    pub offset: usize,
    /// Sector size in bytes
    pub size: usize,
}

impl FlashSector {
    /// Returns true if given offset belongs to this sector
    pub fn contains(&self, offset: usize) -> bool {
        self.offset <= offset && offset < self.offset + self.size
    }
}

/// Iterator of flash memory sectors in a single bank.
/// Yields a size sequence of [16, 16, 16, 64, 128, 128, ..]
pub struct FlashSectorIterator {
    index: u8,
    start_sector: u8,
    start_offset: usize,
    end_offset: usize,
}

impl FlashSectorIterator {
    fn new(start_sector: u8, start_offset: usize, end_offset: usize) -> Self {
        Self {
            index: 0,
            start_sector,
            start_offset,
            end_offset,
        }
    }
}

impl Iterator for FlashSectorIterator {
    type Item = FlashSector;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_offset >= self.end_offset {
            None
        } else {
            // First 4 sectors are 16 KB, then one 64 KB and the rest are 128 KB
            let size = match self.index {
                0..=3 => 16 * 1024,
                4 => 64 * 1024,
                _ => 128 * 1024,
            };

            let sector = FlashSector {
                number: self.start_sector + self.index,
                offset: self.start_offset,
                size,
            };

            self.index += 1;
            self.start_offset += size;

            Some(sector)
        }
    }
}

/// Returns iterator of flash memory sectors for single and dual bank flash.
/// Sectors are returned in continuous memory order, while sector numbers can have spaces between banks.
pub fn flash_sectors(flash_size: usize, dual_bank: bool) -> impl Iterator<Item = FlashSector> {
    if dual_bank {
        // Second memory bank always starts from sector 12
        FlashSectorIterator::new(0, 0, flash_size / 2).chain(FlashSectorIterator::new(
            12,
            flash_size / 2,
            flash_size,
        ))
    } else {
        // Chain an empty iterator to match types
        FlashSectorIterator::new(0, 0, flash_size).chain(FlashSectorIterator::new(0, 0, 0))
    }
}

impl ErrorType for LockedFlash {
    type Error = Error;
}

impl ReadNorFlash for LockedFlash {
    const READ_SIZE: usize = 1;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let offset = offset as usize;
        bytes.copy_from_slice(&self.flash.read()[offset..offset + bytes.len()]);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.flash.len()
    }
}

impl<'a> ErrorType for UnlockedFlash<'a> {
    type Error = Error;
}

impl<'a> ReadNorFlash for UnlockedFlash<'a> {
    const READ_SIZE: usize = 1;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let offset = offset as usize;
        bytes.copy_from_slice(&self.flash.read()[offset..offset + bytes.len()]);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.flash.len()
    }
}

impl<'a> NorFlash for UnlockedFlash<'a> {
    const WRITE_SIZE: usize = 1;

    // Use largest sector size of 128 KB. All smaller sectors will be erased together.
    const ERASE_SIZE: usize = 128 * 1024;

    fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        let mut current = from as usize;

        for sector in flash_sectors(self.flash.len(), self.flash.dual_bank()) {
            if sector.contains(current) {
                UnlockedFlash::erase(self, sector.number)?;
                current += sector.size;
            }

            if current >= to as usize {
                break;
            }
        }

        Ok(())
    }

    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.program(offset as usize, bytes.iter())
    }
}

// STM32F4 supports multiple writes
impl<'a> MultiwriteNorFlash for UnlockedFlash<'a> {}
