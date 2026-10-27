//! Replaces the libdwf functions used here with local definitions, so the
//! wrappers can be tested without a device attached.
use dwf::analog::analog_in::AcquisitionMode;
use dwf::device::Device;
use std::ffi::c_int;
use std::sync::atomic::{AtomicI32, Ordering};

static ACQUISITION_MODE: AtomicI32 = AtomicI32::new(-1);
static CONFIGURE_FLAGS: AtomicI32 = AtomicI32::new(-1);

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

#[no_mangle]
unsafe extern "C" fn FDwfAnalogInConfigure(
    _hdwf: c_int,
    reconfigure: c_int,
    start: c_int,
) -> c_int {
    CONFIGURE_FLAGS.store(reconfigure * 10 + start, Ordering::SeqCst);
    1
}

#[no_mangle]
unsafe extern "C" fn FDwfAnalogInFrequencyGet(_hdwf: c_int, phz_frequency: *mut f64) -> c_int {
    *phz_frequency = 1_250_000.0;
    1
}

#[no_mangle]
unsafe extern "C" fn FDwfAnalogInFrequencyInfo(
    _hdwf: c_int,
    phz_min: *mut f64,
    phz_max: *mut f64,
) -> c_int {
    *phz_min = 1.0e-3;
    *phz_max = 100.0e6;
    1
}

#[test]
fn stop_neither_reconfigures_nor_starts() {
    let device = Device::open(0, 0).unwrap();
    device.analog_in().stop().unwrap();
    assert_eq!(CONFIGURE_FLAGS.load(Ordering::SeqCst), 0);
}

#[test]
fn get_frequency_returns_device_value() {
    let device = Device::open(0, 0).unwrap();
    assert_eq!(device.analog_in().get_frequency().unwrap(), 1_250_000.0);
}

#[test]
fn get_frequency_info_returns_min_then_max() {
    let device = Device::open(0, 0).unwrap();
    assert_eq!(
        device.analog_in().get_frequency_info().unwrap(),
        (1.0e-3, 100.0e6)
    );
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
