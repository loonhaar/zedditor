use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal, Frame,
	crossterm::event::{self, Event, KeyCode},
	layout::{Constraint, HorizontalAlignment, Layout, Spacing},
	style::{Color, Stylize},
	symbols::merge::MergeStrategy,
	widgets::{Block, BorderType, Paragraph},
};
use std::fmt;

#[derive(Debug, Default)]
enum Mode {
	#[default]
	Command,
	Edit, // TODO: make the borders and titles yellow in edit mode
}

impl fmt::Display for Mode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug, Default)]
struct AppState {
	mode: Mode,
	input: String,
	buffer: String,
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
			match app_state.mode {
				Mode::Command => {
					if handle_command(key.code, app_state) {
						break;
					}
				}
				Mode::Edit => handle_edit(key.code, app_state),
			}
		}
	}
	Ok(())
}

fn handle_command(code: KeyCode, app_state: &mut AppState) -> bool {
	// Removes the `?` if the last command was unsuccssful
	if app_state.input == "?" {
		app_state.input.clear();
	}

	match code {
		KeyCode::Char(c) => app_state.input.push(c),
		KeyCode::Backspace => {
			app_state.input.pop();
		}
		KeyCode::Enter => {
			if app_state.input == "q" {
				return true;
			}

			match &*app_state.input {
				"q" => {
					return true;
				}
				"a" => {
					app_state.mode = Mode::Edit;
				}
				_ => {
					app_state.input.clear();
					app_state.input.push('?');
				}
			}

			if !(app_state.input == "?") {
				app_state.input.clear();
			}
		}
		KeyCode::Esc => {
			return true;
		}
		_ => {}
	}

	false
}

fn handle_edit(code: KeyCode, app_state: &mut AppState) {
	match code {
		KeyCode::Esc => {
			app_state.mode = Mode::Command;
		}
		KeyCode::Char(c) => app_state.buffer.push(c),
		KeyCode::Backspace => {
			app_state.buffer.pop();
		}
		KeyCode::Enter => {
			app_state.buffer.push('\n');
		}
		_ => {}
	}
}

// TODO: function that checks if there are unsaved changes when user tries to leave

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

	let inner_editor_area = editor_pane.inner(layout[0]);

	let editor_widget = Paragraph::new(app_state.buffer.as_str());

	let command_pane = Block::bordered()
		.border_type(BorderType::Rounded)
		.fg(Color::LightGreen)
		.title(mode)
		.merge_borders(MergeStrategy::Fuzzy);

	let inner_command_area = command_pane.inner(layout[1]);

	let input_widget = Paragraph::new(app_state.input.as_str());

	frame.render_widget(editor_pane, layout[0]);
	frame.render_widget(editor_widget, inner_editor_area);

	frame.render_widget(command_pane, layout[1]);
	frame.render_widget(input_widget, inner_command_area);

	match app_state.mode {
		Mode::Command => frame.set_cursor_position((
			inner_command_area.x + app_state.input.len() as u16,
			inner_command_area.y,
		)),
		Mode::Edit => frame.set_cursor_position(calculate_cursor_position(
			app_state,
			inner_editor_area.x,
			inner_editor_area.y,
		)),
	}
}

fn calculate_cursor_position(app_state: &mut AppState, x: u16, y: u16) -> (u16, u16) {
	let number_of_cr: u16 = app_state
		.buffer
		.chars()
		.filter(|c| *c == '\n')
		.count()
		.try_into()
		.unwrap();

	let mut last_line_len: u16 = 0;

	for c in app_state.buffer.chars().rev() {
		if c == '\n' {
			break;
		}
		last_line_len += 1;
	}

	(last_line_len + x, number_of_cr + y)
}
