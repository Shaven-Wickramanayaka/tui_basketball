use color_eyre::{eyre::Ok};

use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Direction, Layout}, style::{ Color, Style}, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, Paragraph}
};
use serde::Deserialize;
use reqwest::header::AUTHORIZATION;
use chrono::{DateTime,Utc};    
use std::time::SystemTime;
use std::collections::HashMap;

struct App{
    games: Vec<Stats>,
}
impl App{
    fn new(games: Vec<Stats>) -> Self{
        Self{games}
    }

}
#[derive(Deserialize)]
struct AllData{
    data: Vec<Stats>
}
#[derive(Deserialize)]
struct Stats{
    home_team: HomeTeam,
    visitor_team: VisitorTeam,
    home_team_score: u64,
    visitor_team_score:u64,
}
#[derive(Deserialize)]
struct HomeTeam{
    full_name: String,
    abbreviation: String,
}
#[derive(Deserialize)]
struct VisitorTeam{
    full_name: String,
    abbreviation: String,   
}

fn main() -> color_eyre::Result<()> {

    color_eyre::install()?;
    let games = get_games()?;   
    let mut app = App::new(games);
    ratatui::run(|terminal| run(terminal,&mut app))?;
    Ok(())
}
fn get_games() -> color_eyre::Result<Vec<Stats>>{
    let url = format!("https://api.balldontlie.io/nba/v1/games");
    let client = reqwest::blocking::Client::new();
    let now: DateTime<Utc> = SystemTime::now().into();
    let mut params = HashMap::new();
    params.insert("dates[]", now.format("%Y-%m-%d").to_string());
    let data: AllData = client.get(url).header(AUTHORIZATION, "65ab2ce7-ad41-43bf-a304-0f7ec82a392f").query(&params).send().expect("Error getting api").json().expect("Error parsing Json");
    Ok(data.data)
}
fn run(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()>{
    loop {
        terminal.draw(|f| render(f, app))?;
    }
}

fn render(frame: &mut Frame, app: &App) {   
    let sections = Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(4),Constraint::Min(0),Constraint::Length(3)]).split(frame.area());
    let title = Paragraph::new(Line::from(vec![Span::styled("NBA Games today",Style::default().fg(Color::White))])).block(Block::default().borders(Borders::ALL));
let items: Vec<ListItem> = app.games
    .iter()
    .enumerate()
    .map(|(_i, game)| {
        let title = Line::from(Span::styled(
            format!("{} vs {}", game.home_team.full_name, game.visitor_team.full_name),
            Style::default().fg(Color::White),
        ));

        let score = Line::from(vec![
            Span::raw(" "),
            Span::styled(&game.home_team.abbreviation, Style::default().fg(Color::Cyan)),
            Span::raw(format!(" {} ", game.home_team_score)),
            Span::styled(&game.visitor_team.abbreviation, Style::default().fg(Color::Cyan)),
            Span::raw(format!(" {} ", game.visitor_team_score)),
        ]);

        ListItem::new(vec![title, score, Line::raw("")])
    })
    .collect();
    let list = List::new(items).block(Block::default().title("Games").borders(Borders::ALL)).highlight_style(Style::default().bg(Color::Cyan));
    frame.render_widget(title, sections[0]);
    frame.render_widget(list, sections[1]);

}
