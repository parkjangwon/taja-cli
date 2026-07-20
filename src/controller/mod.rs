mod game;
mod menu;
mod practice;

use crate::app::{ActiveScreen, App};
use crossterm::event::KeyCode;

pub fn handle_key_press(app: &mut App, key: KeyCode) -> bool {
    match app.active_screen {
        ActiveScreen::MainMenu => menu::handle_main_menu(app, key),
        ActiveScreen::FingerPracticeMenu => menu::handle_finger_practice_menu(app, key),
        ActiveScreen::FingerPracticeLevelMenu { is_korean } => {
            menu::handle_finger_practice_level_menu(app, key, is_korean)
        }
        ActiveScreen::WordPracticeMenu => menu::handle_word_practice_menu(app, key),
        ActiveScreen::SentencePracticeMenu => menu::handle_sentence_practice_menu(app, key),
        ActiveScreen::Stats => menu::handle_stats(app, key),
        ActiveScreen::GameModeMenu => menu::handle_game_mode_menu(app, key),
        ActiveScreen::GameLanguageMenu { game_type } => {
            menu::handle_game_language_menu(app, key, game_type)
        }
        ActiveScreen::GameTimeSelect { is_korean } => {
            menu::handle_game_time_select(app, key, is_korean)
        }
        ActiveScreen::LongTextRaceMenu { is_korean } => {
            menu::handle_long_text_race_menu(app, key, is_korean)
        }
        ActiveScreen::FingerPractice { level, is_korean } => {
            practice::handle_finger_practice(app, key, level, is_korean)
        }
        ActiveScreen::WordPractice { is_korean } => {
            practice::handle_word_practice(app, key, is_korean)
        }
        ActiveScreen::SentencePractice { is_korean } => {
            practice::handle_sentence_practice(app, key, is_korean)
        }
        ActiveScreen::TimeAttack { is_korean } => game::handle_time_attack(app, key, is_korean),
        ActiveScreen::Survival { is_korean } => game::handle_survival(app, key, is_korean),
        ActiveScreen::TypingRain { is_korean } => game::handle_typing_rain(app, key, is_korean),
        ActiveScreen::FlashTyping { is_korean } => game::handle_flash_typing(app, key, is_korean),
        ActiveScreen::DailyChallenge { is_korean } => {
            game::handle_daily_challenge(app, key, is_korean)
        }
        ActiveScreen::LongTextRace {
            is_korean,
            text_idx,
        } => game::handle_long_text_race(app, key, is_korean, text_idx),
        ActiveScreen::GameOver {
            game_type,
            is_korean,
        } => game::handle_game_over(app, key, game_type, is_korean),
    }
}

pub fn handle_runtime_tick(app: &mut App, terminal_width: u16, terminal_height: u16) {
    game::handle_runtime_tick(app, terminal_width, terminal_height);
}
