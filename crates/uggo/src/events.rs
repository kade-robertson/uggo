use crossterm::event::{self, Event, KeyCode, KeyModifiers};

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
                    KeyCode::Char('w') => {
                        ctx.state = State::RegionSelect;
                        ctx.match_pos_to_region();
                    }
                    KeyCode::Char('r') => {
                        ctx.state = State::RoleSelect;
                        ctx.match_pos_to_role();
                    }
                    KeyCode::Char('?') => {
                        ctx.state = State::HelpMenu;
                    }
                    _ => {}
                },
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
                State::HelpMenu => {
                    if let KeyCode::Esc = key.code {
                        ctx.return_to_initial(false);
                    }
                }
            }
        }
    }
    Ok(false)
}
