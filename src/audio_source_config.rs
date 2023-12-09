use std::fmt::Error;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};

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

    pub(crate) fn to_sound_data(&self) -> Result<StaticSoundData, Error> {
        StaticSoundData::from_file(&self.path, self.settings).map_err(|e| {
            Default::default()
        })
    }
}