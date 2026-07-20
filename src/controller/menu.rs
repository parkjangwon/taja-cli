use crate::app::{ActiveScreen, App, GameType};
use crossterm::event::KeyCode;

pub fn handle_main_menu(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Up => {
            app.menu_selected_idx = app.menu_selected_idx.saturating_sub(1);
        }
        KeyCode::Down => {
            if app.menu_selected_idx < 5 {
                app.menu_selected_idx += 1;
            }
        }
        KeyCode::Enter => match app.menu_selected_idx {
            0 => {
                app.active_screen = ActiveScreen::FingerPracticeMenu;
                app.menu_selected_idx = 0;
            }
            1 => {
                app.active_screen = ActiveScreen::WordPracticeMenu;
                app.menu_selected_idx = 0;
            }
            2 => {
                app.active_screen = ActiveScreen::SentencePracticeMenu;
                app.menu_selected_idx = 0;
            }
            3 => {
                app.active_screen = ActiveScreen::Stats;
                app.cached_frequent_errors = app.storage.get_frequent_errors(10);
            }
            4 => {
                app.active_screen = ActiveScreen::GameModeMenu;
                app.menu_selected_idx = 0;
            }
            5 => return true,
            _ => {}
        },
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return true,
        _ => {}
    }
    false
}

pub fn handle_finger_practice_menu(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Up => {
            app.menu_selected_idx = app.menu_selected_idx.saturating_sub(1);
        }
        KeyCode::Down => {
            if app.menu_selected_idx < 1 {
                app.menu_selected_idx += 1;
            }
        }
        KeyCode::Enter => {
            let is_korean = app.menu_selected_idx == 0;
            app.active_screen = ActiveScreen::FingerPracticeLevelMenu { is_korean };
            app.menu_selected_idx = 0;
        }
        KeyCode::Esc => {
            app.active_screen = ActiveScreen::MainMenu;
            app.menu_selected_idx = 0;
        }
        _ => {}
    }
    false
}

pub fn handle_finger_practice_level_menu(app: &mut App, key: KeyCode, is_korean: bool) -> bool {
    let max_idx = if is_korean { 5 } else { 4 };
    match key {
        KeyCode::Up => {
            app.menu_selected_idx = app.menu_selected_idx.saturating_sub(1);
        }
        KeyCode::Down => {
            if app.menu_selected_idx < max_idx {
                app.menu_selected_idx += 1;
            }
        }
        KeyCode::Enter => {
            let level = app.menu_selected_idx + 1;
            app.active_screen = ActiveScreen::FingerPractice { level, is_korean };
            let target = App::get_finger_practice_target(level, is_korean);
            app.start_practice_session(target);
        }
        KeyCode::Esc => {
            app.active_screen = ActiveScreen::FingerPracticeMenu;
            app.menu_selected_idx = if is_korean { 0 } else { 1 };
        }
        _ => {}
    }
    false
}

pub fn handle_word_practice_menu(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Up => {
            app.menu_selected_idx = app.menu_selected_idx.saturating_sub(1);
        }
        KeyCode::Down => {
            if app.menu_selected_idx < 1 {
                app.menu_selected_idx += 1;
            }
        }
        KeyCode::Enter => {
            let is_korean = app.menu_selected_idx == 0;
            app.active_screen = ActiveScreen::WordPractice { is_korean };
            app.setup_word_practice(is_korean);
        }
        KeyCode::Esc => {
            app.active_screen = ActiveScreen::MainMenu;
            app.menu_selected_idx = 1;
        }
        _ => {}
    }
    false
}

pub fn handle_sentence_practice_menu(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Up => {
            app.menu_selected_idx = app.menu_selected_idx.saturating_sub(1);
        }
        KeyCode::Down => {
            if app.menu_selected_idx < 1 {
                app.menu_selected_idx += 1;
            }
        }
        KeyCode::Enter => {
            let is_korean = app.menu_selected_idx == 0;
            app.active_screen = ActiveScreen::SentencePractice { is_korean };
            app.setup_sentence_practice(is_korean);
        }
        KeyCode::Esc => {
            app.active_screen = ActiveScreen::MainMenu;
            app.menu_selected_idx = 2;
        }
        _ => {}
    }
    false
}

