use std::{io::{self, Stdout}, thread, time::Duration};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, ListItem, List, Paragraph, Gauge},
    Terminal, prelude::{Direction, Layout, Backend, Constraint}, Frame, style::{Style, Color}
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    

    terminal.draw(|f| {
        ui(f);
    })?;

    // Start a thread to discard any input events. Without handling events, the
    // stdin buffer will fill up, and be read into the shell when the program exits.
    thread::spawn(|| loop {
        event::read();
    });

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(80),
            ].as_ref()
        )
        .split(f.size());

    //render evidences

    let evidences = Block::default()
        .title("Ev(i)dences")
        .borders(Borders::ALL);
    f.render_widget(evidences, main_layout[0]);

    // smudge timer

    let smudge_timer = Gauge::default()
        .block(Block::default().title("Incense (T)imer").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::White))
        .label("5")
        .percent(40);

    f.render_widget(smudge_timer, main_layout[1]);

    //render ghost info

    let ghost_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60)
        ].as_ref())
        .split(main_layout[2]);
    
    let ghost_names_elems = [ListItem::new("Spirit"), ListItem::new("The Mimic"), ListItem::new("Hantu")];
    let ghost_names_list = List::new(ghost_names_elems)
        .block(Block::default().title("Ghosts").borders(Borders::ALL));
    let ghost_evidence_box = Paragraph::new("blah blah placeholder text")
        .block(Block::default().title("Ghost Information").borders(Borders::ALL));
    f.render_widget(ghost_names_list, ghost_layout[0]);
    f.render_widget(ghost_evidence_box, ghost_layout[1]);

}