use crate::app::{ActiveScreen, App, GameType};
use crate::hangeul;
use crossterm::event::KeyCode;

fn finish_game(app: &mut App, game_type: GameType, is_korean: bool, mode_name: &str) {
    app.stop_timer();
    app.save_session_record(mode_name, if is_korean { "한글" } else { "영어" });
    app.active_screen = ActiveScreen::GameOver {
        game_type,
        is_korean,
    };
}

pub fn handle_time_attack(app: &mut App, key: KeyCode, is_korean: bool) -> bool {
    if app.is_time_attack_expired() {
        finish_game(app, GameType::TimeAttack, is_korean, "게임-시간제한");
        return false;
    }

    match key {
        KeyCode::Esc => {
            if app.has_pending_game_input() {
                app.clear_game_input();
            } else {
                app.stop_timer();
                app.active_screen = ActiveScreen::GameModeMenu;
                app.menu_selected_idx = 0;
            }
        }
        KeyCode::Backspace => {
            app.input_automata.backspace();
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            let success = app.is_current_word_success();
            app.time_attack_next_word(success);
        }
        KeyCode::Char(c) => {
            app.push_game_char(c);
            if app.is_target_fully_typed() {
                let success = app.is_current_word_success();
                app.time_attack_next_word(success);
            }
        }
        _ => {}
    }
    false
}

pub fn handle_survival(app: &mut App, key: KeyCode, is_korean: bool) -> bool {
    match key {
        KeyCode::Esc => {
            if app.has_pending_game_input() {
                app.clear_game_input();
            } else {
                app.stop_timer();
                app.active_screen = ActiveScreen::GameModeMenu;
                app.menu_selected_idx = 1;
            }
        }
        KeyCode::Backspace => {
            app.input_automata.backspace();
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            let had_error = !app.is_current_word_success();
            let has_next = app.survival_next_word(had_error);
            if !has_next {
                finish_game(app, GameType::Survival, is_korean, "게임-서바이벌");
            }
        }
        KeyCode::Char(c) => {
            app.push_game_char(c);
            if app.is_target_fully_typed() {
                let had_error = !app.is_current_word_success();
                let has_next = app.survival_next_word(had_error);
                if !has_next {
                    finish_game(app, GameType::Survival, is_korean, "게임-서바이벌");
                }
            }
        }
        _ => {}
    }
    false
}

pub fn handle_typing_rain(app: &mut App, key: KeyCode, is_korean: bool) -> bool {
    if app.game_mode_lives == 0 {
        finish_game(app, GameType::TypingRain, is_korean, "게임-타자레인");
        return false;
    }

    match key {
        KeyCode::Esc => {
            if app.has_pending_game_input() {
                app.clear_game_input();
            } else {
                app.stop_timer();
                app.active_screen = ActiveScreen::GameModeMenu;
                app.menu_selected_idx = 2;
            }
        }
        KeyCode::Backspace => {
            if app.rain_active_idx.is_some() {
                app.input_automata.backspace();
                app.update_rain_typed_progress();
                if app.input_automata.get_text().is_empty() {
                    app.clear_game_input();
                }
            }
        }
        KeyCode::Char(c) => {
            if app.rain_active_idx.is_none() {
                let mut found_idx = None;
                for (i, word) in app.rain_words.iter().enumerate() {
                    if word.destroyed {
                        continue;
                    }
                    if let Some(fc) = word.text.chars().next() {
                        let input_jamo = hangeul::map_qwerty_to_jamo(c);
                        let target_jamos = hangeul::fully_decompose_hangul(fc);
                        let matches = if let Some(ij) = input_jamo {
                            target_jamos.first() == Some(&ij)
                        } else {
                            fc.eq_ignore_ascii_case(&c)
                        };
                        if matches {
                            found_idx = Some(i);
                            break;
                        }
                    }
                }
                if let Some(idx) = found_idx {
                    app.rain_active_idx = Some(idx);
                    app.rain_words[idx].active = true;
                    app.input_automata.clear();
                    app.target_text = app.rain_words[idx].text.clone();
                    app.update_automata_modes();
                    app.push_game_char(c);
                    app.update_rain_typed_progress();
                }
            } else {
                app.push_game_char(c);
                app.update_rain_typed_progress();

                if app.is_rain_word_complete() {
                    let idx = app.rain_active_idx.unwrap();
                    app.rain_words[idx].destroyed = true;
                    app.rain_words[idx].active = false;
                    app.rain_active_idx = None;
                    app.game_mode_score += 10;
                    app.game_words_correct += 1;
                    app.game_words_total += 1;
                    app.game_mode_round += 1;
                    app.game_mode_combo += 1;
                    if app.game_mode_combo > app.game_mode_max_combo {
                        app.game_mode_max_combo = app.game_mode_combo;
                    }
                    app.accumulated_strokes += app.input_automata.get_strokes();
                    app.input_automata.clear();
                    app.target_text.clear();
                    app.prune_destroyed_rain_words();
                }
            }
        }
        _ => {}
    }
    false
}

