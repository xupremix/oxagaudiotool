use crate::error::error::OxAgAudioToolError;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::sound::{EndPosition, PlaybackPosition, Region};

pub struct OxAgSoundConfig {
    pub(crate) path: String,
    pub(crate) settings: StaticSoundSettings,
}

impl OxAgSoundConfig {
    pub fn new(path: String) -> OxAgSoundConfig {
        OxAgSoundConfig {
            path,
            settings: StaticSoundSettings::default(),
        }
    }

    pub fn new_with_settings(path: &str, settings: StaticSoundSettings) -> OxAgSoundConfig {
        OxAgSoundConfig { path: path.to_string(), settings }
    }

    pub fn new_looped(path: &str) -> OxAgSoundConfig {
        let mut settings = StaticSoundSettings::new();
        settings.loop_region(Region {
            start: PlaybackPosition::Samples(0),
            end: EndPosition::EndOfAudio,
        });

        OxAgSoundConfig { path: path.to_string(), settings }
    }

    pub(crate) fn to_sound_data(&self) -> Result<StaticSoundData, OxAgAudioToolError> {
        Ok(StaticSoundData::from_file(&self.path, self.settings)?)
    }
}
