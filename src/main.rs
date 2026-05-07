use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal, Frame,
	crossterm::event::{self, Event},
	layout::{Constraint, Layout, Spacing},
	style::{Color, Stylize},
	symbols::merge::MergeStrategy,
	text::ToSpan,
	widgets::{Block, BorderType},
};
use std::fmt;

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
		.constraints(vec![Constraint::Fill(1), Constraint::Max(3)])
		.spacing(Spacing::Overlap(1))
		.split(frame.area());

	// Text area
	frame.render_widget(
		Block::bordered()
			.border_type(BorderType::Rounded)
			.fg(Color::LightGreen)
			.title(" Zedditor ".to_span().into_centered_line())
			.merge_borders(MergeStrategy::Fuzzy),
		layout[0],
	);

	// Command area
	let mut mode = app_state.mode.to_string();
	mode = format!(" {mode} ");
	frame.render_widget(
		Block::bordered()
			.border_type(BorderType::Rounded)
			.fg(Color::LightGreen)
			.title(mode)
			.merge_borders(MergeStrategy::Fuzzy),
		layout[1],
	);
}
