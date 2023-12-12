use crate::error::error::OxAgAudioToolError;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::sound::{EndPosition, PlaybackPosition, Region};
use kira::tween::Value;
use kira::Volume;

#[derive(Debug)]
pub struct OxAgSoundConfig {
    pub(crate) path: String,
    pub(crate) settings: StaticSoundSettings,
}

impl OxAgSoundConfig {
    pub fn new(path: &str) -> OxAgSoundConfig {
        OxAgSoundConfig {
            path: path.to_string(),
            settings: StaticSoundSettings::default(),
        }
    }

    pub fn new_with_settings(path: &str, settings: StaticSoundSettings) -> OxAgSoundConfig {
        OxAgSoundConfig { path: path.to_string(), settings }
    }

    pub fn new_looped(path: &str) -> OxAgSoundConfig {
        let settings = StaticSoundSettings::new()
            .loop_region(Region {
                start: PlaybackPosition::Samples(0),
                end: EndPosition::EndOfAudio,
            });

        OxAgSoundConfig { path: path.to_string(), settings }
    }

    pub fn new_with_volume(path: &str, amplitude: f64) -> OxAgSoundConfig {
        let settings = StaticSoundSettings::new()
            .volume(Value::Fixed(Volume::Amplitude(amplitude)));

        OxAgSoundConfig { path: path.to_string(), settings }
    }

    pub fn new_looped_with_volume(path: &str, amplitude: f64) -> OxAgSoundConfig {
        let settings = StaticSoundSettings::new()
            .loop_region(Region {
                start: PlaybackPosition::Samples(0),
                end: EndPosition::EndOfAudio,
            })
            .volume(Value::Fixed(Volume::Amplitude(amplitude)));

        OxAgSoundConfig { path: path.to_string(), settings }
    }

    pub fn looped(&mut self) {
        self.settings = self.settings.loop_region(Region {
            start: PlaybackPosition::Samples(0),
            end: EndPosition::EndOfAudio,
        });
    }

    pub fn volume(&mut self, amplitude: f64) {
        self.settings = self.settings.volume(Value::Fixed(Volume::Amplitude(amplitude)));
    }

    pub(crate) fn to_sound_data(&self) -> Result<StaticSoundData, OxAgAudioToolError> {
        Ok(StaticSoundData::from_file(&self.path, self.settings)?)
    }
}