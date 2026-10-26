//! Replaces the libdwf functions used here with local definitions, so the
//! wrappers can be tested without a device attached.
use dwf::device::Device;
use dwf::digital::digital_in::AcquisitionMode;
use std::ffi::{c_int, c_void};
use std::sync::atomic::{AtomicI32, Ordering};

static REQUESTED_BYTES: AtomicI32 = AtomicI32::new(-1);
static ACQUISITION_MODE: AtomicI32 = AtomicI32::new(-1);

#[no_mangle]
unsafe extern "C" fn FDwfDeviceConfigOpen(
    _idx_dev: c_int,
    _idx_cfg: c_int,
    phdwf: *mut c_int,
) -> c_int {
    *phdwf = 1;
    1
}

#[no_mangle]
unsafe extern "C" fn FDwfDeviceClose(_hdwf: c_int) -> c_int {
    1
}

#[no_mangle]
unsafe extern "C" fn FDwfDigitalInStatusData(
    _hdwf: c_int,
    rg_data: *mut c_void,
    count_of_data_bytes: c_int,
) -> c_int {
    REQUESTED_BYTES.store(count_of_data_bytes, Ordering::SeqCst);
    std::ptr::write_bytes(rg_data as *mut u8, 0xAB, count_of_data_bytes as usize);
    1
}

#[no_mangle]
unsafe extern "C" fn FDwfDigitalInAcquisitionModeSet(_hdwf: c_int, acqmode: c_int) -> c_int {
    ACQUISITION_MODE.store(acqmode, Ordering::SeqCst);
    1
}

#[test]
fn set_acquisition_mode_maps_mode() {
    let device = Device::open(0, 0).unwrap();
    let digital_in = device.digital_in();

    for (mode, expected) in [
        (AcquisitionMode::Single, 0),
        (AcquisitionMode::ScanShift, 1),
        (AcquisitionMode::ScanScreen, 2),
        (AcquisitionMode::Record, 3),
    ] {
        digital_in.set_acquisition_mode(mode).unwrap();
        assert_eq!(ACQUISITION_MODE.load(Ordering::SeqCst), expected);
    }
}

#[test]
fn fetch_samples_requests_bytes_for_sample_width() {
    let device = Device::open(0, 0).unwrap();
    let mut samples: Vec<u16> = vec![1];

    device.digital_in().fetch_samples(&mut samples, 4).unwrap();

    assert_eq!(REQUESTED_BYTES.load(Ordering::SeqCst), 8);
    assert_eq!(samples, [1, 0xABAB, 0xABAB, 0xABAB, 0xABAB]);
}
