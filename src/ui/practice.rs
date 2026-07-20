use crate::app::App;
use crate::hangeul;
use crate::ui::keyboard::KeyboardWidget;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub(super) fn draw_finger_practice(f: &mut Frame, area: Rect, app: &App, level: usize, is_korean: bool) {
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
    let text_spans = super::game_typed_spans(&typed, expected);

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(text_spans));
    lines.push(Line::from(""));
    lines.push(super::make_stats_bar(app));

    let info_p = Paragraph::new(lines)
        .block(info_block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(info_p, main_layout[0]);

    // 2. 하단 키보드 위젯 — 전체 문자열 자모 시퀀스 기준 다음 키
    let actual_target_char = hangeul::next_input_unit(&typed, expected);
    let keyboard_widget = KeyboardWidget::new(actual_target_char, is_korean);
    f.render_widget(keyboard_widget, main_layout[1]);
}

pub(super) fn draw_word_practice(f: &mut Frame, area: Rect, app: &App, is_korean: bool) {
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
        Span::styled(current_word.clone(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]));
    
    // 사용자가 실제 입력한 값을 정오 색상으로 표시
    if typed.is_empty() {
        let mut input_line = vec![Span::styled(" 나의 입력: ", Style::default().fg(Color::Gray))];
        input_line.push(Span::styled(
            " [입력을 기다리는 중...]",
            Style::default().fg(Color::DarkGray),
        ));
        lines.push(Line::from(input_line));
    } else {
        let mut input_line = vec![Span::styled(" 나의 입력: ", Style::default().fg(Color::Gray))];
        input_line.extend(super::typed_input_spans(&typed, current_word));
        lines.push(Line::from(input_line));
    }
    
    lines.push(Line::from(""));
    
    // 다음 단어 힌트
    lines.push(Line::from(vec![
        Span::styled(" 다음 단어: ", Style::default().fg(Color::DarkGray)),
        Span::styled(next_word.to_string(), Style::default().fg(Color::DarkGray)),
        Span::styled(format!("  ({} / {})", app.current_word_idx + 1, app.word_list.len()), Style::default().fg(Color::Magenta)),
    ]));
    
    lines.push(Line::from(""));
    lines.push(super::make_stats_bar(app));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, layout[1]);
}

// --- 문장 연습 모드 화면 렌더링 ---
pub(super) fn draw_sentence_practice(f: &mut Frame, area: Rect, app: &App, is_korean: bool) {
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
    
    // 사용자가 실제 입력한 값에 정오 색상 표시
    lines.push(Line::from(Span::styled(" [내 입력]", Style::default().fg(Color::DarkGray))));
    if typed.is_empty() {
        lines.push(Line::from(Span::styled(
            " [입력을 기다리는 중...]",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        let typed_spans = super::typed_input_spans(&typed, current_sentence);
        lines.push(Line::from(typed_spans));
    }
    lines.push(Line::from(""));
    lines.push(super::make_stats_bar(app));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, layout[1]);
}

pub(super) fn draw_stats(f: &mut Frame, area: Rect, app: &App) {
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

