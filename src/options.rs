use rtaudio_sys::MAX_NAME_LENGTH;
use std::os::raw::{c_int, c_uint};

use crate::error::{RtAudioError, RtAudioErrorType};
use crate::{DeviceID, StreamFlags};

/// The default number of frames that can appear in a single process call.
pub const DEFAULT_BUFFER_FRAMES: u32 = 1024;

/// Used for specifying the parameters of a device when opening a
/// stream.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceParams {
    /// The ID of the device to use.
    ///
    /// If this is `None`, then the default device will be used.
    #[cfg_attr(feature = "serde", serde(default = "default_device_id"))]
    pub device_id: Option<DeviceID>,
    /// The number of channels in the device to use (default = 2).
    #[cfg_attr(feature = "serde", serde(default = "default_num_channels"))]
    pub num_channels: u32,
    /// The first channel index on the device to use (default = 0).
    #[cfg_attr(feature = "serde", serde(default))]
    pub first_channel: u32,
    /// If `true`, then fallback to the default device if the device
    /// with the given ID is not found. Otherwise, don't start the stream
    /// and return an error.
    ///
    /// By default this is set to `true`.
    #[cfg_attr(feature = "serde", serde(default = "default_fallback"))]
    pub fallback: bool,
    /// If `true`, then fallback to no input/output if no default device
    /// could be found. Otherwise, don't start the stream and return an
    /// error.
    ///
    /// By default this is set to `true`.
    #[cfg_attr(feature = "serde", serde(default = "default_fallback"))]
    pub no_device_fallback: bool,
}

impl Default for DeviceParams {
    fn default() -> Self {
        Self {
            device_id: default_device_id(),
            num_channels: default_num_channels(),
            first_channel: 0,
            fallback: default_fallback(),
            no_device_fallback: default_fallback(),
        }
    }
}

const fn default_device_id() -> Option<DeviceID> {
    None
}

const fn default_num_channels() -> u32 {
    2
}

const fn default_fallback() -> bool {
    true
}

/// Additional options for opening a stream.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StreamOptions {
    /// The bit flag parameters for this stream.
    ///
    /// By default, no flags are set.
    #[cfg_attr(feature = "serde", serde(default))]
    pub flags: StreamFlags,

    /// Used to control stream latency in the Windows DirectSound, Linux OSS, and Linux Alsa APIs only.
    /// A value of two is usually the smallest allowed. Larger numbers can potentially result in more
    /// robust stream performance, though likely at the cost of stream latency.
    ///
    /// The actual value used when the stream is ran may be different.
    ///
    /// The default value is `4`.
    #[cfg_attr(feature = "serde", serde(default = "default_num_buffers"))]
    pub num_buffers: u32,

    /// Scheduling priority of callback thread (only used with flag `StreamFlags::SCHEDULE_REALTIME`).
    ///
    /// Use a value of `-1` for the default priority.
    ///
    /// The default value is `-1`.
    #[cfg_attr(feature = "serde", serde(default = "default_priority"))]
    pub priority: i32,

    /// The name of the stream (currently used only in Jack).
    ///
    /// The size of the name cannot exceed 511 bytes.
    #[cfg_attr(feature = "serde", serde(default))]
    pub name: String,
}

impl StreamOptions {
    pub fn to_raw(&self) -> Result<rtaudio_sys::rtaudio_stream_options_t, RtAudioError> {
        let name = crate::ffi_utils::str_to_c_str_array::<{ MAX_NAME_LENGTH }>(&self.name)
            .map_err(|e| RtAudioError {
                type_: RtAudioErrorType::InvalidParamter,
                msg: Some(format!("Stream name is invalid: {}", e)),
            })?;

        Ok(rtaudio_sys::rtaudio_stream_options_t {
            flags: self.flags.bits(),
            num_buffers: self.num_buffers as c_uint,
            priority: self.priority as c_int,
            name,
        })
    }
}

impl Default for StreamOptions {
    fn default() -> Self {
        Self {
            flags: StreamFlags::empty(),
            num_buffers: default_num_buffers(),
            priority: default_priority(),
            name: String::from("RtAudio-rs Client"),
        }
    }
}

const fn default_num_buffers() -> u32 {
    4
}

const fn default_priority() -> i32 {
    -1
}
