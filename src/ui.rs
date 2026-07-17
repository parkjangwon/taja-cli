pub mod keyboard;

use crate::app::{App, ActiveScreen};
use crate::ui::keyboard::KeyboardWidget;
use crate::hangeul;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    // 1. 전체 화면을 헤더(Header), 본문(Body), 푸터(Footer)로 분할
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 헤더
            Constraint::Min(10),   // 본문
            Constraint::Length(3), // 푸터
        ])
        .split(f.size());

    // 2. 헤더 그리기
    draw_header(f, chunks[0]);

    // 3. 본문 그리기 (화면 상태에 따라 다름)
    match &app.active_screen {
        ActiveScreen::MainMenu => draw_main_menu(f, chunks[1], app),
        ActiveScreen::FingerPracticeMenu => draw_finger_language_menu(f, chunks[1], app),
        ActiveScreen::FingerPracticeLevelMenu { is_korean } => draw_finger_level_menu(f, chunks[1], app, *is_korean),
        ActiveScreen::FingerPractice { level, is_korean } => draw_finger_practice(f, chunks[1], app, *level, *is_korean),
        ActiveScreen::WordPracticeMenu => draw_word_menu(f, chunks[1], app),
        ActiveScreen::WordPractice { is_korean } => draw_word_practice(f, chunks[1], app, *is_korean),
        ActiveScreen::SentencePracticeMenu => draw_sentence_menu(f, chunks[1], app),
        ActiveScreen::SentencePractice { is_korean } => draw_sentence_practice(f, chunks[1], app, *is_korean),
        ActiveScreen::Stats => draw_stats(f, chunks[1], app),
    }

    // 4. 푸터 그리기
    draw_footer(f, chunks[2], app);
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
        " v0.1.0 ",
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
    };

    footer_spans.push(Span::styled(guide_text, Style::default().fg(Color::Gray)));

    // 타자 연습 중일 때 푸터에 콤팩트한 붉은색 경고 추가
    match app.active_screen {
        ActiveScreen::FingerPractice { .. } | ActiveScreen::WordPractice { .. } | ActiveScreen::SentencePractice { .. } => {
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

// --- 메인 메뉴 렌더링 ---
fn draw_main_menu(f: &mut Frame, area: Rect, app: &App) {
    let menu_items = vec![
        "1. 자리 연습 (Finger Placement Practice)",
        "2. 낱말 연습 (Word Practice)",
        "3. 문장 연습 (Sentence Practice)",
        "4. 통계 및 기록 분석 (Statistics)",
        "5. 종료 (Exit)",
    ];

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // 왼쪽 여백
            Constraint::Percentage(50), // 메뉴판 (가로폭 확장으로 잘림 방지)
            Constraint::Percentage(25), // 오른쪽 여백
        ])
        .split(area);

    let menu_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(" MAIN MENU ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "연습하실 모드를 선택하세요.",
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "💡 OS 입력기 상태를 '영문'으로 설정해 주세요. (한글 결합기 내장)",
        Style::default().fg(Color::Yellow),
    )));
    lines.push(Line::from(""));

    for (idx, item) in menu_items.iter().enumerate() {
        if idx == app.menu_selected_idx {
            lines.push(Line::from(Span::styled(
                format!(" ➔ {}", item),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                format!("   {}", item),
                Style::default().fg(Color::White),
            )));
        }
        lines.push(Line::from(""));
    }

    let paragraph = Paragraph::new(lines)
        .block(menu_block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, layout[1]);
}

