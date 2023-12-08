use std::collections::HashMap;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::manager::backend::DefaultBackend;
use kira::sound::static_sound::StaticSoundData;
use robotics_lib::event::events::Event;
use robotics_lib::runner::Runnable;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::TileType;
use crate::audio_source_config::OxAgAudioSourceConfig;

pub struct OxAgAudioTool {
    audio_manager: AudioManager,
    audio_sources: HashMap<Event, StaticSoundData>,
    current_tile_type: Option<TileType>,
    current_weather_type: Option<WeatherType>
}

impl OxAgAudioTool {
    pub fn new(audio_sources: HashMap<Event, OxAgAudioSourceConfig>) -> OxAgAudioTool {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
        let audio_sources = audio_sources.iter().map(|(e, config)| {
            StaticSoundData::from_file(config.path.as_ref(), config.settings)
        }).collect();

        OxAgAudioTool {
            audio_manager: manager,
            audio_sources,
            current_tile_type: None,
            current_weather_type: None
        }
    }

    pub fn play_audio_based_on_event(&mut self, event: &Event) {
        let audio_source = self.audio_sources.get(event)?;

        let should_play = match event {
            Event::Moved(tile, _) => {
                let tile_type = tile.tile_type;

                if self.current_tile_type.is_none() || tile_type != self.current_tile_type.unwrap() {
                    self.current_tile_type = Some(tile_type);
                    true
                } else {
                    false
                }
            }
            Event::TimeChanged(environmental_conditions) => {
                let weather_type = environmental_conditions.get_weather_condition();

                if self.current_weather_type.is_none() || weather_type != self.current_weather_type.unwrap() {
                    self.current_weather_type = Some(weather_type);
                    true
                } else {
                    false
                }
            }
            _ => true
        };

        if (should_play) {
            self.audio_manager.play(audio_source)?;
        }
    }
}