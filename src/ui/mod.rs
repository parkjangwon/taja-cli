pub mod keyboard;
pub mod menu;
pub mod practice;
pub mod game;

use crate::app::{App, ActiveScreen};
use crate::hangeul;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.size());

    draw_header(f, chunks[0]);

    match &app.active_screen {
        ActiveScreen::MainMenu => menu::draw_main_menu(f, chunks[1], app),
        ActiveScreen::FingerPracticeMenu => menu::draw_finger_language_menu(f, chunks[1], app),
        ActiveScreen::FingerPracticeLevelMenu { is_korean } => menu::draw_finger_level_menu(f, chunks[1], app, *is_korean),
        ActiveScreen::FingerPractice { level, is_korean } => practice::draw_finger_practice(f, chunks[1], app, *level, *is_korean),
        ActiveScreen::WordPracticeMenu => menu::draw_word_menu(f, chunks[1], app),
        ActiveScreen::WordPractice { is_korean } => practice::draw_word_practice(f, chunks[1], app, *is_korean),
        ActiveScreen::SentencePracticeMenu => menu::draw_sentence_menu(f, chunks[1], app),
        ActiveScreen::SentencePractice { is_korean } => practice::draw_sentence_practice(f, chunks[1], app, *is_korean),
        ActiveScreen::Stats => practice::draw_stats(f, chunks[1], app),
        ActiveScreen::GameModeMenu => menu::draw_game_mode_menu(f, chunks[1], app),
        ActiveScreen::GameLanguageMenu { game_type } => menu::draw_game_language_menu(f, chunks[1], app, *game_type),
        ActiveScreen::GameTimeSelect { is_korean } => menu::draw_time_select_menu(f, chunks[1], app, *is_korean),
        ActiveScreen::TimeAttack { is_korean } => game::draw_time_attack(f, chunks[1], app, *is_korean),
        ActiveScreen::Survival { is_korean } => game::draw_survival(f, chunks[1], app, *is_korean),
        ActiveScreen::TypingRain { is_korean } => game::draw_typing_rain(f, chunks[1], app, *is_korean),
        ActiveScreen::FlashTyping { is_korean } => game::draw_flash_typing(f, chunks[1], app, *is_korean),
        ActiveScreen::DailyChallenge { is_korean } => game::draw_daily_challenge(f, chunks[1], app, *is_korean),
        ActiveScreen::LongTextRaceMenu { is_korean } => menu::draw_long_text_menu(f, chunks[1], app, *is_korean),
        ActiveScreen::LongTextRace { is_korean, text_idx } => game::draw_long_text_race(f, chunks[1], app, *is_korean, *text_idx),
        ActiveScreen::GameOver { game_type, is_korean } => game::draw_game_over(f, chunks[1], app, *game_type, *is_korean),
    }

    draw_footer(f, chunks[2], app);
}

/// 자모 접두사 기반 정오 하이라이트 Span 생성 (연습/게임 공용)
/// target 텍스트에 색상을 입혀서 반환 (맞은 부분=초록, 조합 중=노랑 배경, 틀린 부분=빨강)
pub(super) fn game_typed_spans(typed: &str, target: &str) -> Vec<Span<'static>> {
    let prefix_ok = hangeul::is_input_prefix_of(typed, target);
    let matched = hangeul::fully_matched_chars(typed, target);
    let typed_char_count = typed.chars().count();
    let mut text_spans = Vec::new();

    for (i, target_char) in target.chars().enumerate() {
        if i < matched {
            text_spans.push(Span::styled(
                target_char.to_string(),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ));
        } else if prefix_ok && i == matched {
            // 조합 중이거나 다음에 칠 글자 (커서)
            text_spans.push(Span::styled(
                target_char.to_string(),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ));
        } else if !prefix_ok && i < typed_char_count {
            text_spans.push(Span::styled(
                target_char.to_string(),
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::UNDERLINED),
            ));
        } else {
            text_spans.push(Span::styled(
                target_char.to_string(),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }
    text_spans
}

/// 사용자가 입력한 텍스트를 정오에 따라 색상화 (입력값 기준)
/// 각 문자를 원문의 같은 위치 문자와 비교하여 초록(일치)/빨강(불일치) 표시
pub(super) fn typed_input_spans(typed: &str, target: &str) -> Vec<Span<'static>> {
    let typed_chars: Vec<char> = typed.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let mut spans = Vec::new();

    for (i, tc) in typed_chars.iter().enumerate() {
        if i < target_chars.len() && *tc == target_chars[i] {
            spans.push(Span::styled(
                tc.to_string(),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(
                tc.to_string(),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ));
        }
    }
    // 커서 표시 (입력이 완전히 끝나지 않았음을 암시)
    if typed_chars.len() >= target_chars.len() {
        spans.push(Span::styled(
            " ",
            Style::default().fg(Color::White),
        ));
    } else {
        spans.push(Span::styled(
            "▌",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::SLOW_BLINK),
        ));
    }
    spans
}

