use std::fmt;
use std::fmt::Debug;

use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal, Frame,
	crossterm::event::{self, Event},
	layout::{Constraint, Layout},
	style::{Color, Stylize},
	text::ToSpan,
	widgets::Block,
};

#[derive(Debug, Default)]
enum Mode {
	#[default]
	Command,
	//Edit, // TODO: make the borders and titles yellow in edit mode
}

impl fmt::Display for Mode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug, Default)]
struct AppState {
	mode: Mode,
	// TODO: buffer,
}

fn main() -> Result<()> {
	let mut app_state = AppState::default();

	color_eyre::install()?;

	let terminal = ratatui::init();
	let result = run(terminal, &mut app_state);

	ratatui::restore();
	result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
	loop {
		terminal.draw(|t| render(t, app_state))?;

		if let Event::Key(key) = event::read()? {
			match key.code {
				event::KeyCode::Esc => {
					break;
				}
				_ => {}
			}
		}
	}
	Ok(())
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
	let layout = Layout::default()
		.direction(ratatui::layout::Direction::Vertical)
		.constraints(vec![
			Constraint::from_mins([3, 3]),
			Constraint::from_maxes([65535, 3]),
		])
		.split(frame.area());

	// Text area
	frame.render_widget(
		Block::bordered()
			.border_type(ratatui::widgets::BorderType::Rounded)
			.fg(Color::LightGreen)
			.title(" Zedditor ".to_span().into_centered_line()),
		layout[0],
	);

	let mut mode = app_state.mode.to_string();
	mode = format!(" {mode} ");
	frame.render_widget(
		// Command area
		Block::bordered()
			.border_type(ratatui::widgets::BorderType::Rounded)
			.fg(Color::LightGreen)
			.title(mode),
		layout[1],
	);
}
