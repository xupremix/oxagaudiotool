pub mod audio_source_config;
pub mod audio_tool {
    use std::fmt::Error;
    use std::collections::HashMap;
    use kira::manager::{AudioManager, AudioManagerSettings};
    use kira::manager::backend::DefaultBackend;
    use kira::manager::error::PlaySoundError;
    use kira::sound::SoundData;
    use kira::sound::static_sound::StaticSoundData;
    use robotics_lib::event::events::Event;
    use robotics_lib::world::environmental_conditions::WeatherType;
    use robotics_lib::world::tile::TileType;
    use crate::audio_source_config::OxAgSoundConfig;

    pub struct OxAgAudioTool {
        event_to_sound_data: HashMap<Event, StaticSoundData>,
        tile_type_to_sound_data: HashMap<TileType, StaticSoundData>,
        weather_type_to_sound_data: HashMap<WeatherType, StaticSoundData>,
        audio_manager: AudioManager,
        current_tile_type: Option<TileType>,
        current_weather_type: Option<WeatherType>
    }

    enum OxAgAudioToolError {
        // TODO
    }

    impl OxAgAudioTool {
        pub fn new(
            event_to_sound_config: HashMap<Event, OxAgSoundConfig>,
            tile_type_to_sound_config: HashMap<TileType, OxAgSoundConfig>,
            weather_type_to_sound_config: HashMap<WeatherType, OxAgSoundConfig>
        ) -> Result<OxAgAudioTool, Error> {
            let audio_manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                .map_err(|e| {
                    Error
                })?;

            let mut event_to_sound_data: HashMap<Event, StaticSoundData> = HashMap::new();
            let mut tile_type_to_sound_data: HashMap<TileType, StaticSoundData> = HashMap::new();
            let mut weather_type_to_sound_data: HashMap<WeatherType, StaticSoundData> = HashMap::new();

            for (event, config) in event_to_sound_config.iter() {
                event_to_sound_data.insert(event.clone(), config.to_sound_data()?);
            }

            for (tile_type, config) in tile_type_to_sound_config.iter() {
                tile_type_to_sound_data.insert(tile_type.clone(), config.to_sound_data()?);
            }

            for (weather_type, config) in weather_type_to_sound_config.iter() {
                weather_type_to_sound_data.insert(weather_type.clone(), config.to_sound_data()?);
            }

            Ok(OxAgAudioTool {
                event_to_sound_data,
                tile_type_to_sound_data,
                weather_type_to_sound_data,
                audio_manager,
                current_tile_type: None,
                current_weather_type: None,
            })
        }

        // TODO: Default audio sources

        pub fn play_audio_based_on_event(&mut self, event: &Event) -> Result<(), Error> {
            let event_sound_data = self.event_to_sound_data.get(event).cloned();

            if let Some(data) = event_sound_data {
                self.audio_manager.play(data).map_err(|_| Error)?;
            }


            let tile_or_weather_sound_data: Option<&StaticSoundData> = match event {
                Event::Moved(tile, _) => {
                    let tile_type = tile.tile_type;

                    if self.current_tile_type.is_none() || tile_type != self.current_tile_type.unwrap() {
                        self.current_tile_type = Some(tile_type);
                        self.tile_type_to_sound_data.get(&tile_type)
                    } else {
                        None
                    }
                }
                Event::TimeChanged(environmental_conditions) => {
                    let weather_type = environmental_conditions.get_weather_condition();

                    if self.current_weather_type.is_none() || weather_type != self.current_weather_type.unwrap() {
                        self.current_weather_type = Some(weather_type);
                        self.weather_type_to_sound_data.get(&weather_type)
                    } else {
                        None
                    }
                }
                _ => None
            };

            if let Some(data) = tile_or_weather_sound_data {
                self.audio_manager.play(data.clone()).map_err(|_| Error)?;
            }

            Ok(())
        }

        pub fn play_audio(&mut self, sound_config: &OxAgSoundConfig) -> Result<(), PlaySoundError<StaticSoundData>> {
            let sound_data = sound_config.to_sound_data().map_err(|_| { todo!()})?;

            self.audio_manager.play(sound_data); // TODO: ereoreoreoroeroeororoeorre

            Ok(())
        }
    }
}