pub mod evidence;
pub mod ghosts; 
pub mod state;
pub mod util;

use std::{io, fs, collections::HashMap, time::{SystemTime, Duration}, borrow::BorrowMut};
use evidence::Evidence;
use ghosts::Ghost;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, ListItem, List, Paragraph, Gauge, Table, Cell, Row, Wrap, ListState},
    Terminal, prelude::{Direction, Layout, Backend, Constraint, Rect}, Frame, style::{Style, Color, Stylize}, text::{Span, Line}
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::Deserialize;
use state::{SelectionState, MarkState};
use util::hunt_sanity_color;

#[derive(Deserialize)]
struct GhostDocument {
    ghosts: HashMap<String, Ghost>
}

fn main() -> Result<(), io::Error> {
    let ghost_file_paths = ["./ghosts.toml", "/etc/phasmoterm/ghosts.toml"];

    let mut ghosts: Vec<Ghost> = Vec::new();

    for path in ghost_file_paths {
        println!("trying {}", path);
        match fs::read_to_string(path) {
            Ok(content) => {
                println!("found at {}", path);
                
                match toml::from_str::<GhostDocument>(&content) {
                    Ok(table) => {
                        table.ghosts.values().for_each(|k| ghosts.push(k.clone()));
                        break;
                    },
                    Err(e) => {
                        println!("failed to parse {}: {}", path, e);
                    }
                }
            },
            _ => {} //file not found - continue
        }
    }

    if ghosts.is_empty() {
        println!("no ghost data found");
        return Ok(());
    }

    ghosts.sort_by_key(|k| k.name.clone()); 

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = SelectionState::new();
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    let mut time = SystemTime::now();
    loop {
        terminal.draw(|f| {
            ui(f, &state, &ghosts, &mut list_state);
        })?;

        let now = SystemTime::now();
        let delta = now
            .duration_since(time)
            .unwrap();
        state.tick_smudge(delta.as_secs_f32());
        time = now;

        if let Ok(true) = event::poll(Duration::from_secs_f32(0.5)) {
            let event = event::read()?;
            if let Event::Key(key) = event {
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
                    KeyCode::Char('t') => state.start_smudge(),
                    KeyCode::Char('r') => state.reset(),
                    KeyCode::Down => list_scroll_by(&mut list_state, 1),
                    KeyCode::Tab => list_scroll_by(&mut list_state, 1),
                    KeyCode::PageDown => list_scroll_by(&mut list_state, 10),
                    KeyCode::Up => list_scroll_by(&mut list_state, -1),
                    KeyCode::BackTab => list_scroll_by(&mut list_state, -1),
                    KeyCode::PageUp => list_scroll_by(&mut list_state, -10),
                    KeyCode::Delete => {
                        
                        if let Some(index) = list_state.selected() {
                            let ghost_opt = {
                                //re-calculating this is bleh
                                let possible_ghosts = state.possible_ghosts(&ghosts);
                                possible_ghosts.get(index).map(|k| (*k).clone())
                            };
                            
                            if let Some(ghost) = ghost_opt {
                                state.toggle_omit(&ghost);
                            }
                        }
                    },
                    _ => {}
                }
            }

            if let Event::Mouse(mouse_event) = event {
                match mouse_event.kind {
                    event::MouseEventKind::ScrollDown => {
                        list_scroll_by(&mut list_state, 1)
                    }
                    event::MouseEventKind::ScrollUp => {
                        list_scroll_by(&mut list_state, -1)
                    }
                    _ => {}
                }
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

fn list_scroll_by(list_state: &mut ListState, amount: i32) {
    match list_state.selected() {
        Some(s) => {
            let pos = i32::max(0, (s as i32) + amount);
            list_state.select(Some(pos as usize))
        },
        _ => list_state.select(Some(0))
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, state: &SelectionState, ghosts: &Vec<Ghost>, list_state: &mut ListState) {
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

    let possible_ghosts = state.possible_ghosts(ghosts);

    //make sure our selected list state is within the range of possible ghosts
    let selected_index = list_state.selected().unwrap_or_default();
    if possible_ghosts.is_empty() {
        list_state.select(None);
    } else {
        list_state.select(Some(usize::clamp(selected_index, 0, possible_ghosts.len()-1)));
    }

    //render evidences

    render_evidence_table(main_layout[0], f, state, &possible_ghosts);

    // smudge timer
    let smudge_remaining = state.smudge_remaining();

    let smudge_parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]
        ).split(main_layout[1]);

    let smudge_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50), //spirit
                Constraint::Percentage(17), //all ghosts
                Constraint::Percentage(8), //demon
            ]
        ).split(smudge_parent_layout[1]);

    let smudge_timer = Gauge::default()
        .block(Block::default().title("Incense (T)imer").borders(Borders::LEFT | Borders::RIGHT | Borders::TOP))
        .gauge_style(Style::default().fg(Color::White))
        .label(format!("{}s", smudge_remaining))
        .percent(f64::round(smudge_remaining as f64/180.0*100.0) as u16);

    f.render_widget(smudge_timer, smudge_parent_layout[0]);

    let smudge_points = [
        ("┕ Spirit", Color::Cyan),
        ("┕ Default", Color::Yellow),
        ("┕ Demon", Color::Red)
    ];
    for i in 0..smudge_points.len() {
        let borders = match i {
            0 => Borders::LEFT | Borders::BOTTOM,
            1 => Borders::BOTTOM,
            2 => Borders::RIGHT | Borders::BOTTOM,
            _ => Borders::NONE
        };
        f.render_widget(
            Paragraph::new(
                Span::from(smudge_points[i].0)
                .fg(smudge_points[i].1)
            ).block(Block::default().borders(borders)), 
            smudge_layout[i]
        );
    }

    //render ghost info

    let ghost_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ].as_ref())
        .split(main_layout[2]);
    
    let ghost_names_elems: Vec<ListItem> = possible_ghosts.iter()
        .map(|g| {
            let mut spans = Vec::new();
            //push name
            spans.push(Span::from(format!("{} (",g.name.clone())));
            //evidence

            for i in 0..g.evidence.len() {
                let e = g.evidence[i];
                let mut s = Span::from(format!("{}",e.symbol())).fg(e.color());
                if state.marked(e) == MarkState::Positive {
                    s = s.bold();
                };
                if g.is_guaranteed_evidence(e) && state.current_difficulty() < 3 && state.current_difficulty() > 0 {
                    s = s.underlined();
                }
                spans.push(
                   s
                );
                if i != g.evidence.len()-1 {
                   spans.push(Span::from("/"));
                }
            }
            spans.push(Span::from(")"));
            //speed
            if g.max_speed == 0f32 {
                spans.push(Span::from(format!(" {}m/s,", g.min_speed)));
            } else {
                spans.push(Span::from(format!(" {}m/s - {}m/s,", g.min_speed, g.max_speed)));
            }

            //hunt sanity
            let min_color = hunt_sanity_color(g.min_hunt_sanity);
            let max_color = hunt_sanity_color(g.max_hunt_sanity);
            if g.max_hunt_sanity == 0 {
                spans.push(Span::from(format!(" {}%", g.min_hunt_sanity)).fg(min_color));
            } else {
                spans.push(Span::from(format!(" {}%", g.min_hunt_sanity)).fg(min_color));
                spans.push(Span::from(" - "));
                spans.push(Span::from(format!("{}%", g.max_hunt_sanity)).fg(max_color));
            };
            if state.is_omitted(&g) {
                spans = spans.into_iter().map(|k| k.bg(Color::DarkGray)).collect();
            }
            ListItem::new(Line::from(spans))
        })
        .collect();
    
    let ghost_names_list = List::new(ghost_names_elems)
        .block(Block::default().title(format!("Ghosts ({})", possible_ghosts.len())).borders(Borders::ALL))
        .highlight_style(Style::default().bold())
        .highlight_symbol("> ");

    let paragraph = if possible_ghosts.is_empty() {
        Paragraph::new("No possible ghosts given current restrictions".red().bold())
    } else {
        match list_state.selected() {
            Some(i) => possible_ghosts[i].render_information(),
            _ => Paragraph::new("No ghost selected")
        }
    };

    let ghost_evidence_box = paragraph
        .block(Block::default()
        .title("Ghost Information")
        .borders(Borders::ALL))
        .wrap(Wrap { trim: false});
    f.render_stateful_widget(ghost_names_list, ghost_layout[0], list_state);
    f.render_widget(ghost_evidence_box, ghost_layout[1]);

}

