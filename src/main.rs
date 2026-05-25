use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use serde::Deserialize;
use reqwest::header::AUTHORIZATION;
use chrono::{DateTime,Utc};    
use std::time::SystemTime;
use std::collections::HashMap;
 
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

struct BasketballGamesToday{
    games: Vec<Stats>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let url = format!("https://api.balldontlie.io/nba/v1/games");
    let client = reqwest::blocking::Client::new();
    let now = SystemTime::now();
    let now:    DateTime<Utc> = now.into();
    let now = now.to_rfc3339();
    let mut params = HashMap::new();
    params.insert("dates[]", now);
    let mut games_list = Vec::<Stats>::new();
    let data: AllData = client.get(url).header(AUTHORIZATION, "65ab2ce7-ad41-43bf-a304-0f7ec82a392f").query(&params).send().expect("Error getting api").json().expect("Error parsing Json");
    for game in data.data{
        games_list.push(game);
    }
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}
 
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}
 
fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}