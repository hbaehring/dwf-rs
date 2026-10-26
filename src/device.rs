use crate::analog::analog_in::AnalogIn;
use crate::device_info::{DeviceId, DeviceInfo, EnumFilter};
use crate::digital::digital_in::DigitalIn;
use crate::dwf;
use crate::{check_call, Error, Result};
use std::ffi::{c_int, CString};

use std::mem;

/// An open device. The handle is closed when the value is dropped, so `Device`
/// must not be `Clone`:
///
/// ```compile_fail,E0277
/// fn assert_clone<T: Clone>() {}
/// assert_clone::<dwf::device::Device>();
/// ```
pub struct Device {
    handle: dwf::HDWF,
    // Enumeration index for the FDwfEnum* queries. Handles don't map to it.
    index: Option<c_int>,
}

impl Device {
    /// Opens the device at enumeration `index`. With `-1`, enumerates all
    /// devices and opens the first one that is not in use.
    pub fn open(index: i32, config: i32) -> Result<Device> {
        let index = if index == -1 {
            first_free_device()?
        } else {
            index
        };
        unsafe {
            let mut handle = mem::MaybeUninit::uninit();
            check_call(dwf::FDwfDeviceConfigOpen(
                index as c_int,
                config as c_int,
                handle.as_mut_ptr(),
            ))?;
            let handle = handle.assume_init();

            Ok(Device {
                handle,
                index: Some(index),
            })
        }
    }

    pub fn open_ex(opt: &str) -> Result<Device> {
        let opt_c_str = CString::new(opt).map_err(|_| Error::InvalidString)?;
        unsafe {
            let mut handle = mem::MaybeUninit::uninit();
            check_call(dwf::FDwfDeviceOpenEx(
                opt_c_str.as_ptr(),
                handle.as_mut_ptr(),
            ))?;
            let handle = handle.assume_init();
            Ok(Device {
                handle,
                index: None,
            })
        }
    }

    pub fn reset(&self) -> Result<()> {
        unsafe {
            check_call(dwf::FDwfDeviceReset(self.handle as dwf::HDWF))?;
        }
        Ok(())
    }

    pub fn get_handle(&self) -> dwf::HDWF {
        self.handle
    }

    pub fn analog_in(&self) -> AnalogIn<'_> {
        AnalogIn::new(self)
    }

    pub fn digital_in(&self) -> DigitalIn<'_> {
        DigitalIn::new(self)
    }

    pub fn trigger(&self) -> Result<()> {
        unsafe {
            check_call(dwf::FDwfDeviceTriggerPC(self.handle as dwf::HDWF))?;
        }
        Ok(())
    }
}

impl Default for Device {
    fn default() -> Self {
        Device::open(-1, 0).unwrap()
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            check_call(dwf::FDwfDeviceClose(self.handle as c_int)).unwrap();
        }
    }
}

impl DeviceId for Device {
    fn get_device_id(&self) -> Result<c_int> {
        self.index.ok_or(Error::UnknownDeviceIndex)
    }
}

impl DeviceInfo for Device {}

struct EnumIndex(c_int);

impl DeviceId for EnumIndex {
    fn get_device_id(&self) -> Result<c_int> {
        Ok(self.0)
    }
}

impl DeviceInfo for EnumIndex {}

fn first_free_device() -> Result<c_int> {
    for index in 0..crate::enumerate_devices(EnumFilter::All)? {
        if !EnumIndex(index).device_in_use()? {
            return Ok(index);
        }
    }
    Err(Error::NoDeviceAvailable)
}
