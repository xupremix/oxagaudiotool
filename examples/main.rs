use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::World;
use robotics_lib::world::world_generator::Generator;
use oxagaudiotool::sound_config::OxAgSoundConfig;
use oxagaudiotool::audio_tool::OxAgAudioTool;
use oxagaudiotool::error::error::OxAgAudioToolError;

struct DjRobot {
    robot: Robot,
    audio: OxAgAudioTool
}

impl Runnable for DjRobot {
    fn process_tick(&mut self, _: &mut World) {

    }
    fn handle_event(&mut self, event: Event) {
        let _ = self.audio.play_audio_based_on_event(&event);

        println!();
        println!("{:?}", event);
        println!();
    }
    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }
    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}

struct MyGen {}

impl MyGen {
    fn new() -> MyGen {
        MyGen{}
    }
}

impl Generator for MyGen {
    fn gen(&mut self) -> robotics_lib::world::world_generator::World {
        let mut weather = Vec::new();
        weather.push(WeatherType::Sunny);
        weather.push(WeatherType::TrentinoSnow);
        weather.push(WeatherType::TrentinoSnow);
        weather.push(WeatherType::Rainy);
        weather.push(WeatherType::Rainy);
        weather.push(WeatherType::Foggy);
        weather.push(WeatherType::TropicalMonsoon);


        let mut tiles = Vec::new();
        let mut another = Vec::new();
        another.push(Tile {
            tile_type: TileType::DeepWater,
            content: Content::None,
            elevation: 0,
        });
        tiles.push(another);
        (tiles, (0, 0), EnvironmentalConditions::new(&weather, 3, 3).unwrap(), 0.1, None)
    }
}

fn main() -> Result<(), OxAgAudioToolError>{
    println!("Loading game...");

    // Setup the sound configs
    // We suggest you to use small files as if you use too many big audio files the startup times may increase
    let background_music = OxAgSoundConfig::new_looped_with_volume("assets/default/music.ogg", 2.0);

    let mut events = HashMap::new();
    events.insert(Event::Ready, OxAgSoundConfig::new("assets/default/event/event_ready.ogg"));
    events.insert(Event::Terminated, OxAgSoundConfig::new("assets/default/event/event_terminated.ogg"));
    // events.insert(Event::EnergyRecharged(0), OxAgSoundConfig::new_with_volume("assets/default/event/event_energy_recharged.ogg", 0.1));
    events.insert(Event::AddedToBackpack(Content::None, 0), OxAgSoundConfig::new("assets/default/event/event_add_to_backpack.ogg"));
    events.insert(Event::RemovedFromBackpack(Content::None, 0), OxAgSoundConfig::new("assets/default/event/event_remove_from_backpack.ogg"));

    let mut tiles = HashMap::new();
    tiles.insert(TileType::DeepWater, OxAgSoundConfig::new("assets/default/tile/tile_water.ogg"));
    tiles.insert(TileType::ShallowWater, OxAgSoundConfig::new("assets/default/tile/tile_water.ogg"));
    tiles.insert(TileType::Sand, OxAgSoundConfig::new("assets/default/tile/tile_sand.ogg"));
    tiles.insert(TileType::Grass, OxAgSoundConfig::new("assets/default/tile/tile_grass.ogg"));
    tiles.insert(TileType::Hill, OxAgSoundConfig::new("assets/default/tile/tile_grass.ogg"));
    tiles.insert(TileType::Mountain, OxAgSoundConfig::new("assets/default/tile/tile_mountain.ogg"));
    tiles.insert(TileType::Snow, OxAgSoundConfig::new("assets/default/tile/tile_snow.ogg"));
    tiles.insert(TileType::Lava, OxAgSoundConfig::new("assets/default/tile/tile_lava.ogg"));
    tiles.insert(TileType::Teleport(false), OxAgSoundConfig::new("assets/default/tile/tile_teleport.ogg"));
    tiles.insert(TileType::Street, OxAgSoundConfig::new("assets/default/tile/tile_street.ogg"));

    let mut weather = HashMap::new();
    weather.insert(WeatherType::Rainy, OxAgSoundConfig::new("assets/default/weather/weather_rainy.ogg"));
    weather.insert(WeatherType::Foggy, OxAgSoundConfig::new("assets/default/weather/weather_foggy.ogg"));
    weather.insert(WeatherType::Sunny, OxAgSoundConfig::new("assets/default/weather/weather_sunny.ogg"));
    weather.insert(WeatherType::TrentinoSnow, OxAgSoundConfig::new("assets/default/weather/weather_winter.ogg"));
    weather.insert(WeatherType::TropicalMonsoon, OxAgSoundConfig::new("assets/default/weather/weather_tropical.ogg"));

    // Create the audio tool
    let audio = OxAgAudioTool::new(events, tiles, weather)?;

    let mut dj = DjRobot {
        robot: Robot::new(),
        audio
    };

    let mut gen = MyGen::new();

    println!("Running!");

    // Play background music :>
    dj.audio.play_audio(&background_music)?;

    let run = Runner::new(Box::new(dj), &mut gen);

    match run {
        | Ok(mut r) => {
            let _ = loop {
                let _ = r.game_tick();
                sleep(Duration::from_millis(100));
            };
        }
        | Err(e) => println!("{:?}", e),
    }

    Ok(())
}
