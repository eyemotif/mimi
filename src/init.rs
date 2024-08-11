use crate::cli::Args;
use crate::state::{State, Terminal};
use crossterm::{cursor, execute, terminal};
use std::io::{Read, Result};

pub fn init(args: &Args) -> Result<(State, Terminal)> {
    let (file, is_readonly) = read_file(&args.file)?;
    let terminal = init_terminal()?;

    Ok((
        State {
            file,
            is_readonly: is_readonly || args.readonly,
            cursor: 0,
        },
        terminal,
    ))
}
pub fn deinit(mut terminal: Terminal) -> std::io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;

    Ok(())
}

fn init_terminal() -> Result<Terminal> {
    let terminal = std::io::stdout();
    let terminal = ratatui::backend::CrosstermBackend::new(terminal);
    let mut terminal = ratatui::Terminal::new(terminal)?;

    terminal::enable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
    )?;

    Ok(terminal)
}
fn read_file(path: &std::path::Path) -> Result<(String, bool)> {
    let mut file = std::fs::File::open(path)?;

    let meta = file.metadata()?;
    let readonly = meta.permissions().readonly();

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok((buf, readonly))
}
