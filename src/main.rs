pub mod evidence;
pub mod ghosts; 
pub mod state;

use std::{io::{self}, thread, time::Duration};
use evidence::Evidence;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, ListItem, List, Paragraph, Gauge, Table, Cell, Row},
    Terminal, prelude::{Direction, Layout, Backend, Constraint, Rect}, Frame, style::{Style, Color, Stylize}
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use state::SelectionState;

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let mut state = SelectionState::new();

    // Start a thread to discard any input events. Without handling events, the
    // stdin buffer will fill up, and be read into the shell when the program exits.
    loop {
        terminal.draw(|f| {
            ui(f, &state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('e') => state.toggle(Evidence::EMF),
                KeyCode::Char('d') => state.toggle(Evidence::DOTS),
                KeyCode::Char('u') => state.toggle(Evidence::Ultraviolet),
                KeyCode::Char('f') => state.toggle(Evidence::Freezing),
                KeyCode::Char('g') => state.toggle(Evidence::GhostOrbs),
                KeyCode::Char('w') => state.toggle(Evidence::Writing),
                KeyCode::Char('s') => state.toggle(Evidence::SpiritBox),
                KeyCode::Char('i') => state.next_difficulty(),
                _ => {}
            }
        }
    }


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

fn ui<B: Backend>(f: &mut Frame<B>, state: &SelectionState) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Min(5),
                Constraint::Min(5),
                Constraint::Percentage(80),
            ].as_ref()
        )
        .split(f.size());

    //render evidences

    render_evidence_table(main_layout[0], f, state);

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
        .block(Block::default().title("Ghosts").borders(Borders::ALL))
        .highlight_style(Style::default().bold())
        .highlight_symbol("> ");
    let ghost_evidence_box = Paragraph::new("blah blah placeholder text")
        .block(Block::default().title("Ghost Information").borders(Borders::ALL));
    f.render_widget(ghost_names_list, ghost_layout[0]);
    f.render_widget(ghost_evidence_box, ghost_layout[1]);

}

fn render_evidence_table<B: Backend>(area: Rect, f: &mut Frame<B>, state: &SelectionState) {
    // 3x3 grid even though we only use 7 of the slots
    let data = vec![
        (Evidence::EMF, "(E)MF 5", Color::Red),
        (Evidence::DOTS, "(D).O.T.S", Color::Green),
        (Evidence::Ultraviolet, "(U)V", Color::Magenta),
        (Evidence::Freezing, "(F)reezing", Color::LightCyan),
        (Evidence::GhostOrbs, "(G)host Orbs", Color::Yellow),
        (Evidence::Writing, "(W)riting", Color::Blue),
        (Evidence::SpiritBox, "(S)pirit Box", Color::LightRed)
    ];

    let mut row_one_vec = Vec::new();
    let mut row_two_vec = Vec::new();
    let mut row_three_vec = Vec::new();
    for row in 0..3 {
        let this_row_vec = match row {
            0 => &mut row_one_vec,
            1 => &mut row_two_vec,
            _ => &mut row_three_vec
        };

        for col in 0..3 {
            let i = row*3 + col;
            if i <= 6 {
                let evidence_info = &data[i];
                let mut label = String::new();
                if state.marked(evidence_info.0) {
                    label.push_str("[x] ");
                } else {
                    label.push_str("[ ] ")
                }
                label.push_str(evidence_info.1);
    
                this_row_vec.push(Cell::from(label).style(Style::default().fg(evidence_info.2).bold()));
            } else if i == 7 {
                this_row_vec.push(Cell::from(""));
            } else if i == 8 {
                
                this_row_vec.push(Cell::from(format!("Ev(i)dences: {}", state.current_difficulty())));
            }
            
        }
    }
    let table = Table::new(vec![
        Row::new(row_one_vec),
        Row::new(row_two_vec),
        Row::new(row_three_vec)
    ])
    .block(Block::default().title("Ev(i)dence").borders(Borders::ALL))
    .widths(&[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)]);

    f.render_widget(table, area);
    
}