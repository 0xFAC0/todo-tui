use std::{error::Error, io};

use crossterm::{
    event::{self, *},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Color,
    style::Style,
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::task::Task;

enum InputMode {
    Normal,
    Editing,
}

#[derive(Clone, Copy)]
enum Popup {
    NewTaskName,
    NewTaskDetails,
}

struct StateFullList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StateFullList<T> {
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

struct App {
    pub popup: Option<Popup>,
    pub input_mode: InputMode,
    pub input: Vec<String>,
    pub list: StateFullList<Task>,
}

impl App {
    pub fn new() -> Self {
        App {
            popup: None,
            input_mode: InputMode::Normal,
            input: vec![String::new(), String::new()],
            list: StateFullList {
                state: ListState::default(),
                items: vec![],
            },
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
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('n') => {
                            app.popup = Some(Popup::NewTaskName);
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('j') => {
                            if app.list.items.len() > 0 {
                                app.list.next();
                            }
                        }
                        KeyCode::Char('k') => match app.list.state.selected() {
                            Some(i) => {
                                if i > 0 {
                                    app.list.previous();
                                }
                            }
                            None => {
                                if app.list.items.len() > 0 {
                                    app.list.previous();
                                }
                            }
                        },
                        KeyCode::Char('d') => {
                            if let Some(i) = app.list.state.selected() {
                                if i < app.list.items.len() {
                                    app.list.items.remove(i);
                                    app.list.state.select(None);
                                }
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(i) = app.list.state.selected() {
                                app.list.items[i].done = !app.list.items[i].done;
                            }
                        }
                        _ => {}
                    }
                }
            }
            InputMode::Editing => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => {
                            if let Some(popup) = app.popup {
                                match popup {
                                    Popup::NewTaskName => app.input[0].push(c),
                                    Popup::NewTaskDetails => app.input[1].push(c),
                                };
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(popup) = app.popup {
                                match popup {
                                    Popup::NewTaskName => app.input[0].pop(),
                                    Popup::NewTaskDetails => app.input[1].pop(),
                                };
                            }
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.popup = None;
                            app.input[0] = String::new();
                            app.input[1] = String::new();
                        }
                        KeyCode::Enter => {
                            if let Some(popup) = app.popup {
                                match popup {
                                    Popup::NewTaskName => {
                                        if !app.input.is_empty() {
                                            app.popup = Some(Popup::NewTaskDetails);
                                        }
                                    }
                                    Popup::NewTaskDetails => {
                                        if app.input[1].is_empty() {
                                            app.list
                                                .items
                                                .push(Task::new(app.input[0].clone(), None));
                                        } else {
                                            app.list.items.push(Task::new(
                                                app.input[0].clone(),
                                                Some(app.input[1].clone()),
                                            ));
                                        }
                                        app.input[1] = String::new();
                                        app.input[0] = String::new();
                                        app.popup = None;
                                        app.input_mode = InputMode::Normal;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = base_layout(f);

    let items: Vec<ListItem> = app
        .list
        .items
        .iter()
        .map(|i| {
            if i.done {
                return ListItem::new(Span::raw(format!("✓ {}", i.msg.clone())));
            }
            ListItem::new(Span::raw(format!("  {}", i.msg.clone())))
        })
        .collect();
    let list = List::new(items)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Tasks")
                .title_alignment(Alignment::Center),
        );

    if let Some(i) = app.list.state.selected() {
        if let Some(ref details) = app.list.items[i].details {
            let sub_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[0]);
            f.render_stateful_widget(list, sub_chunks[0], &mut app.list.state);
            f.render_widget(
                details_win(details.clone()).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                ),
                sub_chunks[1],
            );
        } else {
            // TO REFRACTOR DUPPLICATION
            f.render_stateful_widget(list, chunks[0], &mut app.list.state);
        }
    } else {
        f.render_stateful_widget(list, chunks[0], &mut app.list.state);
    }

    f.render_widget(command_helper(), chunks[1]);

    if let Some(popup) = app.popup {
        let mut area = centered_rect(60, 20, f.size());
        f.render_widget(Clear, area);
        match popup {
            Popup::NewTaskName => {
                area.height = 3;
                f.render_widget(input_popup(app, Popup::NewTaskName), area);
            }
            Popup::NewTaskDetails => f.render_widget(input_popup(app, Popup::NewTaskDetails), area),
        }
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
        "q: Quit | Space: Select | n: New task | d: delete | h: left | j: up | k: down | l: right | Enter: Mark done",
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

fn input_popup(app: &App, popup: Popup) -> Paragraph<'static> {
    let (text, title) = match popup {
        Popup::NewTaskName => (Text::raw(app.input[0].clone()), "Add a new task"),
        Popup::NewTaskDetails => (
            Text::raw(app.input[1].clone()),
            "Add details (blank for none)",
        ),
    };
    Paragraph::new(text).wrap(Wrap { trim: true }).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(title)
            .title_alignment(Alignment::Center),
    )
}

fn details_win(details: String) -> Paragraph<'static> {
    Paragraph::new(Text::raw(details)).wrap(Wrap { trim: true })
}
