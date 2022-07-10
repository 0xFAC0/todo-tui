use std::{error::Error, io};

use crossterm::{
    event::{self, *},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame, Terminal,
};

enum InputMode {
    Normal,
    Editing,
}

struct App {
    pub new_task_pop_up: bool,
    pub input_mode: InputMode,
    pub input: String,
}

impl App {
    pub fn new() -> Self {
        App {
            new_task_pop_up: false,
            input_mode: InputMode::Normal,
            input: String::new(),
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
        match app.input_mode {
            InputMode::Normal => {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                    if let KeyCode::Char('n') = key.code {
                        app.new_task_pop_up = true;
                        app.input_mode = InputMode::Editing;
                    }
                }
            }
            InputMode::Editing => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            if app.input.len() > 0 {
                                app.input.pop();
                            }
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.new_task_pop_up = false;
                            app.input = String::new();
                        }
                        KeyCode::Enter => {
                            //TODO add a tasklist to app
                            // TODO tasklist.new(blabla) if !input.is_empty()
                            app.input_mode = InputMode::Normal;
                            app.new_task_pop_up = false;
                        }
                        _ => {}
                    }
                }
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
        let mut area = centered_rect(60, 20, f.size());
        area.height = 3;
        f.render_widget(Clear, area);
        f.render_widget(input_pop_up(app), area);
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
                Constraint::Percentage((100 - percent_y) / 2),
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
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn input_pop_up(app: &App) -> Paragraph<'static> {
    Paragraph::new(Text::raw(app.input.clone()))
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Add a new task")
                .title_alignment(Alignment::Center),
        )
}
