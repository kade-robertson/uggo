use ratatui::crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::context::{AppContext, State};

const fn keycode_to_logger_event(key: &event::KeyEvent) -> Option<tui_logger::TuiWidgetEvent> {
    match key.code {
        KeyCode::Char('h') => Some(tui_logger::TuiWidgetEvent::HideKey),
        KeyCode::Char('f') => Some(tui_logger::TuiWidgetEvent::FocusKey),
        KeyCode::Up => Some(tui_logger::TuiWidgetEvent::UpKey),
        KeyCode::Down => Some(tui_logger::TuiWidgetEvent::DownKey),
        KeyCode::Left => Some(tui_logger::TuiWidgetEvent::LeftKey),
        KeyCode::Right => Some(tui_logger::TuiWidgetEvent::RightKey),
        KeyCode::Char('-') => Some(tui_logger::TuiWidgetEvent::MinusKey),
        KeyCode::Char('+') => Some(tui_logger::TuiWidgetEvent::PlusKey),
        KeyCode::PageUp => Some(tui_logger::TuiWidgetEvent::PrevPageKey),
        KeyCode::PageDown => Some(tui_logger::TuiWidgetEvent::NextPageKey),
        KeyCode::Esc => Some(tui_logger::TuiWidgetEvent::EscapeKey),
        KeyCode::Char(' ') => Some(tui_logger::TuiWidgetEvent::SpaceKey),
        _ => None,
    }
}

pub fn handle_events(ctx: &mut AppContext) -> anyhow::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))?
        && let Event::Key(key) = event::read()?
    {
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
            State::ChampSelected | State::Initial => {
                if key.modifiers.contains(KeyModifiers::ALT) {
                    match key.code {
                        KeyCode::Char('s') => {
                            ctx.state = State::TextInput;
                            ctx.show_left_pane = true;
                        }
                        KeyCode::Char('c') => {
                            ctx.state = State::ChampScroll;
                            ctx.show_left_pane = true;
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
                            ctx.version_scroll_pos =
                                Some(ctx.version_scroll_pos.unwrap_or_default());
                        }
                        KeyCode::Char('w') => {
                            ctx.state = State::RegionSelect;
                            ctx.match_pos_to_region();
                        }
                        KeyCode::Char('r') => {
                            ctx.state = State::RoleSelect;
                            ctx.match_pos_to_role();
                        }
                        KeyCode::Char('b') => {
                            ctx.state = State::BuildSelect;
                            ctx.build_scroll_pos = Some(ctx.build_scroll_pos.unwrap_or_default());
                        }
                        KeyCode::Char('h') => {
                            ctx.show_left_pane = !ctx.show_left_pane;
                        }
                        KeyCode::Char('l') => {
                            ctx.state = State::Logger;
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('?') => {
                            ctx.state = State::HelpMenu;
                        }
                        KeyCode::Esc | KeyCode::Enter => {}
                        _ => {
                            ctx.state = State::TextInput;
                            ctx.show_left_pane = true;
                            ctx.on_search_keypress(key);
                        }
                    }
                }
            }
            State::TextInput => match key.code {
                KeyCode::Esc => ctx.return_to_initial(true),
                KeyCode::Enter => ctx.on_search_submit(),
                KeyCode::Backspace => ctx.on_search_backspace(key),
                _ => ctx.on_search_keypress(key),
            },
            State::ChampScroll => match key.code {
                KeyCode::Esc => ctx.return_to_initial(true),
                KeyCode::Up => ctx.prev_champ(),
                KeyCode::Down => ctx.next_champ(),
                KeyCode::Enter => ctx.select_champ(),
                KeyCode::Char('s') => ctx.go_to_search(),
                _ => {}
            },
            State::ModeSelect => match key.code {
                KeyCode::Esc => ctx.return_to_initial(false),
                KeyCode::Up => ctx.prev_mode(),
                KeyCode::Down => ctx.next_mode(),
                KeyCode::Enter => ctx.select_mode(),
                _ => {}
            },
            State::VersionSelect => match key.code {
                KeyCode::Esc => ctx.return_to_initial(false),
                KeyCode::Up => ctx.prev_version(),
                KeyCode::Down => ctx.next_version(),
                KeyCode::Enter => ctx.select_version()?,
                _ => {}
            },
            State::RegionSelect => match key.code {
                KeyCode::Esc => ctx.return_to_initial(false),
                KeyCode::Up => ctx.prev_region(),
                KeyCode::Down => ctx.next_region(),
                KeyCode::Enter => ctx.select_region(),
                _ => {}
            },
            State::RoleSelect => match key.code {
                KeyCode::Esc => ctx.return_to_initial(false),
                KeyCode::Up => ctx.prev_role(),
                KeyCode::Down => ctx.next_role(),
                KeyCode::Enter => ctx.select_role(),
                _ => {}
            },
            State::BuildSelect => match key.code {
                KeyCode::Esc => ctx.return_to_initial(false),
                KeyCode::Up => ctx.prev_build(),
                KeyCode::Down => ctx.next_build(),
                KeyCode::Enter => ctx.select_build(),
                _ => {}
            },
            State::HelpMenu => {
                if let KeyCode::Esc = key.code {
                    ctx.return_to_initial(false);
                }
            }
            State::Logger => match key.code {
                KeyCode::Char('q') => ctx.return_to_initial(false),
                _ => {
                    if let Some(event) = keycode_to_logger_event(&key) {
                        ctx.logger_state.transition(event);
                    }
                }
            },
        }
    }
    Ok(false)
}
