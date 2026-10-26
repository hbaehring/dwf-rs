//! Replaces the libdwf functions used here with local definitions, so the
//! wrappers can be tested without a device attached.
use dwf::device_info::{DeviceType, EnumFilter, EnumTypes};
use std::ffi::c_int;

// Reports the filter it was given as the device count.
#[no_mangle]
unsafe extern "C" fn FDwfEnum(enumfilter: c_int, pc_device: *mut c_int) -> c_int {
    *pc_device = enumfilter;
    1
}

#[test]
fn enumerate_devices_maps_filter() {
    assert_eq!(dwf::enumerate_devices(EnumFilter::All), Ok(0));
    assert_eq!(
        dwf::enumerate_devices(EnumFilter::Device(DeviceType::Discovery3)),
        Ok(10)
    );
    assert_eq!(
        dwf::enumerate_devices(EnumFilter::Types(EnumTypes {
            usb: true,
            network: true,
            ..Default::default()
        })),
        Ok(0x8000003)
    );
    assert_eq!(
        dwf::enumerate_devices(EnumFilter::Types(EnumTypes {
            remote: true,
            audio: true,
            demo: true,
            axi: true,
            ..Default::default()
        })),
        Ok(0xF000004)
    );
}
