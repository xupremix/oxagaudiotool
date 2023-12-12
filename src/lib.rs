pub mod sound_config;
pub mod error;
mod test;
mod util;

pub mod audio_tool {
    use crate::sound_config::OxAgSoundConfig;
    use crate::error::error::OxAgAudioToolError;
    use crate::util::event_key;
    use kira::manager::backend::DefaultBackend;
    use kira::manager::{AudioManager, AudioManagerSettings};
    use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle};
    use kira::tween::Tween;
    use robotics_lib::event::events::Event;
    use robotics_lib::world::environmental_conditions::WeatherType;
    use robotics_lib::world::tile::TileType;
    use std::collections::HashMap;

    pub struct OxAgAudioTool {
        event_to_sound_data: HashMap<String, StaticSoundData>,
        tile_type_to_sound_data: HashMap<TileType, StaticSoundData>,
        weather_type_to_sound_data: HashMap<WeatherType, StaticSoundData>,
        audio_manager: AudioManager,
        current_tile_sound: Option<(TileType, StaticSoundHandle)>,
        current_weather_sound: Option<(WeatherType, StaticSoundHandle)>,
    }

    impl OxAgAudioTool {
        pub fn new(
            event_to_sound_config: HashMap<Event, OxAgSoundConfig>,
            tile_type_to_sound_config: HashMap<TileType, OxAgSoundConfig>,
            weather_type_to_sound_config: HashMap<WeatherType, OxAgSoundConfig>,
        ) -> Result<OxAgAudioTool, OxAgAudioToolError> {
            let audio_manager =
                AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;

            let mut event_to_sound_data: HashMap<String, StaticSoundData> = HashMap::new();
            let mut tile_type_to_sound_data: HashMap<TileType, StaticSoundData> = HashMap::new();
            let mut weather_type_to_sound_data: HashMap<WeatherType, StaticSoundData> =
                HashMap::new();

            for (event, config) in event_to_sound_config.iter() {
                event_to_sound_data.insert(event_key(event).to_string(), config.to_sound_data()?);
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
                current_tile_sound: None,
                current_weather_sound: None,
            })
        }

        pub fn play_audio_based_on_event(
            &mut self,
            event: &Event,
        ) -> Result<(), OxAgAudioToolError> {
            let event_sound_data = self.event_to_sound_data.get(event_key(event)).cloned();

            if let Some(data) = event_sound_data {
                self.audio_manager.play(data)?;
            }

            match event {
                Event::Moved(tile, _) => {
                    let tile_type = tile.tile_type;

                    if self.current_tile_sound.is_none() || tile_type != self.current_tile_sound.as_ref().unwrap().0 {
                        if let Some(previous_tile_sound) = &mut self.current_tile_sound {
                            let _ = previous_tile_sound.1.stop(Tween::default());
                        }

                        let sound_data = self.tile_type_to_sound_data.get(&tile_type);

                        if let Some(sound_data) = sound_data {
                            let sound_handle = self
                                .audio_manager
                                .play::<StaticSoundData>(sound_data.clone())?;
                            self.current_tile_sound = Some((tile_type, sound_handle));
                        } else {
                            self.current_tile_sound = None;
                        }
                    }
                }
                Event::TimeChanged(environmental_conditions) => {
                    let weather_type = environmental_conditions.get_weather_condition();

                    if self.current_weather_sound.is_none() || weather_type != self.current_weather_sound.as_ref().unwrap().0 {
                        if let Some(previous_weather_sound) = &mut self.current_weather_sound {
                            let _ = previous_weather_sound.1.stop(Tween::default());
                        }

                        let sound_data = self.weather_type_to_sound_data.get(&weather_type);

                        if let Some(sound_data) = sound_data {
                            let sound_handle = self
                                .audio_manager
                                .play::<StaticSoundData>(sound_data.clone())?;
                            self.current_weather_sound = Some((weather_type, sound_handle));
                        } else {
                            self.current_weather_sound = None;
                        }
                    }
                }
                _ => {}
            };

            Ok(())
        }

        pub fn play_audio(
            &mut self,
            sound_config: &OxAgSoundConfig,
        ) -> Result<(), OxAgAudioToolError> {
            let sound_data = sound_config.to_sound_data()?;

            self.audio_manager.play(sound_data)?;

            Ok(())
        }
    }
}