// --- 자리 연습 언어 선택 메뉴 렌더링 ---
fn draw_finger_language_menu(f: &mut Frame, area: Rect, app: &App) {
    let options = vec![
        "1. 한글 자리 연습 (Korean Finger Practice)",
        "2. 영어 자리 연습 (English Finger Practice)",
    ];

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(" FINGER PLACEMENT ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from("연습하실 언어를 선택하세요."));
    lines.push(Line::from(""));

    for (idx, item) in options.iter().enumerate() {
        if idx == app.menu_selected_idx {
            lines.push(Line::from(Span::styled(
                format!(" ➔ {}", item),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                format!("   {}", item),
                Style::default().fg(Color::White),
            )));
        }
        lines.push(Line::from(""));
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .style(Style::default().fg(Color::White))
        .alignment(ratatui::layout::Alignment::Center);

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Percentage(35),
        ])
        .split(layout[1]);

    f.render_widget(paragraph, vertical_layout[1]);
}

// --- 자리 연습 단계 선택 메뉴 렌더링 ---
fn draw_finger_level_menu(f: &mut Frame, area: Rect, app: &App, is_korean: bool) {
    let k_levels = vec![
        "Level 1: 기본 자리 연습 (ㅁㄴㅇㄹ ㅓㅏㅣ;)",
        "Level 2: 검지 확장 연습 (ㅎ, ㅗ, ㅜ 추가)",
        "Level 3: 윗줄 자판 연습 (ㅂㅈㄷㄱㅅ ㅛㅕㅑㅐㅔ)",
        "Level 4: 아랫줄 자판 연습 (ㅋㅌㅊㅍ ㅠㅡ ,./)",
        "Level 5: 숫자 및 기호 연습 (1234567890 ...)",
        "Level 6: 쌍자음 및 복합 모음 (ㅃㅉㄸㄲㅆ ㅒㅖ)",
    ];

    let e_levels = vec![
        "Level 1: 기본 자리 연습 (Home Row - asdf jkl;)",
        "Level 2: 윗줄 확장 연습 (Top Row - qwer uiop)",
        "Level 3: 아랫줄 확장 연습 (Bottom Row - zxcv m,./)",
        "Level 4: 대소문자 혼합 연습 (Shift Mixed)",
        "Level 5: 숫자 및 기호 연습 (12345... !@#$)",
    ];

    let levels = if is_korean { &k_levels } else { &e_levels };

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(area);

    let title_str = if is_korean { " KOREAN FINGER PLACEMENT " } else { " ENGLISH FINGER PLACEMENT " };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(title_str)
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from("연습할 자판 단계를 선택하세요."));
    lines.push(Line::from(""));

    for (idx, item) in levels.iter().enumerate() {
        if idx == app.menu_selected_idx {
            lines.push(Line::from(Span::styled(
                format!(" ➔ {}", item),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                format!("   {}", item),
                Style::default().fg(Color::White),
            )));
        }
        lines.push(Line::from(""));
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, layout[1]);
}

