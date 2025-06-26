mod config;
mod maps;
mod robot;

use crate::maps::{Map, Station};
use crate::robot::{Robot, RobotType};

use config::Config;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    env_logger::init();

    // Vérifier les arguments de ligne de commande
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "start" {
        // Créer une configuration par défaut
        let config = Config::new();
        start_simulation(config).await;
    } else {
        println!("Usage: cargo run start");
        println!("Use 'cargo run start' to start the simulation");
    }
}

async fn start_simulation(config: Config) {
    println!("Starting simulation with:");
    println!("  Seed: {}", config.seed);
    println!("  Map: {}x{}", config.map_width, config.map_height);
    println!("  Robots: {}", config.robots_count);

    let mut map = Map::new(config.map_width, config.map_height, config.seed);
    let mut station = Station::new(config.map_width / 2, config.map_height / 2);

    station.receive_resource(crate::maps::entities::ResourceType::Energy, 10000);

    let mut robots = Vec::new();
    for i in 0..config.robots_count {
        let robot_type = match i % 2 {
            0 => RobotType::Explorer,
            _ => RobotType::Scientist,
        };

        let station_pos = station.position();
        let robot = Robot::new(i, robot_type, station_pos.0, station_pos.1, 100);
        robots.push(robot);
    }

    let map_path = format!("map_seed_{}.json", config.seed);
    if let Err(e) = map.save_to_file(&map_path) {
        eprintln!("Failed to save map: {}", e);
    } else {
        println!("Map saved to {}", map_path);
    }

    println!("Simulation running... Press 'q' in TUI to quit");
    tokio::time::sleep(Duration::from_millis(2000)).await;

    enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).expect("Failed to enter alternate screen");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal");

    let mut last_update = Instant::now();
    let robot_start_times = schedule_robot_start_times(robots.len());
    let result = loop {
        if last_update.elapsed() >= Duration::from_millis(500) {
            for (i, robot) in robots.iter_mut().enumerate() {
                if Instant::now() >= robot_start_times[i] {
                    let action = robot.decide_next_action(&map, &station);

                    if let Err(e) = robot.execute_action(&mut map, &mut station, action) {
                        eprintln!("Robot {} failed to execute action: {}", robot.id, e);
                    }
                }
            }

            last_update = Instant::now();
        }

        if let Err(e) = draw_tui(&mut terminal, &map, &robots) {
            break Err(e);
        }

        if event::poll(Duration::from_millis(100)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.code == KeyCode::Char('q') {
                    break Ok(());
                }
            }
        }
    };

    disable_raw_mode().expect("Failed to disable raw mode");
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .expect("Failed to leave alternate screen");
    terminal.show_cursor().expect("Failed to show cursor");

    match result {
        Ok(()) => println!("Simulation stopped by user"),
        Err(e) => eprintln!("Simulation error: {}", e),
    }
}

fn draw_tui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    map: &Map,
    robots: &[Robot],
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::maps::visualization::MapVisualizer;

    terminal.draw(|f| {
        let app = crate::maps::visualization::App::new(map, robots);
        MapVisualizer::ui(f, &app);
    })?;

    Ok(())
}

fn schedule_robot_start_times(robot_count: usize) -> Vec<Instant> {
    let mut start_times = Vec::new();
    for i in 0..robot_count {
        start_times.push(Instant::now() + Duration::from_millis(i as u64 * 1000));
    }
    start_times
}