//! Replaces the libdwf functions used here with local definitions, so the
//! wrappers can be tested without a device attached.
use dwf::device_info::{DeviceId, DeviceInfo};
use std::ffi::c_int;

#[no_mangle]
unsafe extern "C" fn FDwfEnumDeviceIsOpened(idx_device: c_int, pf_is_used: *mut c_int) -> c_int {
    *pf_is_used = (idx_device == 0) as c_int;
    1
}

struct Index(c_int);

impl DeviceId for Index {
    fn get_device_id(&self) -> dwf::Result<c_int> {
        Ok(self.0)
    }
}

impl DeviceInfo for Index {}

#[test]
fn device_in_use_reads_c_bool() {
    assert_eq!(Index(0).device_in_use(), Ok(true));
    assert_eq!(Index(1).device_in_use(), Ok(false));
}
