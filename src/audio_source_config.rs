use kira::sound::static_sound::StaticSoundSettings;

pub struct OxAgAudioSourceConfig {
    pub(crate) path: String,
    pub(crate) settings: StaticSoundSettings
}

impl OxAgAudioSourceConfig {
    pub fn new(path: String) -> OxAgAudioSourceConfig {
        OxAgAudioSourceConfig {
            path,
            settings: StaticSoundSettings::default(),
        }
    }

    pub fn new_with_settings(path: String, settings: StaticSoundSettings) -> OxAgAudioSourceConfig {
        OxAgAudioSourceConfig {
            path,
            settings,
        }
    }
}