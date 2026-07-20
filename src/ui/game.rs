use crate::app::{App, GameType};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

pub(super) fn draw_time_attack(f: &mut Frame, area: Rect, app: &App, is_korean: bool) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 타이머 바
            Constraint::Min(8),    // 메인 영역
        ])
        .split(area);

    // 1. 타이머 바
    let remaining = app.time_attack_remaining_secs();
    let total = app.game_time_limit_secs as f64;
    let ratio = (remaining / total).clamp(0.0, 1.0);
    let timer_color = if ratio > 0.5 {
        Color::Green
    } else if ratio > 0.2 {
        Color::Yellow
    } else {
        Color::Red
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(timer_color)).title(" ⏱️ 남은 시간 "))
        .gauge_style(Style::default().fg(timer_color))
        .ratio(ratio)
        .label(format!("{:.1}초", remaining));
    f.render_widget(gauge, main_layout[0]);

    // 2. 메인 게임 영역
    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(main_layout[1]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(format!(" ⏱️ 시간 제한 모드 ({}) ", if is_korean { "한글" } else { "영어" }))
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Yellow));

    let typed = app.input_automata.get_text();
    let current_word = &app.target_text;
    let text_spans = super::game_typed_spans(&typed, current_word);

    // 콤보 표시
    let combo_text = if app.game_mode_combo > 1 {
        format!("🔥 {}콤보! (x{} 배수)", app.game_mode_combo, std::cmp::min(app.game_mode_combo, 5))
    } else {
        String::new()
    };

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("🎯 점수: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}", app.game_mode_score), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled("  |  ✅ 완료: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}개", app.game_words_correct), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(""));
    if !combo_text.is_empty() {
        lines.push(Line::from(Span::styled(
            combo_text,
            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        )));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(" 제시 단어: ", Style::default().fg(Color::Gray)),
        Span::styled(current_word.clone(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(vec![
        Span::styled(" 나의 입력: ", Style::default().fg(Color::Gray)),
    ]));
    lines.push(Line::from(text_spans));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, content_layout[1]);
}

