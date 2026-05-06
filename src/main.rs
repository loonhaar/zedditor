use color_eyre::eyre::{Ok, Result};
use ratatui::{
	DefaultTerminal, Frame,
	crossterm::event::{self, Event},
	layout::{Constraint, Layout},
	style::{Color, Stylize},
	text::ToSpan,
	widgets::{Block, Widget},
};

fn main() -> Result<()> {
	color_eyre::install()?;

	let terminal = ratatui::init();
	let result = run(terminal);

	ratatui::restore();
	result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
	loop {
		terminal.draw(|t| render(t))?;

		// TODO mode switching via key events
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

fn render(frame: &mut Frame) {
	let [border_area] = Layout::vertical([Constraint::Fill(1)])
		.margin(0)
		.areas(frame.area());

	// Text area
	Block::bordered()
		.border_type(ratatui::widgets::BorderType::Rounded)
		.fg(Color::LightGreen)
		.title(" Zedditor ".to_span().into_centered_line())
		.render(border_area, frame.buffer_mut());

	// Command area
	Block::bordered()
		.border_type(ratatui::widgets::BorderType::Rounded)
		.fg(Color::LightGreen)
		.title(" Zedditor ".to_span().into_centered_line())
		.render(border_area, frame.buffer_mut());
}