// --- 낱말 연습 메뉴 렌더링 ---
fn draw_word_menu(f: &mut Frame, area: Rect, app: &App) {
    let items = vec![
        "1. 한글 낱말 연습 (Korean Word Practice)",
        "2. 영어 낱말 연습 (English Word Practice - Easter Eggs Included!)",
    ];

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(" WORD PRACTICE ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from("연습할 언어를 선택해 주세요."));
    lines.push(Line::from(""));

    for (idx, item) in items.iter().enumerate() {
        if idx == app.menu_selected_idx {
            lines.push(Line::from(Span::styled(
                format!(" ➔ {}", item),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                format!("   {}", item),
                Style::default().fg(Color::White),
            )));
        }
        lines.push(Line::from(""));
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, layout[1]);
}

// --- 문장 연습 메뉴 렌더링 ---
fn draw_sentence_menu(f: &mut Frame, area: Rect, app: &App) {
    let items = vec![
        "1. 한글 문장 연습 (Korean Sentence Practice)",
        "2. 영어 문장 연습 (English Sentence Practice)",
    ];

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(" SENTENCE PRACTICE ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from("연습할 언어를 선택해 주세요."));
    lines.push(Line::from(""));

    for (idx, item) in items.iter().enumerate() {
        if idx == app.menu_selected_idx {
            lines.push(Line::from(Span::styled(
                format!(" ➔ {}", item),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                format!("   {}", item),
                Style::default().fg(Color::White),
            )));
        }
        lines.push(Line::from(""));
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, layout[1]);
}

// --- 공통: 실시간 통계 바 렌더링 헬퍼 ---
fn make_stats_bar(app: &App) -> Line<'_> {
    Line::from(vec![
        Span::styled(" 진행시간: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:02}초", app.elapsed_time.as_secs()), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled("  |  분당타수(CPM): ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{} 타", app.get_cpm()), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled("  |  정확도: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:.1}%", app.get_accuracy()), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    ])
}

// --- 자리 연습 모드 화면 렌더링 ---
fn draw_finger_practice(f: &mut Frame, area: Rect, app: &App, level: usize, is_korean: bool) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // 상단 타자판 및 입력 정보
            Constraint::Min(10),   // 하단 키보드 레이아웃
        ])
        .split(area);

    // 1. 상단 정보 패널
    let lang_title = if is_korean { "한글 자리 연습" } else { "영어 자리 연습" };
    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!(" {} :: Level {} ", lang_title, level))
        .border_style(Style::default().fg(Color::Green));

    let typed = app.input_automata.get_text();
    let expected = &app.target_text;

    // 타자 정오 대조 Span 빌드
    let mut text_spans = Vec::new();
    for (i, target_char) in expected.chars().enumerate() {
        let typed_char_opt = typed.chars().nth(i);
        
        match typed_char_opt {
            Some(tc) => {
                if hangeul::is_typing_valid(tc, target_char) {
                    // 맞았으면 녹색
                    text_spans.push(Span::styled(target_char.to_string(), Style::default().fg(Color::Green)));
                } else {
                    // 틀렸으면 빨간색에 밑줄
                    text_spans.push(Span::styled(
                        target_char.to_string(),
                        Style::default().fg(Color::Red).add_modifier(Modifier::UNDERLINED),
                    ));
                }
            }
            None => {
                // 아직 입력하지 않은 글자
                if i == typed.chars().count() {
                    // 현재 입력 포커스 위치 (깜빡이 효과 대신 검은색 배경에 노란 글씨)
                    text_spans.push(Span::styled(
                        target_char.to_string(),
                        Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    text_spans.push(Span::styled(target_char.to_string(), Style::default().fg(Color::DarkGray)));
                }
            }
        }
    }

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(text_spans));
    lines.push(Line::from(""));
    lines.push(make_stats_bar(app));

    let info_p = Paragraph::new(lines)
        .block(info_block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(info_p, main_layout[0]);

    // 2. 하단 키보드 위젯 렌더링
    // 다음에 입력해야 하는 타깃 자모 찾기
    let current_pos = typed.chars().count();
    let next_target_char = expected.chars().nth(current_pos);
    
    // 타깃 글자가 완성형 한글인 경우, 조합 상태에 맞추어 실제 입력해야 할 다음 자모 추출
    let actual_target_char = match next_target_char {
        Some(c) => {
            let code = c as u32;
            if (0xAC00..=0xD7A3).contains(&code) {
                let target_jamos = hangeul::fully_decompose_hangul(c);
                let current_composed_char = app.input_automata.get_current_char();
                let current_jamos = match current_composed_char {
                    Some(cc) => hangeul::fully_decompose_hangul(cc),
                    None => Vec::new(),
                };
                
                let next_jamo_idx = current_jamos.len();
                if next_jamo_idx < target_jamos.len() {
                    Some(target_jamos[next_jamo_idx])
                } else {
                    Some(c)
                }
            } else {
                Some(c)
            }
        }
        None => None,
    };
    
    let keyboard_widget = KeyboardWidget::new(actual_target_char, is_korean);
    f.render_widget(keyboard_widget, main_layout[1]);
}

// --- 낱말 연습 모드 화면 렌더링 ---
fn draw_word_practice(f: &mut Frame, area: Rect, app: &App, is_korean: bool) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(format!(" 낱말 연습 ({}) ", if is_korean { "한글" } else { "영어" }))
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Green));

    let current_word = &app.target_text;
    let next_word = if app.current_word_idx + 1 < app.word_list.len() {
        &app.word_list[app.current_word_idx + 1]
    } else {
        "없음"
    };

    let typed = app.input_automata.get_text();

    // 단어 정오 렌더링
    let mut text_spans = Vec::new();
    for (i, target_char) in current_word.chars().enumerate() {
        let typed_char_opt = typed.chars().nth(i);
        match typed_char_opt {
            Some(tc) => {
                if tc == target_char {
                    text_spans.push(Span::styled(target_char.to_string(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)));
                } else {
                    text_spans.push(Span::styled(
                        target_char.to_string(),
                        Style::default().fg(Color::Red).add_modifier(Modifier::UNDERLINED),
                    ));
                }
            }
            None => {
                text_spans.push(Span::styled(target_char.to_string(), Style::default().fg(Color::White)));
            }
        }
    }

    // 개발자 이스터에그 단어 강조 (재미용)
    let is_easter_egg = !is_korean && vec![
        "String", "Array", "Java", "JavaScript", "SQL", "nullptr", "async", "await",
        "struct", "impl", "let_mut", "panic!", "unwrap", "println!", "Option", "Result",
        "git_commit", "stack_overflow"
    ].contains(&current_word.as_str());

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    if is_easter_egg {
        lines.push(Line::from(Span::styled(
            "⚡ DEV EASTER EGG DETECTED! ⚡",
            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        )));
    } else {
        lines.push(Line::from(""));
    }
    
    // 목표 단어 출력
    lines.push(Line::from(vec![
        Span::styled(" 제시 단어: ", Style::default().fg(Color::Gray)),
        Span::styled(current_word, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]));
    
    // 입력 중인 단어 출력
    lines.push(Line::from(vec![
        Span::styled(" 나의 입력: ", Style::default().fg(Color::Gray)),
        Span::styled(typed, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]));
    
    lines.push(Line::from(""));
    
    // 다음 단어 힌트
    lines.push(Line::from(vec![
        Span::styled(" 다음 단어: ", Style::default().fg(Color::DarkGray)),
        Span::styled(next_word, Style::default().fg(Color::DarkGray)),
        Span::styled(format!("  ({} / {})", app.current_word_idx + 1, app.word_list.len()), Style::default().fg(Color::Magenta)),
    ]));
    
    lines.push(Line::from(""));
    lines.push(make_stats_bar(app));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, layout[1]);
}