pub(super) fn draw_survival(f: &mut Frame, area: Rect, app: &App, is_korean: bool) {
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
        .title(format!(" ❤️ 서바이벌 ({}) ", if is_korean { "한글" } else { "영어" }))
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Red));

    let typed = app.input_automata.get_text();
    let current_word = &app.target_text;
    let text_spans = super::game_typed_spans(&typed, current_word);

    // 라이프 표시
    let lives_str: String = "♥".repeat(app.game_mode_lives as usize) + &"♡".repeat(5usize.saturating_sub(app.game_mode_lives as usize));

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        lives_str,
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("🎯 점수: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}", app.game_mode_score), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled("  |  📝 라운드: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}", app.game_mode_round + 1), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("  |  🔥 콤보: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}", app.game_mode_combo), Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(""));
    // 난이도 힌트
    let difficulty = 1 + app.game_mode_round / 10;
    lines.push(Line::from(Span::styled(
        format!("난이도: ★{}", "★".repeat(std::cmp::min(difficulty, 5))),
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(" 제시 단어: ", Style::default().fg(Color::Gray)),
        Span::styled(current_word, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(vec![
        Span::styled(" 나의 입력: ", Style::default().fg(Color::Gray)),
    ]));
    lines.push(Line::from(text_spans));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, layout[1]);
}

pub(super) fn draw_typing_rain(f: &mut Frame, area: Rect, app: &App, _is_korean: bool) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // 상태 바
            Constraint::Min(8),    // 레인 영역
            Constraint::Length(3), // 입력 바
        ])
        .split(area);

    // 1. 상태 바
    let lives_str: String = "♥".repeat(app.game_mode_lives as usize) + &"♡".repeat(5usize.saturating_sub(app.game_mode_lives as usize));
    let status_line = Line::from(vec![
        Span::styled(format!(" {} ", lives_str), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::styled("  |  🎯 점수: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}", app.game_mode_score), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled("  |  💀 파괴: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}개", app.game_words_correct), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    ]);
    let status_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));
    let status_p = Paragraph::new(status_line).block(status_block);
    f.render_widget(status_p, main_layout[0]);

    // 2. 레인 영역 - 간소화된 버전 (Paragraph 기반)
    let rain_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" 🌧️ 타자 레인 ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Blue));

    let rain_height = main_layout[1].height.saturating_sub(2) as usize;
    let rain_width = main_layout[1].width.saturating_sub(2) as usize;

    // 각 줄을 구성
    let mut rain_lines: Vec<Line> = Vec::new();
    for row in 0..rain_height {
        let mut row_spans = Vec::new();
        // 이 줄에 표시할 단어들 찾기
        let mut line_chars: Vec<(usize, char, Style)> = Vec::new();

        for (idx, word) in app.rain_words.iter().enumerate() {
            if word.destroyed {
                continue;
            }
            let word_row = word.row as usize;
            if word_row != row {
                continue;
            }
            let col = word.column as usize;
            for (ci, ch) in word.text.chars().enumerate() {
                let pos = col + ci;
                if pos < rain_width {
                    let style = if word.active {
                        if ci < word.typed_len {
                            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                        } else if ci == word.typed_len {
                            Style::default().fg(Color::Yellow).bg(Color::DarkGray).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                        }
                    } else if app.rain_active_idx == Some(idx) {
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                    } else {
                        // 바닥에 가까울수록 빨간색
                        if word.row > (rain_height as f32 * 0.7) {
                            Style::default().fg(Color::Red)
                        } else {
                            Style::default().fg(Color::White)
                        }
                    };
                    line_chars.push((pos, ch, style));
                }
            }
        }

        if line_chars.is_empty() {
            rain_lines.push(Line::from(""));
        } else {
            // 위치순 정렬
            line_chars.sort_by_key(|(pos, _, _)| *pos);
            let mut current_pos = 0;
            for (pos, ch, style) in line_chars {
                if pos > current_pos {
                    row_spans.push(Span::raw(" ".repeat(pos - current_pos)));
                }
                row_spans.push(Span::styled(ch.to_string(), style));
                current_pos = pos + 1;
            }
            rain_lines.push(Line::from(row_spans));
        }
    }

    let rain_p = Paragraph::new(rain_lines).block(rain_block);
    f.render_widget(rain_p, main_layout[1]);

    // 3. 입력 바
    let typed = app.input_automata.get_text();
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" 입력 ")
        .border_style(Style::default().fg(Color::Yellow));
    let input_p = Paragraph::new(Line::from(Span::styled(
        format!(" ▸ {}", typed),
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    )))
    .block(input_block);
    f.render_widget(input_p, main_layout[2]);
}

pub(super) fn draw_flash_typing(f: &mut Frame, area: Rect, app: &App, is_korean: bool) {
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
        .title(format!(" 👻 플래시 타이핑 ({}) ", if is_korean { "한글" } else { "영어" }))
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Magenta));

    let typed = app.input_automata.get_text();

    let mut lines = Vec::new();
    lines.push(Line::from(""));

    // 상태 정보
    lines.push(Line::from(vec![
        Span::styled("📝 라운드: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}/{}", app.game_mode_round + 1, app.word_list.len()), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("  |  🎯 점수: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}", app.game_mode_score), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled("  |  ⚡ 표시시간: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}ms", app.flash_duration_ms), Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::from(""));

    if app.flash_answer_shown {
        // 정답/오답 표시
        match app.flash_was_correct {
            Some(true) => {
                lines.push(Line::from(Span::styled(
                    "✅ 정답! 잘했습니다!",
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                )));
            }
            Some(false) => {
                lines.push(Line::from(Span::styled(
                    "❌ 오답!",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(vec![
                    Span::styled(" 정답: ", Style::default().fg(Color::Gray)),
                    Span::styled(&app.target_text, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled("  |  입력: ", Style::default().fg(Color::Gray)),
                    Span::styled(&typed, Style::default().fg(Color::Red)),
                ]));
            }
            None => {}
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "[Enter] 다음 라운드로",
            Style::default().fg(Color::DarkGray),
        )));
    } else if app.flash_visible {
        // 단어 표시 중
        lines.push(Line::from(Span::styled(
            "👀 이 단어를 기억하세요!",
            Style::default().fg(Color::Yellow),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            &app.target_text,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )));
    } else {
        // 단어 숨김 - 입력 모드
        lines.push(Line::from(Span::styled(
            "💭 기억나는 단어를 입력하세요!",
            Style::default().fg(Color::Yellow),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "????",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(" 나의 입력: ", Style::default().fg(Color::Gray)),
            Span::styled(&typed, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]));
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, layout[1]);
}

pub(super) fn draw_daily_challenge(f: &mut Frame, area: Rect, app: &App, is_korean: bool) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(area);

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(format!(" 🏆 데일리 챌린지 ({}) - {} ", if is_korean { "한글" } else { "영어" }, today))
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Yellow));

    let typed = app.input_automata.get_text();
    let current_word = &app.target_text;
    let text_spans = super::game_typed_spans(&typed, current_word);

    // 진행 바 문자열
    let progress = format!(
        "{}{}",
        "█".repeat(app.current_word_idx),
        "░".repeat(app.word_list.len().saturating_sub(app.current_word_idx))
    );

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("📊 진행: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}/{}", app.current_word_idx + 1, app.word_list.len()), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("  ", Style::default()),
        Span::styled(progress, Style::default().fg(Color::Green)),
    ]));
    if let Some(best) = app.daily_best_score {
        lines.push(Line::from(Span::styled(
            format!("🏅 오늘의 최고 점수: {}", best),
            Style::default().fg(Color::Yellow),
        )));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(" 제시 단어: ", Style::default().fg(Color::Gray)),
        Span::styled(current_word, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(vec![
        Span::styled(" 나의 입력: ", Style::default().fg(Color::Gray)),
    ]));
    lines.push(Line::from(text_spans));
    lines.push(Line::from(""));
    lines.push(super::make_stats_bar(app));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, layout[1]);
}

