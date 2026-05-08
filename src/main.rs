use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal, Frame,
	crossterm::event::{self, Event},
	layout::{Constraint, HorizontalAlignment, Layout, Spacing},
	style::{Color, Stylize},
	symbols::merge::MergeStrategy,
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
	let constraints = [Constraint::Min(3), Constraint::Length(3)];

	let layout = Layout::vertical(constraints)
		.spacing(Spacing::Overlap(1))
		.split(frame.area());

	let mut mode = app_state.mode.to_string();
	mode = format!(" {mode} ");

	let editor_pane = Block::bordered()
		.border_type(BorderType::Rounded)
		.fg(Color::LightGreen)
		.title(" Zedditor ")
		.title_alignment(HorizontalAlignment::Center)
		.merge_borders(MergeStrategy::Fuzzy);

	let command_pane = Block::bordered()
		.border_type(BorderType::Rounded)
		.fg(Color::LightGreen)
		.title(mode)
		.merge_borders(MergeStrategy::Fuzzy);

	frame.render_widget(editor_pane, layout[0]);
	frame.render_widget(command_pane, layout[1]);
}
