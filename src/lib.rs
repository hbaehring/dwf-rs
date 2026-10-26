use crate::device_info::EnumFilter;
use crate::dwf::DwfError;
use crate::Error::FailToGetErrorMessage;
use std::ffi::{c_char, CStr};
use std::mem;

pub mod analog;
pub mod device;
pub mod device_info;
pub mod digital;
pub mod dwf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Debug)]
pub enum Error {
    FailToRun,
    FailToGetErrorCode,
    FailToGetErrorMessage,
    FailWithErrorCode(DwfError),
    /// `Device::open(-1, ..)` found no enumerated device that is not in use.
    NoDeviceAvailable,
    /// The device was opened without an enumeration index (`Device::open_ex`).
    UnknownDeviceIndex,
    /// A string argument contains a NUL byte and can't be passed to C.
    InvalidString,
}

impl Error {
    pub fn get_error_message(&self) -> Result<String> {
        unsafe {
            let mut error_message = [0 as c_char; 512];
            if dwf::from_c_bool(dwf::FDwfGetLastErrorMsg(error_message.as_mut_ptr())) {
                Ok(CStr::from_ptr(error_message.as_ptr())
                    .to_str()
                    .unwrap()
                    .to_owned())
            } else {
                Err(FailToGetErrorMessage)
            }
        }
    }
}

fn check_call(status: dwf::BOOL) -> Result<()> {
    if dwf::from_c_bool(status) {
        Ok(())
    } else {
        Err(get_error_code())
    }
}

fn get_error_code() -> Error {
    unsafe {
        let mut error_code = mem::MaybeUninit::uninit();
        if dwf::from_c_bool(dwf::FDwfGetLastError(
            (error_code.as_mut_ptr()) as *mut dwf::DWFERC,
        )) {
            let error_code = error_code.assume_init();
            let error = DwfError::from(error_code);
            if error == DwfError::NoError {
                Error::FailToRun
            } else {
                Error::FailWithErrorCode(error)
            }
        } else {
            Error::FailToGetErrorCode
        }
    }
}

pub fn get_version() -> Result<String> {
    let mut version = [0 as c_char; 32];
    unsafe {
        check_call(dwf::FDwfGetVersion(version.as_mut_ptr()))?;

        Ok(CStr::from_ptr(version.as_ptr())
            .to_str()
            .unwrap()
            .to_owned())
    }
}

pub fn enumerate_devices(filter: EnumFilter) -> Result<i32> {
    let filter = match filter {
        EnumFilter::All => dwf::enumfilterAll,
        EnumFilter::Device(device_type) => device_type.to_devid(),
        EnumFilter::Types(types) => types.to_enumfilter(),
    };
    let mut num_devices = mem::MaybeUninit::uninit();
    unsafe {
        check_call(dwf::FDwfEnum(filter, num_devices.as_mut_ptr()))?;
        let num_devices = num_devices.assume_init();
        Ok(num_devices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_version() {
        let version = get_version().unwrap();
        assert_ne!(version, "".to_string());
    }

    #[test]
    fn check_error_code() {
        let error = get_error_code();

        assert_eq!(error, Error::FailToRun);
        assert_eq!(error.get_error_message(), Ok("".to_string()));
    }
}
