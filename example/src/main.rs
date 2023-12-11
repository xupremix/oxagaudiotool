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
use oxagaudiotool::audio_source_config::OxAgSoundConfig;
use oxagaudiotool::audio_tool::OxAgAudioTool;
use oxagaudiotool::error::error::OxAgAudioToolError;

struct DjRobot {
    robot: Robot,
    audio: OxAgAudioTool
}

impl Runnable for DjRobot {
    fn process_tick(&mut self, world: &mut World) {

    }
    fn handle_event(&mut self, event: Event) {
        self.audio.play_audio_based_on_event(&event);

        match event {
            Event::Ready => {
                self.audio.play_audio(&OxAgSoundConfig::new_looped("assets/music.ogg"));
            },
            _ => {}
        };

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
        weather.push(WeatherType::TrentinoSnow);
        weather.push(WeatherType::TrentinoSnow);
        weather.push(WeatherType::TrentinoSnow);
        weather.push(WeatherType::Rainy);
        weather.push(WeatherType::Rainy);
        weather.push(WeatherType::Rainy);
        weather.push(WeatherType::Rainy);


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
    let mut events = HashMap::new();

    let mut weather = HashMap::new();
    weather.insert(WeatherType::TrentinoSnow, OxAgSoundConfig::new_looped("assets/winter.ogg"));
    weather.insert(WeatherType::Rainy, OxAgSoundConfig::new_looped("assets/rain.ogg"));

   // events.insert(Event::EnergyRecharged(Default::default()), OxAgSoundConfig::new("assets/blip.ogg".to_string()));
    let audio = OxAgAudioTool::new(events, HashMap::new(), weather)?;

    let dj = DjRobot {
        robot: Robot::new(),
        audio: audio
    };
    let mut gen = MyGen::new();
    let run = Runner::new(Box::new(dj), &mut gen);
    match run {
        | Ok(mut r) => {
            let _ = loop {
                r.game_tick();
                sleep(Duration::from_millis(        10));
            };
        }
        | Err(e) => println!("{:?}", e),
    }

    Ok(())
}
