use crate::simulation::entities::{Map, ResourceType};
use crate::simulation::map::TerrainType;
use crate::simulation::robot_ai::robot::Robot;
use crate::simulation::robot_ai::types::RobotType;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct MapVisualizer;

impl MapVisualizer {
    pub fn ui(f: &mut Frame, app: &App) {
        // Layout principal horizontal : Carte à gauche, Stats à droite
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(75), // 75% pour la carte
                Constraint::Percentage(25), // 25% pour les stats
            ])
            .split(f.area());

        // Layout vertical pour la partie droite (stats)
        let stats_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), // Resources
                Constraint::Percentage(50), // Robots
            ])
            .split(main_chunks[1]);

        // Rendu de la carte (zone principale à gauche)
        Self::render_map(f, main_chunks[0], app);

        // Rendu des stats (zones à droite)
        Self::render_resources_stats(f, stats_chunks[0], app);
        Self::render_robots_stats(f, stats_chunks[1], app);
    }

    fn render_map(f: &mut Frame, area: Rect, app: &App) {
        let map_height = area.height as usize - 2;
        let map_width = area.width as usize - 2;

        let cell_width = 2;
        let visible_height = map_height.min(app.map.height);
        let visible_width = (map_width / cell_width).min(app.map.width);

        let mut start_y = app
            .scroll_y
            .min(app.map.height.saturating_sub(visible_height));
        let mut start_x = app
            .scroll_x
            .min(app.map.width.saturating_sub(visible_width));

        let mut center_y = 0;
        let mut center_x = 0;

        if app.map.height < visible_height {
            center_y = (visible_height - app.map.height) / 2;
            start_y = 0;
        }

        if app.map.width < visible_width {
            center_x = (visible_width - app.map.width) / 2;
            start_x = 0;
        }

        let mut lines = Vec::new();

        for row in 0..visible_height {
            let mut spans = Vec::new();

            for col in 0..visible_width {
                let map_y = start_y + row.saturating_sub(center_y);
                let map_x = start_x + col.saturating_sub(center_x);

                if row >= center_y
                    && row < center_y + app.map.height.min(visible_height)
                    && col >= center_x
                    && col < center_x + app.map.width.min(visible_width)
                    && map_x < app.map.width
                    && map_y < app.map.height
                {
                    let (symbol, color) = Self::get_cell_display(app.map, app.robots, map_x, map_y);
                    spans.push(Span::styled(
                        format!("{} ", symbol),
                        Style::default().fg(color),
                    ));
                } else {
                    spans.push(Span::raw("  "));
                }
            }
            lines.push(Line::from(spans));
        }

        // Informations de scroll et dimensions dans le titre
        let scroll_info = if app.map.height > visible_height || app.map.width > visible_width {
            format!(" - Défilement: ↑↓←→ ({},{}) | Taille: {}x{}",
                    app.scroll_x, app.scroll_y, app.map.width, app.map.height)
        } else {
            format!(" | Taille: {}x{}", app.map.width, app.map.height)
        };

        let title = format!("Carte de Simulation Nova{}", scroll_info);
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().fg(Color::White));

        let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_resources_stats(f: &mut Frame, area: Rect, app: &App) {
        let (energy_count, mineral_count, scientific_count) =
            Self::calculate_resource_stats(app.map);

        let resource_text = vec![
            Line::from(vec![Span::raw("")]),
            Line::from(vec![
                Span::styled("Energie: ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("{}", energy_count),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Mineraux: ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("{}", mineral_count),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Scientifique: ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("{}", scientific_count),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![Span::raw("")]), // Ligne vide
            Line::from(vec![
                Span::styled("E", Style::default().fg(Color::White)),
                Span::raw("=Energie "),
                Span::styled("M", Style::default().fg(Color::White)),
                Span::raw("=Mineral"),
            ]),
            Line::from(vec![
                Span::styled("S", Style::default().fg(Color::White)),
                Span::raw("=Scientifique "),
                Span::styled("@", Style::default().fg(Color::White)),
                Span::raw("=Station"),
            ]),
        ];

        let resource_block = Block::default()
            .title(" ")
            .borders(Borders::empty())
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().fg(Color::White));

        let resource_paragraph = Paragraph::new(resource_text)
            .block(resource_block)
            .wrap(Wrap { trim: true });

        f.render_widget(resource_paragraph, area);
    }

    fn render_robots_stats(f: &mut Frame, area: Rect, app: &App) {
        let (explorers, harvesters, scientists) = app.get_robot_stats();
        let total_robots = explorers + harvesters + scientists;

        let robot_text = vec![
            Line::from(vec![Span::styled(
                "",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![Span::raw("")]), // Ligne vide
            Line::from(vec![
                Span::styled("Total Robots: ", Style::default().fg(Color::White)),
                Span::styled(
                    format!("{}", total_robots),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![Span::raw("")]), // Ligne vide
            Line::from(vec![Span::raw("")]), // Ligne vide
            Line::from(vec![Span::styled(
                "Symboles:",
                Style::default().fg(Color::White).add_modifier(Modifier::ITALIC),
            )]),
            Line::from(vec![
                Span::styled("X", Style::default().fg(Color::White)),
                Span::raw("=Explorateur "),
                Span::styled("H", Style::default().fg(Color::White)),
                Span::raw("=Recolteur"),
            ]),
            Line::from(vec![
                Span::styled("R", Style::default().fg(Color::White)),
                Span::raw("=Scientifique"),
            ]),
        ];

        let robot_block = Block::default()
            .title(" ")
            .borders(Borders::empty())
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().fg(Color::White));

        let robot_paragraph = Paragraph::new(robot_text)
            .block(robot_block)
            .wrap(Wrap { trim: true });

        f.render_widget(robot_paragraph, area);
    }

    fn get_cell_display(map: &Map, robots: &[Robot], x: usize, y: usize) -> (char, Color) {
        let station_pos = (map.width / 2, map.height / 2);

        if (x, y) == station_pos {
            return ('@', Color::White);
        }

        for robot in robots {
            if robot.x == x && robot.y == y {
                return match robot.robot_type {
                    RobotType::Explorer => ('E', Color::Yellow),
                    RobotType::Scientist => ('S', Color::Red),
                };
            }
        }

        if let Some((resource_type, _)) = map.resources.get(&(x, y)) {
            return match resource_type {
                ResourceType::Energy => ('🔋', Color::White),
                ResourceType::Mineral => ('🪨', Color::White),
            };
        }

        let terrain_type = TerrainType::from(map.terrain[y][x]);
        match terrain_type {
            TerrainType::Plain => ('.', Color::White),
            TerrainType::Hill => ('^', Color::White),
            TerrainType::Mountain => ('M', Color::Green),
            TerrainType::Canyon => ('#', Color::White),
        }
    }

    fn calculate_resource_stats(map: &Map) -> (u32, u32, u32) {
        let mut energy_count = 0;
        let mut mineral_count = 0;
        let mut scientific_count = 0;

        for (resource_type, amount) in map.resources.values() {
            match resource_type {
                ResourceType::Energy => energy_count += amount,
                ResourceType::Mineral => mineral_count += amount,
            }
        }

        (energy_count, mineral_count, scientific_count)
    }
}

pub struct App<'a> {
    pub map: &'a Map,
    pub robots: &'a [Robot],
    pub scroll_x: usize,
    pub scroll_y: usize,
}

impl<'a> App<'a> {
    pub fn new(map: &'a Map, robots: &'a [Robot]) -> Self {
        Self {
            map,
            robots,
            scroll_x: 0,
            scroll_y: 0,
        }
    }

    pub fn get_robot_stats(&self) -> (usize, usize, usize) {
        let mut explorers = 0;
        let mut harvesters = 0;
        let mut scientists = 0;

        for robot in self.robots {
            match robot.robot_type {
                RobotType::Explorer => explorers += 1,
                RobotType::Scientist => scientists += 1,
            }
        }

        (explorers, harvesters, scientists)
    }
}