use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;
use ugg_types::mappings::Mode;

use crate::context::{AppContext, State};

pub fn handle_events(ctx: &mut AppContext) -> anyhow::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            // Ignore release events to fix undesired double-input issues.
            // https://github.com/ratatui-org/ratatui/issues/347
            if key.kind == event::KeyEventKind::Release {
                return Ok(false);
            }

            if key.kind == event::KeyEventKind::Press
                && key.code == KeyCode::Char('q')
                && key.modifiers.contains(KeyModifiers::CONTROL)
            {
                return Ok(true);
            }
            match ctx.state {
                State::ChampSelected | State::Initial => match key.code {
                    KeyCode::Char('s') => {
                        ctx.state = State::TextInput;
                    }
                    KeyCode::Char('c') => {
                        ctx.state = State::ChampScroll;
                        if !ctx.champ_list.is_empty() {
                            ctx.champ_scroll_pos = Some(0);
                        }
                    }
                    KeyCode::Char('m') => {
                        ctx.state = State::ModeSelect;
                        ctx.mode_scroll_pos = Some(ctx.mode_scroll_pos.unwrap_or_default());
                    }
                    KeyCode::Char('v') => {
                        ctx.state = State::VersionSelect;
                        ctx.version_scroll_pos = Some(ctx.version_scroll_pos.unwrap_or_default());
                    }
                    _ => {}
                },
                State::TextInput => match key.code {
                    KeyCode::Esc => ctx.return_to_initial(true),
                    KeyCode::Enter => {
                        ctx.state = State::ChampScroll;
                        if !ctx.champ_list.is_empty() {
                            ctx.champ_scroll_pos = Some(0);
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
                    KeyCode::Esc => ctx.return_to_initial(true),
                    KeyCode::Up => ctx.prev_champ(),
                    KeyCode::Down => ctx.next_champ(),
                    KeyCode::Enter => {
                        if let Some(champ) = ctx
                            .champ_scroll_pos
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
                        ctx.champ_scroll_pos = None;
                    }
                    _ => {}
                },
                State::ModeSelect => match key.code {
                    KeyCode::Esc => ctx.return_to_initial(false),
                    KeyCode::Up => ctx.prev_mode(),
                    KeyCode::Down => ctx.next_mode(),
                    KeyCode::Enter => {
                        if let Some(mode) = ctx.mode_scroll_pos.and_then(|p| Mode::all().get(p)) {
                            ctx.mode = *mode;
                            ctx.state = State::Initial;
                            if let Some(champ) = ctx.selected_champ.clone() {
                                ctx.select_champion(&champ);
                                ctx.state = State::ChampSelected;
                            }
                        }
                    }
                    _ => {}
                },
                State::VersionSelect => match key.code {
                    KeyCode::Esc => ctx.return_to_initial(false),
                    KeyCode::Up => ctx.prev_version(),
                    KeyCode::Down => ctx.next_version(),
                    KeyCode::Enter => {
                        let allowed_versions = ctx.api.allowed_versions.clone();
                        if let Some(version) =
                            ctx.version_scroll_pos.and_then(|p| allowed_versions.get(p))
                        {
                            if ctx.version != version.ddragon {
                                ctx.replace(&version.ddragon)?;
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(false)
}