// --- 문장 연습 모드 화면 렌더링 ---
fn draw_sentence_practice(f: &mut Frame, area: Rect, app: &App, is_korean: bool) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(format!(" 문장 연습 ({}) ", if is_korean { "한글" } else { "영어" }))
        .border_style(Style::default().fg(Color::Green));

    let current_sentence = &app.target_text;
    let typed = app.input_automata.get_text();

    // 문장 정오 렌더링
    let mut text_spans = Vec::new();
    for (i, target_char) in current_sentence.chars().enumerate() {
        let typed_char_opt = typed.chars().nth(i);
        match typed_char_opt {
            Some(tc) => {
                if tc == target_char {
                    text_spans.push(Span::styled(target_char.to_string(), Style::default().fg(Color::Green)));
                } else {
                    text_spans.push(Span::styled(
                        target_char.to_string(),
                        Style::default().fg(Color::Red).add_modifier(Modifier::UNDERLINED),
                    ));
                }
            }
            None => {
                if i == typed.chars().count() {
                    text_spans.push(Span::styled(
                        target_char.to_string(),
                        Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    text_spans.push(Span::styled(target_char.to_string(), Style::default().fg(Color::White)));
                }
            }
        }
    }

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!(" [ {} / {} 문장 ]", app.current_sentence_idx + 1, app.sentence_list.len()),
        Style::default().fg(Color::Magenta),
    )));
    lines.push(Line::from(""));
    
    // 원문 문장 출력
    lines.push(Line::from(Span::styled(" [원문]", Style::default().fg(Color::DarkGray))));
    lines.push(Line::from(Span::styled(current_sentence, Style::default().fg(Color::Cyan))));
    lines.push(Line::from(""));
    
    // 사용자가 입력한 대조 문장 출력
    lines.push(Line::from(Span::styled(" [대조]", Style::default().fg(Color::DarkGray))));
    lines.push(Line::from(text_spans));
    lines.push(Line::from(""));
    lines.push(make_stats_bar(app));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, layout[1]);
}

