use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::io::stdout;

use crate::cli::{Cli, LogLevel};
use crate::log::LogStore;

pub fn run_app(cli: Cli, logs: LogStore) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = main_loop(&mut terminal, &cli, &logs);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn main_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    cli: &Cli,
    logs: &LogStore,
) -> anyhow::Result<()> {
    // aplica os filtros de min_level e query passados pela CLI
    let filtered_logs: Vec<_> = logs
        .entries
        .iter()
        .filter(|entry| {
            if entry.level < cli.min_level {
                return false;
            }
            if let Some(ref q) = cli.query {
                return entry.raw.to_lowercase().contains(&q.to_lowercase());
            }
            true
        })
        .collect();

    let mut list_state = ListState::default();
    if !filtered_logs.is_empty() {
        list_state.select(Some(0));
    }

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.size());

            // header informativo
            let header_text = format!(
                "rlog v0.1.0 | Arquivo: {:?} | Exibindo: {}/{} logs | [↑/↓ ou k/j]: Rolar | [q]: Sair",
                cli.file,
                filtered_logs.len(),
                logs.entries.len()
            );
            let header = Paragraph::new(header_text).block(
                Block::default()
                    .title(" Interactive Log Analyzer ")
                    .borders(Borders::ALL),
            );
            f.render_widget(header, chunks[0]);

            // monta os itens da lista estilizando de acordo com o nível
            let items: Vec<ListItem> = filtered_logs
                .iter()
                .map(|entry| {
                    let color = match entry.level {
                        LogLevel::Error => Color::Red,
                        LogLevel::Warn => Color::Yellow,
                        LogLevel::Info => Color::Green,
                        LogLevel::Debug => Color::DarkGray,
                    };
                    ListItem::new(entry.raw.as_str()).style(Style::default().fg(color))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title(" Logs ").borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::Indexed(237)));

            f.render_stateful_widget(list, chunks[1], &mut list_state);
        })?;

        // control do teclado para rolagem
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Down | KeyCode::Char('j') => {
                    if let Some(i) = list_state.selected() {
                        if !filtered_logs.is_empty() && i < filtered_logs.len() - 1 {
                            list_state.select(Some(i + 1));
                        }
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if let Some(i) = list_state.selected() {
                        if i > 0 {
                            list_state.select(Some(i - 1));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}