fn draw_header(f: &mut Frame, area: Rect) {
    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Magenta))
        .style(Style::default().bg(Color::Reset));

    let logo = Span::styled(
        " ⌨️ TAJA-CLI :: 한국인을 위한 TUI 타자 연습기 ",
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    );

    let info = Span::styled(
        format!(" v{} ", env!("CARGO_PKG_VERSION")),
        Style::default().fg(Color::DarkGray),
    );

    let header_paragraph = Paragraph::new(Line::from(vec![logo, info]))
        .block(header_block)
        .style(Style::default().fg(Color::White));

    f.render_widget(header_paragraph, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let mut footer_spans = Vec::new();

    let guide_text = match app.active_screen {
        ActiveScreen::MainMenu => " [↑/↓]: 이동  [Enter]: 선택  [Esc/Q]: 종료 ",
        ActiveScreen::FingerPracticeMenu | ActiveScreen::FingerPracticeLevelMenu { .. } | ActiveScreen::WordPracticeMenu | ActiveScreen::SentencePracticeMenu => {
            " [↑/↓]: 이동  [Enter]: 선택  [Esc]: 이전 화면 "
        }
        ActiveScreen::FingerPractice { .. } => {
            " [Esc]: 연습 중단 (기록 저장 X)  [아무 키나 입력해 시작] "
        }
        ActiveScreen::WordPractice { .. } => {
            " [Esc]: 연습 중단  [Space]: 다음 단어로 전환  [아무 키나 입력해 시작] "
        }
        ActiveScreen::SentencePractice { .. } => {
            " [Esc]: 연습 중단  [Enter]: 줄 바꿈 (다음 문장)  [아무 키나 입력해 시작] "
        }
        ActiveScreen::Stats => " [Esc]: 메인 메뉴로 돌아가기 ",
        // 게임 모드 푸터
        ActiveScreen::GameModeMenu | ActiveScreen::GameLanguageMenu { .. } | ActiveScreen::GameTimeSelect { .. } => {
            " [↑/↓]: 이동  [Enter]: 선택  [Esc]: 이전 화면 "
        }
        ActiveScreen::TimeAttack { .. } | ActiveScreen::Survival { .. } | ActiveScreen::DailyChallenge { .. } => {
            " [Esc]: 입력 초기화 (비어 있으면 중단)  [Space/Enter]: 단어 제출 "
        }
        ActiveScreen::TypingRain { .. } => {
            " [Esc]: 입력 초기화 (비어 있으면 중단)  타이핑으로 단어 파괴! "
        }
        ActiveScreen::FlashTyping { .. } => {
            " [Esc]: 입력 초기화 (비어 있으면 중단)  [Enter]: 정답 제출 / 다음 라운드 "
        }
        ActiveScreen::LongTextRaceMenu { .. } => {
            " [↑/↓]: 이동  [Enter]: 선택  [Esc]: 이전 화면 "
        }
        ActiveScreen::LongTextRace { .. } => {
            " [Esc]: 입력 초기화 (비어 있으면 중단)  [Enter]: 정확히 입력한 줄만 다음으로 "
        }
        ActiveScreen::GameOver { .. } => {
            " [Enter]: 다시 하기  [Esc]: 게임 모드 메뉴로 "
        }
    };

    footer_spans.push(Span::styled(guide_text, Style::default().fg(Color::Gray)));

    // 타자 연습 중일 때 푸터에 콤팩트한 붉은색 경고 추가
    match app.active_screen {
        ActiveScreen::FingerPractice { .. } | ActiveScreen::WordPractice { .. } | ActiveScreen::SentencePractice { .. }
        | ActiveScreen::TimeAttack { .. } | ActiveScreen::Survival { .. } | ActiveScreen::TypingRain { .. }
        | ActiveScreen::FlashTyping { .. } | ActiveScreen::DailyChallenge { .. }
        | ActiveScreen::LongTextRace { .. } => {
            footer_spans.push(Span::styled(
                " (⚠️ OS 입력기: 영문 필수) ",
                Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD),
            ));
        }
        _ => {}
    }

    let footer_paragraph = Paragraph::new(Line::from(footer_spans)).block(footer_block);
    f.render_widget(footer_paragraph, area);
}

// --- 공통: 실시간 통계 바 렌더링 헬퍼 ---
pub(super) fn make_stats_bar(app: &App) -> Line<'_> {
    Line::from(vec![
        Span::styled(" 진행시간: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:02}초", app.elapsed_time.as_secs()), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled("  |  분당타수(CPM): ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{} 타", app.get_cpm()), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled("  |  정확도: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:.1}%", app.get_accuracy()), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    ])
}

