use crate::app::{ActiveScreen, App};
use crossterm::event::KeyCode;

pub fn handle_finger_practice(app: &mut App, key: KeyCode, level: usize, is_korean: bool) -> bool {
    app.ensure_timer_started();
    match key {
        KeyCode::Esc => {
            app.stop_timer();
            app.active_screen = ActiveScreen::FingerPracticeLevelMenu { is_korean };
            app.menu_selected_idx = level - 1;
        }
        KeyCode::Backspace => {
            app.input_automata.backspace();
        }
        KeyCode::Char(c) => {
            app.push_game_char(c);

            if app.is_target_fully_typed() {
                app.input_automata.commit_current();
                app.update_elapsed_time();

                let lang_str = if is_korean { "한글" } else { "영어" };
                app.save_session_record("자리연습", &format!("{} Level {}", lang_str, level));

                let next_target = App::get_finger_practice_target(level, is_korean);
                app.advance_practice_target(next_target);
            }
        }
        _ => {}
    }
    false
}

pub fn handle_word_practice(app: &mut App, key: KeyCode, is_korean: bool) -> bool {
    match key {
        KeyCode::Esc => {
            app.stop_timer();
            app.active_screen = ActiveScreen::WordPracticeMenu;
            app.menu_selected_idx = if is_korean { 0 } else { 1 };
        }
        KeyCode::Backspace => {
            app.input_automata.backspace();
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            app.input_automata.commit_current();
            if !app.is_target_fully_typed() {
                if let Some(last_exp) = app.target_text.chars().last() {
                    let last_typ = app.input_automata.get_text().chars().last().unwrap_or(' ');
                    app.record_error(last_exp, last_typ);
                }
            }

            let has_next = app.next_word();
            if has_next {
                // 단어 사이 타이머 일시정지 → 쉬는 시간이 CPM에 누적되지 않음
                app.pause_timer();
            } else {
                app.stop_timer();
                app.save_session_record("낱말연습", if is_korean { "한글" } else { "영어" });
                app.active_screen = ActiveScreen::WordPracticeMenu;
                app.menu_selected_idx = if is_korean { 0 } else { 1 };
            }
        }
        KeyCode::Char(c) => {
            // 첫 글자 타이핑 시 타이머 재개
            app.ensure_timer_started();
            app.push_game_char(c);
        }
        _ => {}
    }
    false
}

pub fn handle_sentence_practice(app: &mut App, key: KeyCode, is_korean: bool) -> bool {
    match key {
        KeyCode::Esc => {
            app.stop_timer();
            app.active_screen = ActiveScreen::SentencePracticeMenu;
            app.menu_selected_idx = if is_korean { 0 } else { 1 };
        }
        KeyCode::Backspace => {
            app.input_automata.backspace();
        }
        KeyCode::Enter => {
            app.input_automata.commit_current();
            let typed = app.input_automata.get_text();
            let expected = &app.target_text;

            let exp_len = expected.chars().count();
            let typ_len = typed.chars().count();
            if typ_len < exp_len {
                app.total_errors += exp_len - typ_len;
            }

            let has_next = app.next_sentence();
            if has_next {
                // 문장 사이에 타이머 일시정지 → 쉬는 시간에 CPM/시간이 누적되지 않음
                app.pause_timer();
            } else {
                app.stop_timer();
                app.save_session_record("문장연습", if is_korean { "한글" } else { "영어" });
                app.active_screen = ActiveScreen::SentencePracticeMenu;
                app.menu_selected_idx = if is_korean { 0 } else { 1 };
            }
        }
        KeyCode::Char(c) => {
            // 첫 글자 타이핑 시 타이머 재개 (일시정지 상태였다면 누적 시간 유지)
            app.ensure_timer_started();
            app.push_game_char(c);
        }
        _ => {}
    }
    false
}
