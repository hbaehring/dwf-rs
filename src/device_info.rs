use crate::dwf;
use crate::{check_call, Result};

use std::ffi::{c_char, c_int, CStr};
use std::mem;
pub trait DeviceId {
    /// The enumeration index used by the `FDwfEnum*` queries.
    fn get_device_id(&self) -> Result<c_int>;
}

pub trait DeviceInfo: DeviceId {
    fn get_device_name(&self) -> Result<String> {
        let mut name = [0 as c_char; 32];
        unsafe {
            check_call(dwf::FDwfEnumDeviceName(
                self.get_device_id()?,
                name.as_mut_ptr(),
            ))?;

            Ok(CStr::from_ptr(name.as_ptr()).to_str().unwrap().to_owned())
        }
    }

    fn get_device_serial(&self) -> Result<String> {
        let mut serial = [0 as c_char; 32];
        unsafe {
            check_call(dwf::FDwfEnumSN(self.get_device_id()?, serial.as_mut_ptr()))?;

            let sn = CStr::from_ptr(serial.as_ptr()).to_str().unwrap().to_owned();

            // Remove the SN: prefix
            let sn = sn.replace("SN:", "");

            Ok(sn)
        }
    }

    fn device_in_use(&self) -> Result<bool> {
        unsafe {
            let mut in_use = mem::MaybeUninit::<dwf::BOOL>::uninit();
            check_call(dwf::FDwfEnumDeviceIsOpened(
                self.get_device_id()?,
                in_use.as_mut_ptr(),
            ))?;

            Ok(dwf::from_c_bool(in_use.assume_init()))
        }
    }
}

/// Selects the devices `enumerate_devices` lists.
#[derive(Eq, PartialEq, Debug)]
pub enum EnumFilter {
    All,
    Device(DeviceType),
    Types(EnumTypes),
}

/// Connection types for `EnumFilter::Types`; any combination can be enabled.
#[derive(Eq, PartialEq, Debug, Default)]
pub struct EnumTypes {
    pub usb: bool,
    pub network: bool,
    pub axi: bool,
    pub remote: bool,
    pub audio: bool,
    pub demo: bool,
}

impl EnumTypes {
    pub(crate) fn to_enumfilter(&self) -> dwf::ENUMFILTER {
        [
            (self.usb, dwf::enumfilterUSB),
            (self.network, dwf::enumfilterNetwork),
            (self.axi, dwf::enumfilterAXI),
            (self.remote, dwf::enumfilterRemote),
            (self.audio, dwf::enumfilterAudio),
            (self.demo, dwf::enumfilterDemo),
        ]
        .iter()
        .filter(|(enabled, _)| *enabled)
        .fold(dwf::enumfilterType, |filter, (_, flag)| filter | flag)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum DeviceType {
    EExplorer,
    Discovery,
    Discovery2,
    DDiscovery,
    ADP3X50,
    Eclypse,
    ADP5250,
    DPS3340,
    Discovery3,
    ADP5470,
    ADP5490,
    ADP2230,
    ADSMax,
}

impl DeviceType {
    pub(crate) fn to_devid(&self) -> dwf::DEVID {
        match self {
            DeviceType::EExplorer => dwf::devidEExplorer,
            DeviceType::Discovery => dwf::devidDiscovery,
            DeviceType::Discovery2 => dwf::devidDiscovery2,
            DeviceType::DDiscovery => dwf::devidDDiscovery,
            DeviceType::ADP3X50 => dwf::devidADP3X50,
            DeviceType::Eclypse => dwf::devidEclypse,
            DeviceType::ADP5250 => dwf::devidADP5250,
            DeviceType::DPS3340 => dwf::devidDPS3340,
            DeviceType::Discovery3 => dwf::devidDiscovery3,
            DeviceType::ADP5470 => dwf::devidADP5470,
            DeviceType::ADP5490 => dwf::devidADP5490,
            DeviceType::ADP2230 => dwf::devidADP2230,
            DeviceType::ADSMax => dwf::devidADSMax,
        }
    }
}
