use dwf;
use dwf::device_info::{DeviceInfo, EnumFilter};

fn main() {
    let num_devices = dwf::enumerate_devices(EnumFilter::All).unwrap();
    println!("Device count: {}", num_devices);

    for device_idx in 0..num_devices {
        let device = dwf::device::Device::open(device_idx, 0).unwrap();
        device.reset().unwrap();
        println!(
            "Device #{} name: {}",
            device_idx + 1,
            device.get_device_name().unwrap()
        );
        println!(
            "Device #{} serial: {}",
            device_idx + 1,
            device.get_device_serial().unwrap()
        );
        println!(
            "Device #{} in use: {}",
            device_idx + 1,
            device.device_in_use().unwrap()
        );
    }
}
