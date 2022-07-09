use std::{error::Error, io, slice::Chunks, sync::Arc};

use crossterm::{
    event::{self, *},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::Style,
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Clear},
    Frame, Terminal,
};

struct App {
    pub new_task_pop_up: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            new_task_pop_up: false,
        }
    }
}

pub fn start_ui() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        println!("UI Crashed:\n{:#?}", e);
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
            if let KeyCode::Char('n') = key.code {
                app.new_task_pop_up = !app.new_task_pop_up;
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("TODO List")
        .title_alignment(Alignment::Center);

    let chunks = base_layout(f);
    f.render_widget(main_block, chunks[0]);
    f.render_widget(command_helper(), chunks[1]);

    if app.new_task_pop_up {
        // Todo wrap pop up widget in main widget
        let block = Block::default()
            .title("New task")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);
        let area = centered_rect(60, 20, f.size());
        f.render_widget(Clear, area);
        f.render_widget(block, area);
    }
}

fn base_layout<B: Backend>(f: &Frame<B>) -> Vec<Rect> {
    vec![
        Rect::new(
            f.size().x,
            f.size().y,
            f.size().width,
            f.size().height.checked_sub(3).unwrap_or(0),
        ),
        Rect::new(
            f.size().x,
            f.size().height.checked_sub(3).unwrap_or(0),
            f.size().width,
            3,
        ),
    ]
}

fn command_helper() -> Paragraph<'static> {
    Paragraph::new(Text::raw(
        "q: Quit | Space: Select | n: New task | d: delete | h: left | j: up | k: down | l: right",
    ))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2)
            ]
            .as_ref(),
        )
        .split(r);
    
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2)
            ]
            .as_ref()
        )
        .split(popup_layout[1])[1]
}