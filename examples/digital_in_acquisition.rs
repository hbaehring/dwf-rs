use dwf;
use dwf::digital::digital_in::DigitalAcquisitionStatus;

const FREQ: f64 = 1000000.0f64;
const NUM_SAMPLES: usize = 4000;
fn main() {
    println!("DWF Version: {}", dwf::get_version().unwrap());

    println!("Opening first device");
    let device = dwf::device::Device::default();
    let digital_in = device.digital_in();

    println!(
        "Device buffer size: {}",
        digital_in.get_buffer_size().unwrap()
    );

    let clock = digital_in.get_clock_info().unwrap();
    println!("Device clock: {} Hz", clock);
    // Sample rate = system frequency / divider
    digital_in.set_clock_divider((clock / FREQ) as u32).unwrap();
    digital_in.set_buffer_size(NUM_SAMPLES as i32).unwrap();
    // The device may cap the buffer below the requested size
    let num_samples = digital_in.get_buffer_size().unwrap();
    println!("Acquiring {} samples", num_samples);
    digital_in.set_sample_format(8).unwrap();

    println!("Starting oscilloscope");
    digital_in.start().unwrap();

    while digital_in.get_status().unwrap() != DigitalAcquisitionStatus::Done {}
    println!("Acquisition done");

    let mut samples: Vec<u8> = vec![];
    digital_in.fetch_samples(&mut samples, num_samples).unwrap();
    for sample in samples {
        println!("{:#010b}", sample);
    }
}
