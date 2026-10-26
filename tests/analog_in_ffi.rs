//! Replaces the libdwf functions used here with local definitions, so the
//! wrappers can be tested without a device attached.
use dwf::analog::analog_in::AcquisitionMode;
use dwf::device::Device;
use std::ffi::c_int;
use std::sync::atomic::{AtomicI32, Ordering};

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
unsafe extern "C" fn FDwfAnalogInAcquisitionModeSet(_hdwf: c_int, acqmode: c_int) -> c_int {
    ACQUISITION_MODE.store(acqmode, Ordering::SeqCst);
    1
}

#[test]
#[allow(deprecated)]
fn set_acquisition_mode_maps_mode() {
    let device = Device::open(0, 0).unwrap();
    let analog_in = device.analog_in();

    for (mode, expected) in [
        (AcquisitionMode::Single, 0),
        (AcquisitionMode::ScanShift, 1),
        (AcquisitionMode::ScanScreen, 2),
        (AcquisitionMode::Record, 3),
        (AcquisitionMode::Overs, 4),
        (AcquisitionMode::Single1, 5),
    ] {
        analog_in.set_acquisition_mode(mode).unwrap();
        assert_eq!(ACQUISITION_MODE.load(Ordering::SeqCst), expected);
    }

    analog_in.set_record_mode().unwrap();
    assert_eq!(ACQUISITION_MODE.load(Ordering::SeqCst), 3);
}
