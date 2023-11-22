use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;
use ugg_types::mappings::Mode;

use crate::context::{AppContext, State};

pub fn handle_events(ctx: &mut AppContext) -> anyhow::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press
                && key.code == KeyCode::Char('q')
                && key.modifiers.contains(KeyModifiers::CONTROL)
            {
                return Ok(true);
            }
            if ctx.state != State::TextInput
                && key.kind == event::KeyEventKind::Press
                && key.code == KeyCode::Char('m')
            {
                // Cycle through all modes.
                ctx.mode = match ctx.mode {
                    Mode::Normal => Mode::ARAM,
                    Mode::ARAM => Mode::OneForAll,
                    Mode::OneForAll => Mode::URF,
                    Mode::URF => Mode::ARURF,
                    Mode::ARURF => Mode::NexusBlitz,
                    Mode::NexusBlitz => Mode::Normal,
                };
                let selected = ctx.selected_champ.clone();
                if let Some(champ) = selected {
                    ctx.select_champion(&champ);
                }
                return Ok(false);
            }
            match ctx.state {
                State::ChampSelected | State::Initial => {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('s') {
                        ctx.state = State::TextInput;
                    }
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('c') {
                        ctx.state = State::ChampScroll;
                        if !ctx.champ_list.is_empty() {
                            ctx.scroll_pos = Some(0);
                        }
                    }
                }
                State::TextInput => match key.code {
                    KeyCode::Esc => {
                        ctx.state = State::Initial;
                        ctx.scroll_pos = None;
                    }
                    KeyCode::Enter => {
                        ctx.state = State::ChampScroll;
                        if !ctx.champ_list.is_empty() {
                            ctx.scroll_pos = Some(0);
                        }
                        if ctx.champ_list.len() == 1 {
                            if let Some(champ) = ctx
                                .list_indices
                                .first()
                                .and_then(|p| ctx.champ_data.iter().find(|(i, _)| i == p))
                                .map(|(_, c)| c)
                                .cloned()
                            {
                                ctx.select_champion(&champ);
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        ctx.input.handle_event(&Event::Key(key));
                        ctx.update_champ_list();
                    }
                    _ => {
                        if ctx.input.value().len() < 17 {
                            ctx.input.handle_event(&Event::Key(key));
                            ctx.update_champ_list();
                        }
                    }
                },
                State::ChampScroll => match key.code {
                    KeyCode::Esc => {
                        ctx.state = State::Initial;
                        ctx.scroll_pos = None;
                    }
                    KeyCode::Up => {
                        if let Some(pos) = ctx.scroll_pos {
                            if pos > 0 {
                                ctx.scroll_pos = Some(pos - 1);
                            }
                        }
                    }
                    KeyCode::Down => {
                        if let Some(pos) = ctx.scroll_pos {
                            if pos < ctx.champ_list.len() - 1 {
                                ctx.scroll_pos = Some(pos + 1);
                            }
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(champ) = ctx
                            .scroll_pos
                            .and_then(|p| ctx.list_indices.get(p))
                            .and_then(|p| ctx.champ_data.iter().find(|(i, _)| i == p))
                            .map(|(_, c)| c)
                            .cloned()
                        {
                            ctx.select_champion(&champ);
                        }
                    }
                    KeyCode::Char('s') => {
                        ctx.state = State::TextInput;
                        ctx.scroll_pos = None;
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(false)
}
