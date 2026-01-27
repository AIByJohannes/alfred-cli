use std::io::{self, Stdout};
use std::time::Duration;

use alfred_core::{Message, Role};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Terminal;
use tokio::sync::mpsc;
use tokio::time;

#[derive(Debug)]
enum AppEvent {
    Input(Event),
    Tick,
    AgentChunk(String),
    AgentDone,
}

struct App {
    messages: Vec<Message>,
    input: String,
    streaming_idx: Option<usize>,
    scroll: u16,
}

impl App {
    fn new() -> Self {
        Self {
            messages: Vec::new(),
            input: String::new(),
            streaming_idx: None,
            scroll: 0,
        }
    }

    fn push_user(&mut self, content: String) {
        self.messages.push(Message::new(Role::User, content));
    }

    fn append_assistant_chunk(&mut self, chunk: String) {
        let idx = match self.streaming_idx {
            Some(idx) => idx,
            None => {
                self.messages
                    .push(Message::new(Role::Assistant, String::new()));
                let idx = self.messages.len() - 1;
                self.streaming_idx = Some(idx);
                idx
            }
        };

        if let Some(message) = self.messages.get_mut(idx) {
            if !message.content.is_empty() {
                message.content.push(' ');
            }
            message.content.push_str(&chunk);
        }
    }

    fn finish_assistant(&mut self) {
        self.streaming_idx = None;
    }

    fn render_messages(&self) -> Text<'_> {
        let mut buffer = String::new();
        for message in &self.messages {
            buffer.push_str(&format!("{}: {}\n\n", message.role, message.content));
        }
        Text::from(buffer)
    }
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen);
        default_hook(info);
    }));
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    install_panic_hook();
    let _terminal_guard = TerminalGuard;
    let mut terminal = init_terminal()?;

    let (tx, mut rx) = mpsc::channel::<AppEvent>(256);

    spawn_input_reader(tx.clone());
    spawn_tick(tx.clone());

    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(2), Constraint::Length(3)])
                .split(frame.size());

            let messages = Paragraph::new(app.render_messages())
                .block(Block::default().borders(Borders::ALL).title("Alfred"))
                .wrap(Wrap { trim: false })
                .scroll((app.scroll, 0));
            frame.render_widget(messages, chunks[0]);

            let input = Paragraph::new(format!("> {}", app.input))
                .block(Block::default().borders(Borders::ALL).title("Input"))
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(input, chunks[1]);
        })?;

        let event = match rx.recv().await {
            Some(event) => event,
            None => break,
        };

        match event {
            AppEvent::Input(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    KeyCode::Esc => break,
                    KeyCode::Enter => {
                        let content = app.input.trim().to_string();
                        if !content.is_empty() {
                            app.push_user(content.clone());
                            app.input.clear();
                            spawn_mock_agent(content, tx.clone());
                        }
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Up => {
                        app.scroll = app.scroll.saturating_add(1);
                    }
                    KeyCode::Down => {
                        app.scroll = app.scroll.saturating_sub(1);
                    }
                    KeyCode::Char(ch) => {
                        app.input.push(ch);
                    }
                    _ => {}
                }
            }
            AppEvent::Input(Event::Resize(_, _)) => {}
            AppEvent::AgentChunk(chunk) => app.append_assistant_chunk(chunk),
            AppEvent::AgentDone => app.finish_assistant(),
            AppEvent::Tick => {}
            _ => {}
        }
    }

    Ok(())
}

fn spawn_input_reader(tx: mpsc::Sender<AppEvent>) {
    tokio::spawn(async move {
        loop {
            let event_ready = tokio::task::spawn_blocking(|| event::poll(Duration::from_millis(250)))
                .await
                .ok()
                .and_then(|res| res.ok())
                .unwrap_or(false);
            if event_ready {
                if let Ok(ev) = event::read() {
                    if tx.send(AppEvent::Input(ev)).await.is_err() {
                        break;
                    }
                }
            }
        }
    });
}

fn spawn_tick(tx: mpsc::Sender<AppEvent>) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(250));
        loop {
            interval.tick().await;
            if tx.send(AppEvent::Tick).await.is_err() {
                break;
            }
        }
    });
}

fn spawn_mock_agent(input: String, tx: mpsc::Sender<AppEvent>) {
    tokio::spawn(async move {
        let reply = format!("(mock) I heard: {}", input);
        for chunk in reply.split_whitespace() {
            if tx.send(AppEvent::AgentChunk(chunk.to_string())).await.is_err() {
                return;
            }
            time::sleep(Duration::from_millis(80)).await;
        }
        let _ = tx.send(AppEvent::AgentDone).await;
    });
}