pub fn handle_stats(app: &mut App, key: KeyCode) -> bool {
    if key == KeyCode::Esc {
        app.active_screen = ActiveScreen::MainMenu;
        app.menu_selected_idx = 3;
    }
    false
}

pub fn handle_game_mode_menu(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Up => {
            app.menu_selected_idx = app.menu_selected_idx.saturating_sub(1);
        }
        KeyCode::Down => {
            if app.menu_selected_idx < 5 {
                app.menu_selected_idx += 1;
            }
        }
        KeyCode::Enter => {
            let game_type = match app.menu_selected_idx {
                0 => GameType::TimeAttack,
                1 => GameType::Survival,
                2 => GameType::TypingRain,
                3 => GameType::FlashTyping,
                4 => GameType::DailyChallenge,
                5 => GameType::LongTextRace,
                _ => GameType::TimeAttack,
            };
            app.active_screen = ActiveScreen::GameLanguageMenu { game_type };
            app.menu_selected_idx = 0;
        }
        KeyCode::Esc => {
            app.active_screen = ActiveScreen::MainMenu;
            app.menu_selected_idx = 4;
        }
        _ => {}
    }
    false
}

pub fn handle_game_language_menu(app: &mut App, key: KeyCode, game_type: GameType) -> bool {
    match key {
        KeyCode::Up => {
            app.menu_selected_idx = app.menu_selected_idx.saturating_sub(1);
        }
        KeyCode::Down => {
            if app.menu_selected_idx < 1 {
                app.menu_selected_idx += 1;
            }
        }
        KeyCode::Enter => {
            let is_korean = app.menu_selected_idx == 0;
            match game_type {
                GameType::TimeAttack => {
                    app.active_screen = ActiveScreen::GameTimeSelect { is_korean };
                    app.menu_selected_idx = 0;
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
                    app.active_screen = ActiveScreen::LongTextRaceMenu { is_korean };
                    app.menu_selected_idx = 0;
                }
            }
        }
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

pub fn handle_game_time_select(app: &mut App, key: KeyCode, is_korean: bool) -> bool {
    match key {
        KeyCode::Up => {
            app.menu_selected_idx = app.menu_selected_idx.saturating_sub(1);
        }
        KeyCode::Down => {
            if app.menu_selected_idx < 2 {
                app.menu_selected_idx += 1;
            }
        }
        KeyCode::Enter => {
            let time_secs = match app.menu_selected_idx {
                0 => 30,
                1 => 60,
                2 => 120,
                _ => 60,
            };
            app.setup_time_attack(is_korean, time_secs);
            app.active_screen = ActiveScreen::TimeAttack { is_korean };
        }
        KeyCode::Esc => {
            app.active_screen = ActiveScreen::GameLanguageMenu {
                game_type: GameType::TimeAttack,
            };
            app.menu_selected_idx = 0;
        }
        _ => {}
    }
    false
}

pub fn handle_long_text_race_menu(app: &mut App, key: KeyCode, is_korean: bool) -> bool {
    let max_idx = App::get_long_text_titles(is_korean).len().saturating_sub(1);
    match key {
        KeyCode::Up => {
            app.menu_selected_idx = app.menu_selected_idx.saturating_sub(1);
        }
        KeyCode::Down => {
            if app.menu_selected_idx < max_idx {
                app.menu_selected_idx += 1;
            }
        }
        KeyCode::Enter => {
            let idx = app.menu_selected_idx;
            app.setup_long_text_race(is_korean, idx);
            app.active_screen = ActiveScreen::LongTextRace {
                is_korean,
                text_idx: idx,
            };
        }
        KeyCode::Esc => {
            app.active_screen = ActiveScreen::GameLanguageMenu {
                game_type: GameType::LongTextRace,
            };
            app.menu_selected_idx = 0;
        }
        _ => {}
    }
    false
}