// --- 통계 및 기록 분석 화면 렌더링 ---
fn draw_stats(f: &mut Frame, area: Rect, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // 최근 연습 이력 리스트
            Constraint::Percentage(50), // 자주 틀리는 키 탑 리스트
        ])
        .split(area);

    // 1. 왼쪽 최근 연습 이력 패널
    let history_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" 최근 연습 기록 (최신 10개) ")
        .border_style(Style::default().fg(Color::Cyan));

    let history = app.storage.load_history();
    let mut history_lines = Vec::new();
    history_lines.push(Line::from(""));

    if history.records.is_empty() {
        history_lines.push(Line::from(" 저장된 타자 연습 기록이 없습니다."));
        history_lines.push(Line::from(" 연습을 완료하면 이곳에 통계가 표시됩니다."));
    } else {
        // 최신 기록이 먼저 보이도록 역순으로 10개 표시
        for record in history.records.iter().rev().take(10) {
            let short_date = record.date.chars().take(16).collect::<String>().replace("T", " ");
            
            let line = Line::from(vec![
                Span::styled(format!(" {} ", short_date), Style::default().fg(Color::DarkGray)),
                Span::styled(format!(" [{}] ", record.mode), Style::default().fg(Color::Magenta)),
                Span::styled(format!("CPM: {}타", record.cpm), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" (정확도: {:.1}%)", record.accuracy), Style::default().fg(Color::Green)),
            ]);
            history_lines.push(line);
            history_lines.push(Line::from(""));
        }
    }

    let history_p = Paragraph::new(history_lines)
        .block(history_block)
        .wrap(Wrap { trim: true });
    f.render_widget(history_p, main_layout[0]);

    // 2. 오른쪽 자주 틀리는 키 분석 패널
    let error_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" 취약 키 진단 (많이 틀린 키 탑 10) ")
        .border_style(Style::default().fg(Color::Cyan));

    let frequent_errors = &app.cached_frequent_errors;
    let mut error_lines = Vec::new();
    error_lines.push(Line::from(""));

    if frequent_errors.is_empty() {
        error_lines.push(Line::from(" 아직 오타 분석 정보가 없습니다."));
        error_lines.push(Line::from(" 틀리기 쉬운 자판을 자동으로 수집해 분석해 드립니다."));
    } else {
        error_lines.push(Line::from("  오타율이 높은 순위:"));
        error_lines.push(Line::from(""));
        
        for (rank, (key, count)) in frequent_errors.iter().enumerate() {
            let key_display = if *key == ' ' {
                "Space (공백)".to_string()
            } else {
                key.to_string()
            };
            
            let rank_color = match rank {
                0 => Color::Red,
                1 => Color::LightRed,
                2 => Color::Yellow,
                _ => Color::White,
            };

            let line = Line::from(vec![
                Span::styled(format!("   {}위 :  ", rank + 1), Style::default().fg(rank_color).add_modifier(Modifier::BOLD)),
                Span::styled(format!("'{}'", key_display), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(format!("  ➔ {}회 오타 기록됨", count), Style::default().fg(Color::Gray)),
            ]);
            error_lines.push(line);
            error_lines.push(Line::from(""));
        }
    }

    let error_p = Paragraph::new(error_lines)
        .block(error_block)
        .wrap(Wrap { trim: true });
    f.render_widget(error_p, main_layout[1]);
}
