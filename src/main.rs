mod hangeul;
mod storage;
mod app;
mod ui;

use app::{App, ActiveScreen, GameType};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, terminal::Terminal};
use std::{io, time::Duration};

fn main() -> Result<(), io::Error> {
    // 1. 터미널 환경 설정 (Raw Mode 활성화 및 Alternate Screen 진입)
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. 앱 상태 생성 및 실행
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // 3. 터미널 복원 (오류 발생 시에도 복원)
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        // UI 그리기
        terminal.draw(|f| ui::draw(f, app))?;

        // 100ms 폴링으로 키 이벤트가 없어도 실시간 통계(시간 경과) 업데이트
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.active_screen {
                        ActiveScreen::MainMenu => {
                            match key.code {
                                KeyCode::Up => {
                                    app.menu_selected_idx = app.menu_selected_idx.saturating_sub(1);
                                }
                                KeyCode::Down => {
                                    if app.menu_selected_idx < 5 {
                                        app.menu_selected_idx += 1;
                                    }
                                }
                                KeyCode::Enter => {
                                    match app.menu_selected_idx {
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
                                            // 자주 틀리는 키 목록 캐시 갱신
                                            app.cached_frequent_errors = app.storage.get_frequent_errors(10);
                                        }
                                        4 => {
                                            app.active_screen = ActiveScreen::GameModeMenu;
                                            app.menu_selected_idx = 0;
                                        }
                                        5 => return Ok(()), // 종료
                                        _ => {}
                                    }
                                }
                                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }
                        ActiveScreen::FingerPracticeMenu => {
                            match key.code {
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
                        }
                        ActiveScreen::FingerPracticeLevelMenu { is_korean } => {
                            let max_idx = if is_korean { 5 } else { 4 }; // 한글 Level 1~6, 영어 Level 1~5
                            match key.code {
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
                        }
                        ActiveScreen::FingerPractice { level, is_korean } => {
                            app.ensure_timer_started();
                            
                            match key.code {
                                KeyCode::Esc => {
                                    // 자리 연습 중단, 메뉴로 백
                                    app.active_screen = ActiveScreen::FingerPracticeLevelMenu { is_korean };
                                    app.menu_selected_idx = level - 1;
                                }
                                KeyCode::Backspace => {
                                    app.input_automata.backspace();
                                }
                                KeyCode::Char(c) => {
                                    let current_pos = app.input_automata.get_text().chars().count();
                                    let expected_chars: Vec<char> = app.target_text.chars().collect();
                                    let expected_char = expected_chars.get(current_pos).copied();
                                    
                                    if current_pos < expected_chars.len() {
                                        let ec = expected_char.unwrap();
                                        
                                        // 입력 처리
                                        app.input_automata.push_char(c, expected_char);
                                        
                                        // 정오 판정
                                        let typed_text = app.input_automata.get_text();
                                        if let Some(new_typed_char) = typed_text.chars().nth(current_pos) {
                                            if !hangeul::is_typing_valid(new_typed_char, ec) {
                                                app.record_error(ec, c);
                                            }
                                        }
                                    }
                                    
                                    // 입력 완료 확인
                                    let typed_text = app.input_automata.get_text();
                                    let completed_len = hangeul::count_completed_chars(&typed_text, &app.target_text);
                                    let expected_len = app.target_text.chars().count();
                                    
                                    if completed_len >= expected_len {
                                        // 자리 연습은 바로 기록 저장
                                        app.input_automata.commit_current();
                                        app.update_elapsed_time();
                                        
                                        let lang_str = if is_korean { "한글" } else { "영어" };
                                        app.save_session_record("자리연습", &format!("{} Level {}", lang_str, level));
                                        
                                        // 바로 다음 문장 세트 생성해서 계속 진행
                                        let next_target = App::get_finger_practice_target(level, is_korean);
                                        app.start_practice_session(next_target);
                                    }
                                }
                                _ => {}
                            }
                        }
                        ActiveScreen::WordPracticeMenu => {
                            match key.code {
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
                        }
                        ActiveScreen::WordPractice { is_korean } => {
                            app.ensure_timer_started();
                            
                            match key.code {
                                KeyCode::Esc => {
                                    app.active_screen = ActiveScreen::WordPracticeMenu;
                                    app.menu_selected_idx = if is_korean { 0 } else { 1 };
                                }
                                KeyCode::Backspace => {
                                    app.input_automata.backspace();
                                }
                                // 단어 단위 완료는 Space 또는 Enter
                                KeyCode::Char(' ') | KeyCode::Enter => {
                                    app.input_automata.commit_current();
                                    let typed = app.input_automata.get_text();
                                    
                                    // 단어별 오타 검사
                                    let expected = &app.target_text;
                                    if typed.trim() != expected.trim() {
                                        // 단어가 통째로 틀렸다면 마지막 글자를 오타로 기록
                                        let last_exp = expected.chars().last().unwrap_or(' ');
                                        let last_typ = typed.chars().last().unwrap_or(' ');
                                        app.record_error(last_exp, last_typ);
                                    }
                                    
                                    // 다음 단어로 전환
                                    let has_next = app.next_word();
                                    if !has_next {
                                        // 낱말 연습 전체 완료 -> 저장
                                        app.update_elapsed_time();
                                        app.save_session_record("낱말연습", if is_korean { "한글" } else { "영어" });
                                        app.active_screen = ActiveScreen::WordPracticeMenu;
                                        app.menu_selected_idx = if is_korean { 0 } else { 1 };
                                    }
                                }
                                KeyCode::Char(c) => {
                                    let current_pos = app.input_automata.get_text().chars().count();
                                    let expected_chars: Vec<char> = app.target_text.chars().collect();
                                    let expected_char = expected_chars.get(current_pos).copied();
                                    
                                    app.input_automata.push_char(c, expected_char);
                                    
                                    // 실시간 오타 판정 (낱말 내 자모 비교)
                                    if let Some(ec) = expected_char {
                                        let typed_text = app.input_automata.get_text();
                                        if let Some(new_typed_char) = typed_text.chars().nth(current_pos) {
                                            if !hangeul::is_typing_valid(new_typed_char, ec) {
                                                app.record_error(ec, c);
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        ActiveScreen::SentencePracticeMenu => {
                            match key.code {
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
                        }
                        ActiveScreen::SentencePractice { is_korean } => {
                            app.ensure_timer_started();
                            
                            match key.code {
                                KeyCode::Esc => {
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
                                    
                                    // 문장 오타 체크 (글자수 다른 만큼 오타 누적)
                                    let exp_len = expected.chars().count();
                                    let typ_len = typed.chars().count();
                                    if typ_len < exp_len {
                                        app.total_errors += exp_len - typ_len;
                                    }
                                    
                                    // 다음 문장 전환
                                    let has_next = app.next_sentence();
                                    if !has_next {
                                        // 문장 연습 전체 완료 -> 저장
                                        app.update_elapsed_time();
                                        app.save_session_record("문장연습", if is_korean { "한글" } else { "영어" });
                                        app.active_screen = ActiveScreen::SentencePracticeMenu;
                                        app.menu_selected_idx = if is_korean { 0 } else { 1 };
                                    }
                                }
                                KeyCode::Char(c) => {
                                    let current_pos = app.input_automata.get_text().chars().count();
                                    let expected_chars: Vec<char> = app.target_text.chars().collect();
                                    let expected_char = expected_chars.get(current_pos).copied();
                                    
                                    app.input_automata.push_char(c, expected_char);
                                    
                                    // 실시간 오타 판정
                                    if let Some(ec) = expected_char {
                                        let typed_text = app.input_automata.get_text();
                                        if let Some(new_typed_char) = typed_text.chars().nth(current_pos) {
                                            if !hangeul::is_typing_valid(new_typed_char, ec) {
                                                app.record_error(ec, c);
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        ActiveScreen::Stats => {
                            if key.code == KeyCode::Esc {
                                app.active_screen = ActiveScreen::MainMenu;
                                app.menu_selected_idx = 3;
                            }
                        }

                        // ════════════════════════════════════════
                        // 게임 모드 이벤트 핸들러
                        // ════════════════════════════════════════

                        ActiveScreen::GameModeMenu => {
                            match key.code {
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
                        }

                        ActiveScreen::GameLanguageMenu { game_type } => {
                            match key.code {
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
                        }

                        ActiveScreen::GameTimeSelect { is_korean } => {
                            match key.code {
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
                                    app.active_screen = ActiveScreen::GameLanguageMenu { game_type: GameType::TimeAttack };
                                    app.menu_selected_idx = 0;
                                }
                                _ => {}
                            }
                        }

                        // ── 시간 제한 모드 ──
                        ActiveScreen::TimeAttack { is_korean } => {
                            // 시간 초과 체크
                            if app.is_time_attack_expired() {
                                app.update_elapsed_time();
                                app.save_session_record("게임-시간제한", if is_korean { "한글" } else { "영어" });
                                app.active_screen = ActiveScreen::GameOver { game_type: GameType::TimeAttack, is_korean };
                            } else {
                                match key.code {
                                    KeyCode::Esc => {
                                        app.active_screen = ActiveScreen::GameModeMenu;
                                        app.menu_selected_idx = 0;
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
                                        // 정확히 입력 완료 시 자동으로 다음 단어
                                        if app.is_target_fully_typed() {
                                            let success = app.is_current_word_success();
                                            app.time_attack_next_word(success);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        // ── 서바이벌 모드 ──
                        ActiveScreen::Survival { is_korean } => {
                            match key.code {
                                KeyCode::Esc => {
                                    app.active_screen = ActiveScreen::GameModeMenu;
                                    app.menu_selected_idx = 1;
                                }
                                KeyCode::Backspace => {
                                    app.input_automata.backspace();
                                }
                                KeyCode::Char(' ') | KeyCode::Enter => {
                                    let had_error = !app.is_current_word_success();
                                    let has_next = app.survival_next_word(had_error);
                                    if !has_next {
                                        app.update_elapsed_time();
                                        app.save_session_record("게임-서바이벌", if is_korean { "한글" } else { "영어" });
                                        app.active_screen = ActiveScreen::GameOver { game_type: GameType::Survival, is_korean };
                                    }
                                }
                                KeyCode::Char(c) => {
                                    app.push_game_char(c);
                                    // 정확히 입력 완료 시 자동 제출 (오타 없이 맞춘 경우만 성공)
                                    if app.is_target_fully_typed() {
                                        let had_error = !app.is_current_word_success();
                                        let has_next = app.survival_next_word(had_error);
                                        if !has_next {
                                            app.update_elapsed_time();
                                            app.save_session_record("게임-서바이벌", if is_korean { "한글" } else { "영어" });
                                            app.active_screen = ActiveScreen::GameOver { game_type: GameType::Survival, is_korean };
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }

                        // ── 타자 레인 모드 ──
                        ActiveScreen::TypingRain { is_korean } => {
                            if app.game_mode_lives == 0 {
                                app.update_elapsed_time();
                                app.save_session_record("게임-타자레인", if is_korean { "한글" } else { "영어" });
                                app.active_screen = ActiveScreen::GameOver { game_type: GameType::TypingRain, is_korean };
                            } else {
                                match key.code {
                                    KeyCode::Esc => {
                                        app.active_screen = ActiveScreen::GameModeMenu;
                                        app.menu_selected_idx = 2;
                                    }
                                    KeyCode::Backspace => {
                                        if app.rain_active_idx.is_some() {
                                            app.input_automata.backspace();
                                            app.update_rain_typed_progress();
                                            // 입력이 비면 타겟 선택 해제
                                            if app.input_automata.get_text().is_empty() {
                                                if let Some(idx) = app.rain_active_idx {
                                                    app.rain_words[idx].active = false;
                                                    app.rain_words[idx].typed_len = 0;
                                                }
                                                app.rain_active_idx = None;
                                                app.target_text.clear();
                                            }
                                        }
                                    }
                                    KeyCode::Char(c) => {
                                        // 활성 단어가 없으면 첫 자모가 매칭되는 단어 찾기
                                        if app.rain_active_idx.is_none() {
                                            let mut found_idx = None;
                                            for (i, word) in app.rain_words.iter().enumerate() {
                                                if word.destroyed {
                                                    continue;
                                                }
                                                let first_char = word.text.chars().next();
                                                if let Some(fc) = first_char {
                                                    let input_jamo = hangeul::map_qwerty_to_jamo(c);
                                                    let target_jamos = hangeul::fully_decompose_hangul(fc);
                                                    let matches = if let Some(ij) = input_jamo {
                                                        target_jamos.first() == Some(&ij)
                                                    } else {
                                                        // 영문: 대소문자 무시 비교
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
                                                let word_text = app.rain_words[idx].text.clone();
                                                app.target_text = word_text;
                                                app.update_automata_modes();
                                                app.push_game_char(c);
                                                app.update_rain_typed_progress();
                                            }
                                        } else {
                                            app.push_game_char(c);
                                            app.update_rain_typed_progress();

                                            // 정확히 입력했을 때만 파괴 (오타 통과 방지)
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
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        // ── 플래시 타이핑 모드 ──
                        ActiveScreen::FlashTyping { is_korean } => {
                            match key.code {
                                KeyCode::Esc => {
                                    app.active_screen = ActiveScreen::GameModeMenu;
                                    app.menu_selected_idx = 3;
                                }
                                KeyCode::Enter => {
                                    if app.flash_answer_shown {
                                        // 다음 라운드로
                                        let has_next = app.flash_next_round();
                                        if !has_next {
                                            app.update_elapsed_time();
                                            app.save_session_record("게임-플래시타이핑", if is_korean { "한글" } else { "영어" });
                                            app.active_screen = ActiveScreen::GameOver { game_type: GameType::FlashTyping, is_korean };
                                        }
                                    } else if !app.flash_visible {
                                        // 정답 제출
                                        app.flash_submit_answer();
                                    }
                                    // 표시 중 Enter: 즉시 숨기고 입력 단계로 (조기 진행 허용)
                                    else {
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
                                        // 표시 중에도 입력 시작 가능 (동시에 단어 숨김)
                                        if app.flash_visible {
                                            app.flash_visible = false;
                                        }
                                        app.push_game_char(c);
                                    }
                                }
                                _ => {}
                            }
                        }

                        // ── 데일리 챌린지 모드 ──
                        ActiveScreen::DailyChallenge { is_korean } => {
                            match key.code {
                                KeyCode::Esc => {
                                    app.active_screen = ActiveScreen::GameModeMenu;
                                    app.menu_selected_idx = 4;
                                }
                                KeyCode::Backspace => {
                                    app.input_automata.backspace();
                                }
                                KeyCode::Char(' ') | KeyCode::Enter => {
                                    app.input_automata.commit_current();
                                    let has_next = app.daily_next_word();
                                    if !has_next {
                                        app.update_elapsed_time();
                                        app.save_session_record("게임-데일리챌린지", if is_korean { "한글" } else { "영어" });
                                        app.active_screen = ActiveScreen::GameOver { game_type: GameType::DailyChallenge, is_korean };
                                    }
                                }
                                KeyCode::Char(c) => {
                                    app.push_game_char(c);
                                    if app.is_target_fully_typed() {
                                        app.input_automata.commit_current();
                                        let has_next = app.daily_next_word();
                                        if !has_next {
                                            app.update_elapsed_time();
                                            app.save_session_record("게임-데일리챌린지", if is_korean { "한글" } else { "영어" });
                                            app.active_screen = ActiveScreen::GameOver { game_type: GameType::DailyChallenge, is_korean };
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }

                        // ── 긴 글 레이스 메뉴 ──
                        ActiveScreen::LongTextRaceMenu { is_korean } => {
                            let max_idx = App::get_long_text_titles(is_korean).len().saturating_sub(1);
                            match key.code {
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
                                    app.active_screen = ActiveScreen::LongTextRace { is_korean, text_idx: idx };
                                }
                                KeyCode::Esc => {
                                    app.active_screen = ActiveScreen::GameLanguageMenu { game_type: GameType::LongTextRace };
                                    app.menu_selected_idx = 0;
                                }
                                _ => {}
                            }
                        }

                        // ── 긴 글 레이스 실제 연습 ──
                        ActiveScreen::LongTextRace { is_korean, text_idx } => {
                            app.ensure_timer_started();

                            match key.code {
                                KeyCode::Esc => {
                                    app.active_screen = ActiveScreen::LongTextRaceMenu { is_korean };
                                    app.menu_selected_idx = text_idx;
                                }
                                KeyCode::Backspace => {
                                    app.input_automata.backspace();
                                }
                                KeyCode::Enter => {
                                    app.input_automata.commit_current();
                                    let has_next = app.long_text_next_paragraph();
                                    if !has_next {
                                        app.update_elapsed_time();
                                        app.save_session_record("게임-긴글레이스", if is_korean { "한글" } else { "영어" });
                                        app.active_screen = ActiveScreen::GameOver { game_type: GameType::LongTextRace, is_korean };
                                    }
                                }
                                KeyCode::Char(c) => {
                                    app.push_game_char(c);
                                    // 현재 문단을 정확히 다 치면 Enter 없이 다음 문단으로
                                    if app.is_target_fully_typed() {
                                        app.input_automata.commit_current();
                                        let has_next = app.long_text_next_paragraph();
                                        if !has_next {
                                            app.update_elapsed_time();
                                            app.save_session_record("게임-긴글레이스", if is_korean { "한글" } else { "영어" });
                                            app.active_screen = ActiveScreen::GameOver { game_type: GameType::LongTextRace, is_korean };
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }

                        // ── 게임 오버 화면 ──
                        ActiveScreen::GameOver { game_type, is_korean } => {
                            match key.code {
                                KeyCode::Enter => {
                                    // 다시 하기
                                    match game_type {
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
                                            app.active_screen = ActiveScreen::LongTextRace { is_korean, text_idx: idx };
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
                        }
                    }
                }
            }
        }
        
        // 실시간 타이머 흐름 업데이트
        app.update_elapsed_time();

        // 게임 모드별 틱 처리
        match app.active_screen {
            ActiveScreen::TimeAttack { is_korean } => {
                if app.is_time_attack_expired() {
                    app.update_elapsed_time();
                    app.save_session_record("게임-시간제한", if is_korean { "한글" } else { "영어" });
                    app.active_screen = ActiveScreen::GameOver { game_type: GameType::TimeAttack, is_korean };
                }
            }
            ActiveScreen::TypingRain { is_korean } => {
                app.rain_screen_width = terminal.size()?.width;
                app.rain_screen_height = terminal.size()?.height.saturating_sub(10);
                app.tick_rain();
                if app.game_mode_lives == 0 {
                    app.update_elapsed_time();
                    app.save_session_record("게임-타자레인", if is_korean { "한글" } else { "영어" });
                    app.active_screen = ActiveScreen::GameOver { game_type: GameType::TypingRain, is_korean };
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
}