fn render_evidence_table<B: Backend>(area: Rect, f: &mut Frame<B>, state: &SelectionState, possible_ghosts: &Vec<&Ghost>) {
    // 3x3 grid even though we only use 7 of the slots
    let data = vec![
        (Evidence::EMF, "(E)MF 5"),
        (Evidence::DOTS, "(D).O.T.S"),
        (Evidence::Ultraviolet, "(U)V"),
        (Evidence::Freezing, "(F)reezing"),
        (Evidence::GhostOrbs, "(G)host Orbs"),
        (Evidence::Writing, "(W)riting"),
        (Evidence::SpiritBox, "(S)pirit Box")
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
                let s = match state.marked(evidence_info.0) {
                    MarkState::Positive => "[✓]",
                    MarkState::Neutral => "[ ]",
                    MarkState::Negative => "[✗]"
                };
                let label = format!{"{} {}", s, evidence_info.1};
                let is_possible = evidence_info.0.possible(possible_ghosts);
                let mut style = if is_possible {
                    Style::default().fg(evidence_info.0.color()).bold()
                } else {
                    Style::default().fg(Color::Gray).bg(Color::DarkGray)
                };

                if possible_ghosts.is_empty() {
                    style = style.slow_blink().bg(Color::Reset);
                }
                
                this_row_vec.push(Cell::from(label).style(style));
            } else if i == 7 {
                this_row_vec.push(Cell::from("(R)eset"));
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
    .block(Block::default().title("Evidence").borders(Borders::ALL))
    .widths(&[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)]);

    f.render_widget(table, area);
    
}
