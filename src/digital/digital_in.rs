use crate::device::Device;
use crate::dwf;
use crate::dwf::to_c_bool;
use crate::{check_call, Result};
use std::mem;
use std::os::raw::c_void;

#[derive(Clone)]
pub struct DigitalIn<'a> {
    device: &'a Device,
}

impl<'a> DigitalIn<'a> {
    pub fn new(device: &'a Device) -> DigitalIn<'a> {
        Self { device }
    }

    pub fn start(&self) -> Result<()> {
        unsafe {
            check_call(dwf::FDwfDigitalInConfigure(
                self.device.get_handle(),
                to_c_bool(false),
                to_c_bool(true),
            ))?;
        }
        Ok(())
    }

    pub fn reconfigure_start(&self) -> Result<()> {
        unsafe {
            check_call(dwf::FDwfDigitalInConfigure(
                self.device.get_handle(),
                to_c_bool(true),
                to_c_bool(true),
            ))?;
        }
        Ok(())
    }

    pub fn set_acquisition_mode(&self, mode: AcquisitionMode) -> Result<()> {
        let mode = match mode {
            AcquisitionMode::Single => dwf::acqmodeSingle,
            AcquisitionMode::ScanShift => dwf::acqmodeScanShift,
            AcquisitionMode::ScanScreen => dwf::acqmodeScanScreen,
            AcquisitionMode::Record => dwf::acqmodeRecord,
        };
        unsafe {
            check_call(dwf::FDwfDigitalInAcquisitionModeSet(
                self.device.get_handle(),
                mode,
            ))?;
        }
        Ok(())
    }

    pub fn set_buffer_size(&self, size: i32) -> Result<()> {
        unsafe {
            check_call(dwf::FDwfDigitalInBufferSizeSet(
                self.device.get_handle(),
                size,
            ))?;
        }
        Ok(())
    }

    pub fn get_buffer_size(&self) -> Result<i32> {
        unsafe {
            let mut size = mem::MaybeUninit::uninit();
            check_call(dwf::FDwfDigitalInBufferSizeGet(
                self.device.get_handle(),
                size.as_mut_ptr(),
            ))?;
            Ok(size.assume_init())
        }
    }

    pub fn get_buffer_size_info(&self) -> Result<i32> {
        unsafe {
            let mut max_size = mem::MaybeUninit::uninit();
            check_call(dwf::FDwfDigitalInBufferSizeInfo(
                self.device.get_handle(),
                max_size.as_mut_ptr(),
            ))?;
            Ok(max_size.assume_init())
        }
    }

    pub fn get_status(&self) -> Result<DigitalAcquisitionStatus> {
        unsafe {
            let mut state = mem::MaybeUninit::uninit();
            check_call(dwf::FDwfDigitalInStatus(
                self.device.get_handle(),
                to_c_bool(true),
                state.as_mut_ptr(),
            ))?;
            let state = state.assume_init();
            Ok(match state {
                dwf::DwfStateReady => DigitalAcquisitionStatus::Ready,
                dwf::DwfStateConfig => DigitalAcquisitionStatus::Config,
                dwf::DwfStatePrefill => DigitalAcquisitionStatus::Prefill,
                dwf::DwfStateArmed => DigitalAcquisitionStatus::Armed,
                dwf::DwfStateWait => DigitalAcquisitionStatus::Waiting,
                dwf::DwfStateRunning => DigitalAcquisitionStatus::Running,
                dwf::DwfStateDone => DigitalAcquisitionStatus::Done,
                // TODO: change to error code
                _ => panic!("Unknown state: {}", state),
            })
        }
    }

    pub fn get_record_status(&self) -> Result<(i32, i32, i32)> {
        unsafe {
            let mut available = mem::MaybeUninit::uninit();
            let mut lost = mem::MaybeUninit::uninit();
            let mut corrupted = mem::MaybeUninit::uninit();
            check_call(dwf::FDwfDigitalInStatusRecord(
                self.device.get_handle(),
                available.as_mut_ptr(),
                lost.as_mut_ptr(),
                corrupted.as_mut_ptr(),
            ))?;
            Ok((
                available.assume_init(),
                lost.assume_init(),
                corrupted.assume_init(),
            ))
        }
    }

    pub fn set_trigger_source(&self, trigger_source: TriggerSource) -> Result<()> {
        let trigger_source = match trigger_source {
            TriggerSource::None => dwf::trigsrcNone,
            TriggerSource::PC => dwf::trigsrcPC,
            TriggerSource::AnalogIn => dwf::trigsrcAnalogIn,
            TriggerSource::DigitalIn => dwf::trigsrcDigitalIn,
            TriggerSource::DigitalOut => dwf::trigsrcDigitalOut,
            TriggerSource::AnalogOut1 => dwf::trigsrcAnalogOut1,
            TriggerSource::AnalogOut2 => dwf::trigsrcAnalogOut2,
            TriggerSource::AnalogOut3 => dwf::trigsrcAnalogOut3,
            TriggerSource::AnalogOut4 => dwf::trigsrcAnalogOut4,
            TriggerSource::External1 => dwf::trigsrcExternal1,
            TriggerSource::External2 => dwf::trigsrcExternal2,
            TriggerSource::External3 => dwf::trigsrcExternal3,
            TriggerSource::External4 => dwf::trigsrcExternal4,
            TriggerSource::High => dwf::trigsrcHigh,
            TriggerSource::Low => dwf::trigsrcLow,
            TriggerSource::Clock => dwf::trigsrcClock,
        };
        unsafe {
            check_call(dwf::FDwfDigitalInTriggerSourceSet(
                self.device.get_handle(),
                trigger_source,
            ))?;
        }
        Ok(())
    }

    pub fn set_trigger_slope(&self, slope: TriggerSlope) -> Result<()> {
        let slope = match slope {
            TriggerSlope::Rise => dwf::DwfTriggerSlopeRise,
            TriggerSlope::Fall => dwf::DwfTriggerSlopeFall,
            TriggerSlope::Either => dwf::DwfTriggerSlopeEither,
        };
        unsafe {
            check_call(dwf::FDwfDigitalInTriggerSlopeSet(
                self.device.get_handle(),
                slope,
            ))?
        }
        Ok(())
    }

    pub fn set_trigger_position(&self, position: u32) -> Result<()> {
        unsafe {
            check_call(dwf::FDwfDigitalInTriggerPositionSet(
                self.device.get_handle(),
                position,
            ))?;
        }
        Ok(())
    }

    pub fn get_clock_info(&self) -> Result<f64> {
        unsafe {
            let mut freq = mem::MaybeUninit::uninit();
            check_call(dwf::FDwfDigitalInInternalClockInfo(
                self.device.get_handle(),
                freq.as_mut_ptr(),
            ))?;
            Ok(freq.assume_init())
        }
    }

    pub fn set_clock_divider(&self, divider: u32) -> Result<()> {
        unsafe {
            check_call(dwf::FDwfDigitalInDividerSet(
                self.device.get_handle(),
                divider,
            ))?;
            Ok(())
        }
    }

    pub fn set_sensible_sample(&self, sensible_sample: u32) -> Result<()> {
        unsafe {
            check_call(dwf::FDwfDigitalInSampleSensibleSet(
                self.device.get_handle(),
                sensible_sample,
            ))?;
            Ok(())
        }
    }

    pub fn set_sample_format(&self, num_bits: i32) -> Result<()> {
        unsafe {
            check_call(dwf::FDwfDigitalInSampleFormatSet(
                self.device.get_handle(),
                num_bits,
            ))?;
            Ok(())
        }
    }

    /// Appends `available` samples to `dest`. The library writes raw bytes, so
    /// only integer sample types are accepted:
    ///
    /// ```compile_fail,E0277
    /// # let device = dwf::device::Device::default();
    /// let mut samples: Vec<bool> = vec![];
    /// device.digital_in().fetch_samples(&mut samples, 4);
    /// ```
    pub fn fetch_samples<B: DigitalSample>(&self, dest: &mut Vec<B>, available: i32) -> Result<()> {
        unsafe {
            let original_len = dest.len();
            dest.reserve(available as usize);
            let remaining = dest.spare_capacity_mut();
            check_call(dwf::FDwfDigitalInStatusData(
                self.device.get_handle(),
                remaining.as_mut_ptr() as *mut c_void,
                available * mem::size_of::<B>() as i32,
            ))?;
            dest.set_len(original_len + available as usize);
        }
        Ok(())
    }
}

mod sealed {
    pub trait Sealed {}
}

/// Sample types `DigitalIn::fetch_samples` can fill: integers, for which every
/// bit pattern is a valid value. Match the width to `set_sample_format`.
pub trait DigitalSample: sealed::Sealed {}

macro_rules! digital_sample {
    ($($t:ty),*) => {
        $(
            impl sealed::Sealed for $t {}
            impl DigitalSample for $t {}
        )*
    };
}

digital_sample!(u8, u16, u32, i8, i16, i32);

#[derive(Eq, PartialEq, Debug)]
pub enum AcquisitionMode {
    Single,
    ScanShift,
    ScanScreen,
    Record,
}

#[derive(Eq, PartialEq, Debug)]
pub enum DigitalAcquisitionStatus {
    Ready,
    Config,
    Prefill,
    Armed,
    Waiting,
    Running,
    Done,
}

#[derive(Eq, PartialEq, Debug)]
pub enum TriggerSource {
    None,
    PC,
    AnalogIn,
    DigitalIn,
    DigitalOut,
    AnalogOut1,
    AnalogOut2,
    AnalogOut3,
    AnalogOut4,
    External1,
    External2,
    External3,
    External4,
    High,
    Low,
    Clock,
}

#[derive(Eq, PartialEq, Debug)]
pub enum TriggerType {
    Edge,
    Pulse,
    Transition,
    Window,
}

#[derive(Eq, PartialEq, Debug)]
pub enum TriggerSlope {
    Rise,
    Fall,
    Either,
}
