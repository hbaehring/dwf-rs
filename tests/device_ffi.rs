//! Replaces the libdwf functions used here with local definitions, so the
//! wrappers can be tested without a device attached.
use dwf::device::Device;
use dwf::device_info::DeviceInfo;
use dwf::Error;
use std::ffi::{c_char, c_int};

// Handles have no relation to enumeration indexes.
const HANDLE: c_int = 7;

#[no_mangle]
unsafe extern "C" fn FDwfEnum(_enumfilter: c_int, pc_device: *mut c_int) -> c_int {
    *pc_device = 3;
    1
}

#[no_mangle]
unsafe extern "C" fn FDwfEnumDeviceIsOpened(idx_device: c_int, pf_is_used: *mut c_int) -> c_int {
    *pf_is_used = (idx_device == 0) as c_int;
    1
}

#[no_mangle]
unsafe extern "C" fn FDwfEnumDeviceName(idx_device: c_int, sz_device_name: *mut c_char) -> c_int {
    let name = format!("Device {}\0", idx_device);
    std::ptr::copy_nonoverlapping(name.as_ptr() as *const c_char, sz_device_name, name.len());
    1
}

#[no_mangle]
unsafe extern "C" fn FDwfDeviceConfigOpen(
    idx_device: c_int,
    _idx_cfg: c_int,
    phdwf: *mut c_int,
) -> c_int {
    if !(0..3).contains(&idx_device) {
        return 0;
    }
    *phdwf = HANDLE;
    1
}

#[no_mangle]
unsafe extern "C" fn FDwfDeviceOpenEx(_sz_opt: *const c_char, phdwf: *mut c_int) -> c_int {
    *phdwf = HANDLE;
    1
}

#[no_mangle]
unsafe extern "C" fn FDwfDeviceClose(_hdwf: c_int) -> c_int {
    1
}

#[no_mangle]
unsafe extern "C" fn FDwfGetLastError(pdwferc: *mut c_int) -> c_int {
    *pdwferc = 0;
    1
}

#[test]
fn open_reports_info_of_opened_index() {
    let device = Device::open(2, 0).unwrap();
    assert_eq!(device.get_device_name(), Ok("Device 2".to_string()));
}

#[test]
fn default_opens_first_device_not_in_use() {
    let device = Device::default();
    assert_eq!(device.get_device_name(), Ok("Device 1".to_string()));
}

#[test]
fn open_ex_has_no_device_index() {
    let device = Device::open_ex("sn:123").unwrap();
    assert_eq!(device.get_device_name(), Err(Error::UnknownDeviceIndex));
}

#[test]
fn open_ex_rejects_nul_byte() {
    assert!(matches!(
        Device::open_ex("sn:\0abc"),
        Err(Error::InvalidString)
    ));
}
