use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RtAudioError {
    pub type_: RtAudioErrorType,
    pub msg: Option<String>,
}

#[repr(i32)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RtAudioErrorType {
    /// A non-critical error.
    Warning = rtaudio_sys::RTAUDIO_ERROR_WARNING,
    /// An unspecified error type.
    Unkown = rtaudio_sys::RTAUDIO_ERROR_UNKNOWN,
    /// No devices found on system.
    NoDevicesFound = rtaudio_sys::RTAUDIO_ERROR_NO_DEVICES_FOUND,
    /// An invalid device ID was specified.
    InvalidDevice = rtaudio_sys::RTAUDIO_ERROR_INVALID_DEVICE,
    /// A device in use was disconnected.
    DeviceDisconnect = rtaudio_sys::RTAUDIO_ERROR_DEVICE_DISCONNECT,
    /// An error occurred during memory allocation.
    MemoryError = rtaudio_sys::RTAUDIO_ERROR_MEMORY_ERROR,
    /// An invalid parameter was specified to a function.
    InvalidParamter = rtaudio_sys::RTAUDIO_ERROR_INVALID_PARAMETER,
    /// The function was called incorrectly.
    InvalidUse = rtaudio_sys::RTAUDIO_ERROR_INVALID_USE,
    /// A system driver error occurred.
    DriverError = rtaudio_sys::RTAUDIO_ERROR_DRIVER_ERROR,
    /// A system error occurred.
    SystemError = rtaudio_sys::RTAUDIO_ERROR_SYSTEM_ERROR,
    /// A thread error occurred.
    ThreadError = rtaudio_sys::RTAUDIO_ERROR_THREAD_ERROR,
}

impl RtAudioErrorType {
    pub fn from_raw(e: rtaudio_sys::rtaudio_error_t) -> Option<RtAudioErrorType> {
        match e {
            rtaudio_sys::RTAUDIO_ERROR_NONE => None,
            rtaudio_sys::RTAUDIO_ERROR_WARNING => Some(RtAudioErrorType::Warning),
            rtaudio_sys::RTAUDIO_ERROR_UNKNOWN => Some(RtAudioErrorType::Unkown),
            rtaudio_sys::RTAUDIO_ERROR_NO_DEVICES_FOUND => Some(RtAudioErrorType::NoDevicesFound),
            rtaudio_sys::RTAUDIO_ERROR_INVALID_DEVICE => Some(RtAudioErrorType::InvalidDevice),
            rtaudio_sys::RTAUDIO_ERROR_DEVICE_DISCONNECT => {
                Some(RtAudioErrorType::DeviceDisconnect)
            }
            rtaudio_sys::RTAUDIO_ERROR_MEMORY_ERROR => Some(RtAudioErrorType::MemoryError),
            rtaudio_sys::RTAUDIO_ERROR_INVALID_PARAMETER => Some(RtAudioErrorType::InvalidParamter),
            rtaudio_sys::RTAUDIO_ERROR_INVALID_USE => Some(RtAudioErrorType::InvalidUse),
            rtaudio_sys::RTAUDIO_ERROR_DRIVER_ERROR => Some(RtAudioErrorType::DriverError),
            rtaudio_sys::RTAUDIO_ERROR_SYSTEM_ERROR => Some(RtAudioErrorType::SystemError),
            rtaudio_sys::RTAUDIO_ERROR_THREAD_ERROR => Some(RtAudioErrorType::ThreadError),
            _ => Some(RtAudioErrorType::Unkown),
        }
    }
}

impl Error for RtAudioError {}

impl fmt::Display for RtAudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type_ {
            RtAudioErrorType::Warning => write!(f, "RtAudio: warning"),
            RtAudioErrorType::Unkown => write!(f, "RtAudio: unspecified error"),
            RtAudioErrorType::NoDevicesFound => write!(f, "RtAudio: no devices found on system"),
            RtAudioErrorType::InvalidDevice => {
                write!(f, "RtAudio: an invalid device ID was specified")
            }
            RtAudioErrorType::DeviceDisconnect => {
                write!(f, "RtAudio: a device in use was disconnected")
            }
            RtAudioErrorType::MemoryError => {
                write!(f, "RtAudio: an error occurred during memory allocation")
            }
            RtAudioErrorType::InvalidParamter => write!(
                f,
                "RtAudio: an invalid parameter was specified to a function"
            ),
            RtAudioErrorType::InvalidUse => {
                write!(f, "RtAudio: the function was called incorrectly")
            }
            RtAudioErrorType::DriverError => write!(f, "RtAudio: a system driver error occurred"),
            RtAudioErrorType::SystemError => write!(f, "RtAudio: a system error occurred"),
            RtAudioErrorType::ThreadError => write!(f, "RtAudio: a thread error occurred"),
        }?;

        if let Some(msg) = &self.msg {
            write!(f, " | {}", msg)?;
        }

        Ok(())
    }
}