pub fn handle_flash_typing(app: &mut App, key: KeyCode, is_korean: bool) -> bool {
    match key {
        KeyCode::Esc => {
            if app.has_pending_game_input() && !app.flash_answer_shown {
                app.clear_game_input();
            } else {
                app.stop_timer();
                app.active_screen = ActiveScreen::GameModeMenu;
                app.menu_selected_idx = 3;
            }
        }
        KeyCode::Enter => {
            if app.flash_answer_shown {
                let has_next = app.flash_next_round();
                if !has_next {
                    finish_game(app, GameType::FlashTyping, is_korean, "게임-플래시타이핑");
                }
            } else if !app.flash_visible {
                app.flash_submit_answer();
            } else {
                app.flash_visible = false;
            }
        }
        KeyCode::Backspace => {
            if !app.flash_answer_shown && !app.flash_visible {
                app.input_automata.backspace();
            }
        }
        KeyCode::Char(c) => {
            if !app.flash_answer_shown {
                if app.flash_visible {
                    app.flash_visible = false;
                }
                app.push_game_char(c);
            }
        }
        _ => {}
    }
    false
}

pub fn handle_daily_challenge(app: &mut App, key: KeyCode, is_korean: bool) -> bool {
    match key {
        KeyCode::Esc => {
            if app.has_pending_game_input() {
                app.clear_game_input();
            } else {
                app.stop_timer();
                app.active_screen = ActiveScreen::GameModeMenu;
                app.menu_selected_idx = 4;
            }
        }
        KeyCode::Backspace => {
            app.input_automata.backspace();
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            app.input_automata.commit_current();
            let has_next = app.daily_next_word();
            if !has_next {
                finish_game(
                    app,
                    GameType::DailyChallenge,
                    is_korean,
                    "게임-데일리챌린지",
                );
            }
        }
        KeyCode::Char(c) => {
            app.push_game_char(c);
            if app.is_target_fully_typed() {
                app.input_automata.commit_current();
                let has_next = app.daily_next_word();
                if !has_next {
                    finish_game(
                        app,
                        GameType::DailyChallenge,
                        is_korean,
                        "게임-데일리챌린지",
                    );
                }
            }
        }
        _ => {}
    }
    false
}

pub fn handle_long_text_race(
    app: &mut App,
    key: KeyCode,
    is_korean: bool,
    text_idx: usize,
) -> bool {
    app.ensure_timer_started();
    match key {
        KeyCode::Esc => {
            if app.has_pending_game_input() {
                app.clear_game_input();
            } else {
                app.stop_timer();
                app.active_screen = ActiveScreen::LongTextRaceMenu { is_korean };
                app.menu_selected_idx = text_idx;
            }
        }
        KeyCode::Backspace => {
            app.input_automata.backspace();
        }
        KeyCode::Enter => {
            app.input_automata.commit_current();
            if app.is_target_fully_typed() {
                let has_next = app.long_text_next_paragraph();
                if !has_next {
                    finish_game(app, GameType::LongTextRace, is_korean, "게임-긴글레이스");
                }
            }
        }
        KeyCode::Char(c) => {
            app.push_game_char(c);
            if app.is_target_fully_typed() {
                app.input_automata.commit_current();
                let has_next = app.long_text_next_paragraph();
                if !has_next {
                    finish_game(app, GameType::LongTextRace, is_korean, "게임-긴글레이스");
                }
            }
        }
        _ => {}
    }
    false
}

pub fn handle_game_over(app: &mut App, key: KeyCode, game_type: GameType, is_korean: bool) -> bool {
    match key {
        KeyCode::Enter => match game_type {
            GameType::TimeAttack => {
                let time_secs = app.game_time_limit_secs;
                app.setup_time_attack(is_korean, time_secs);
                app.active_screen = ActiveScreen::TimeAttack { is_korean };
            }
            GameType::Survival => {
                app.setup_survival(is_korean);
                app.active_screen = ActiveScreen::Survival { is_korean };
            }
            GameType::TypingRain => {
                app.active_screen = ActiveScreen::TypingRain { is_korean };
                app.setup_typing_rain(is_korean);
            }
            GameType::FlashTyping => {
                app.setup_flash_typing(is_korean);
                app.active_screen = ActiveScreen::FlashTyping { is_korean };
            }
            GameType::DailyChallenge => {
                app.setup_daily_challenge(is_korean);
                app.active_screen = ActiveScreen::DailyChallenge { is_korean };
            }
            GameType::LongTextRace => {
                let idx = app.long_text_selected_idx;
                app.setup_long_text_race(is_korean, idx);
                app.active_screen = ActiveScreen::LongTextRace {
                    is_korean,
                    text_idx: idx,
                };
            }
        },
        KeyCode::Esc => {
            app.active_screen = ActiveScreen::GameModeMenu;
            app.menu_selected_idx = match game_type {
                GameType::TimeAttack => 0,
                GameType::Survival => 1,
                GameType::TypingRain => 2,
                GameType::FlashTyping => 3,
                GameType::DailyChallenge => 4,
                GameType::LongTextRace => 5,
            };
        }
        _ => {}
    }
    false
}

pub fn handle_runtime_tick(app: &mut App, terminal_width: u16, terminal_height: u16) {
    match app.active_screen {
        ActiveScreen::TimeAttack { is_korean } => {
            if app.is_time_attack_expired() {
                finish_game(app, GameType::TimeAttack, is_korean, "게임-시간제한");
            }
        }
        ActiveScreen::TypingRain { is_korean } => {
            app.rain_screen_width = terminal_width;
            app.rain_screen_height = terminal_height.saturating_sub(10);
            app.tick_rain();
            if app.game_mode_lives == 0 {
                finish_game(app, GameType::TypingRain, is_korean, "게임-타자레인");
            }
        }
        ActiveScreen::FlashTyping { .. } => {
            app.check_flash_visibility();
        }
        ActiveScreen::LongTextRace { .. } => {
            app.record_cpm_history();
        }
        _ => {}
    }
}