pub(super) fn draw_game_over(f: &mut Frame, area: Rect, app: &App, game_type: GameType, _is_korean: bool) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(area);

    let type_name = match game_type {
        GameType::TimeAttack => "⏱️ 시간 제한 모드",
        GameType::Survival => "❤️ 서바이벌 모드",
        GameType::TypingRain => "🌧️ 타자 레인",
        GameType::FlashTyping => "👻 플래시 타이핑",
        GameType::DailyChallenge => "🏆 데일리 챌린지",
        GameType::LongTextRace => "🏎️ 긴 글 레이스",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(format!(" {} - 결과 ", type_name))
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Yellow));

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "🏁 게임 종료!",
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(""));

    // 점수
    lines.push(Line::from(vec![
        Span::styled("🎯 최종 점수: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}", app.game_mode_score), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(""));

    // 완료 진행 (모드는 단어 / 긴 글은 문단)
    let progress_label = if game_type == GameType::LongTextRace {
        "✅ 완료 문단: "
    } else {
        "✅ 완료 단어: "
    };
    lines.push(Line::from(vec![
        Span::styled(progress_label, Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{} / {}", app.game_words_correct, app.game_words_total),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ),
    ]));

    // 최대 콤보
    lines.push(Line::from(vec![
        Span::styled("🔥 최대 콤보: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}", app.game_mode_max_combo), Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
    ]));

    // CPM
    lines.push(Line::from(vec![
        Span::styled("⌨️ 분당타수: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{} CPM", app.get_cpm()), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]));

    // 정확도
    lines.push(Line::from(vec![
        Span::styled("📊 정확도: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:.1}%", app.get_accuracy()), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    ]));

    // 경과 시간
    lines.push(Line::from(vec![
        Span::styled("⏱️ 경과 시간: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}초", app.elapsed_time.as_secs()), Style::default().fg(Color::White)),
    ]));

    // 플래시 타이핑 전용 통계
    if game_type == GameType::FlashTyping && !app.flash_response_times.is_empty() {
        let avg_time: f64 = app.flash_response_times.iter().sum::<f64>() / app.flash_response_times.len() as f64;
        lines.push(Line::from(vec![
            Span::styled("⚡ 평균 응답 시간: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.2}초", avg_time), Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        ]));
    }

    // 데일리 챌린지 전용
    if game_type == GameType::DailyChallenge {
        if let Some(best) = app.daily_best_score {
            lines.push(Line::from(vec![
                Span::styled("🏅 오늘의 최고 점수: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{}", best), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Enter] 다시 하기  |  [Esc] 메뉴로 돌아가기",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, layout[1]);
}
pub(super) fn draw_long_text_race(f: &mut Frame, area: Rect, app: &App, _is_korean: bool, _text_idx: usize) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 제목 바
            Constraint::Min(10),   // 스크롤 본문 뷰포트
            Constraint::Length(5), // 하단 스탯 & 미니 그래프
        ])
        .split(area);

    // 1. 상단 제목 바
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Magenta));
    let progress_pct = (app.long_text_current_para_idx as f64 / app.long_text_paragraphs.len() as f64) * 100.0;
    let title_line = Line::from(vec![
        Span::styled(format!(" 🏎️  {}  ", app.long_text_title), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(format!(" |  진행률: {:.1}% ({}/{}) ", progress_pct, app.long_text_current_para_idx, app.long_text_paragraphs.len()), Style::default().fg(Color::Cyan)),
    ]);
    let title_p = Paragraph::new(title_line).block(title_block);
    f.render_widget(title_p, main_layout[0]);

    // 2. 스크롤 본문 뷰포트 (현재 인덱스 기준 상하 2문단씩 노출)
    let body_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(" 📄 본문 타이핑 ")
        .border_style(Style::default().fg(Color::Yellow));

    let current_idx = app.long_text_current_para_idx;
    let total_paras = app.long_text_paragraphs.len();

    let mut body_lines = Vec::new();
    body_lines.push(Line::from(""));

    // 현재 문단 앞 2줄 노출
    let start_idx = current_idx.saturating_sub(2);
    for idx in start_idx..current_idx {
        body_lines.push(Line::from(Span::styled(
            format!("   {}", app.long_text_paragraphs[idx]),
            Style::default().fg(Color::DarkGray),
        )));
        body_lines.push(Line::from(""));
    }

    // 현재 타이핑 중인 문단 (자모 접두사 기반 정오 표시)
    let current_word = &app.target_text;
    let typed = app.input_automata.get_text();
    let mut text_spans = vec![Span::styled(
        " ➔ ",
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    )];
    text_spans.extend(super::game_typed_spans(&typed, current_word));
    body_lines.push(Line::from(text_spans));
    body_lines.push(Line::from(""));

    // 다음 문단 2줄 노출
    let end_idx = std::cmp::min(current_idx + 3, total_paras);
    for idx in (current_idx + 1)..end_idx {
        body_lines.push(Line::from(Span::styled(
            format!("   {}", app.long_text_paragraphs[idx]),
            Style::default().fg(Color::Gray),
        )));
        body_lines.push(Line::from(""));
    }

    let body_p = Paragraph::new(body_lines).block(body_block).wrap(Wrap { trim: false });
    f.render_widget(body_p, main_layout[1]);

    // 3. 하단 실시간 통계 및 미니 타속 그래프
    let stats_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" 📈 실시간 분석 ")
        .border_style(Style::default().fg(Color::DarkGray));

    // 미니 타속 변화 차트 생성
    let mut graph_spans = vec![Span::styled("타속 변화: ", Style::default().fg(Color::Gray))];
    if app.long_text_cpm_history.is_empty() {
        graph_spans.push(Span::styled("데이터 측정 중...", Style::default().fg(Color::DarkGray)));
    } else {
        let max_val = *app.long_text_cpm_history.iter().max().unwrap_or(&1) as f64;
        for &val in app.long_text_cpm_history.iter() {
            let ratio = val as f64 / max_val;
            let block_char = if ratio < 0.15 {
                " "
            } else if ratio < 0.3 {
                "▃"
            } else if ratio < 0.45 {
                "▄"
            } else if ratio < 0.6 {
                "▅"
            } else if ratio < 0.75 {
                "▆"
            } else if ratio < 0.9 {
                "▇"
            } else {
                "█"
            };
            let color = if val > 400 {
                Color::Magenta
            } else if val > 250 {
                Color::Green
            } else {
                Color::Yellow
            };
            graph_spans.push(Span::styled(block_char, Style::default().fg(color)));
        }
        graph_spans.push(Span::styled(format!(" {} CPM", app.get_cpm()), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    }

    let stats_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(stats_block.inner(main_layout[2]));

    let stat_info = Line::from(vec![
        Span::styled("⌨️ 현재 속도: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{} CPM", app.get_cpm()), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("  |  📊 정확도: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:.1}%", app.get_accuracy()), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::styled("  |  ⚠️ 오타 수: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}회", app.total_errors), Style::default().fg(Color::Red)),
    ]);

    f.render_widget(stats_block, main_layout[2]);
    f.render_widget(Paragraph::new(stat_info), stats_layout[0]);
    f.render_widget(Paragraph::new(Line::from(graph_spans)), stats_layout[1]);
}

