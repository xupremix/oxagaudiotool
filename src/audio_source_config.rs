use std::fmt::Error;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use crate::audio_tool::{OxAgAudioTool, OxAgAudioToolError};

pub struct OxAgSoundConfig {
    pub(crate) path: String,
    pub(crate) settings: StaticSoundSettings
}

impl OxAgSoundConfig {
    pub fn new(path: String) -> OxAgSoundConfig {
        OxAgSoundConfig {
            path,
            settings: StaticSoundSettings::default(),
        }
    }

    pub fn new_with_settings(path: String, settings: StaticSoundSettings) -> OxAgSoundConfig {
        OxAgSoundConfig {
            path,
            settings,
        }
    }

    pub(crate) fn to_sound_data(&self) -> Result<StaticSoundData, OxAgAudioToolError> {
        StaticSoundData::from_file(&self.path, self.settings).map_err(|e| {
            OxAgAudioToolError::FileError(e)
        })
    }
}