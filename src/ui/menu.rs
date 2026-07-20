use crate::app::{App, GameType};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
pub(super) fn draw_main_menu(f: &mut Frame, area: Rect, app: &App) {
    let menu_items = vec![
        "1. 자리 연습 (Finger Placement Practice)",
        "2. 낱말 연습 (Word Practice)",
        "3. 문장 연습 (Sentence Practice)",
        "4. 통계 및 기록 분석 (Statistics)",
        "5. 🎮 게임 모드 (Game Mode)",
        "6. 종료 (Exit)",
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
pub(super) fn draw_finger_language_menu(f: &mut Frame, area: Rect, app: &App) {
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

pub(super) fn draw_finger_level_menu(f: &mut Frame, area: Rect, app: &App, is_korean: bool) {
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

pub(super) fn draw_word_menu(f: &mut Frame, area: Rect, app: &App) {
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

pub(super) fn draw_sentence_menu(f: &mut Frame, area: Rect, app: &App) {
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
pub(super) fn draw_game_mode_menu(f: &mut Frame, area: Rect, app: &App) {
    let menu_items = vec![
        "1. ⏱️  시간 제한 모드 (Time Attack)",
        "2. ❤️  서바이벌 모드 (Survival)",
        "3. 🌧️  타자 레인 (Typing Rain)",
        "4. 👻 플래시 타이핑 (Flash Typing)",
        "5. 🏆 데일리 챌린지 (Daily Challenge)",
        "6. 🏎️  긴 글 레이스 (Long Text Race)",
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
        .title(" 🎮 GAME MODE ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Magenta));

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "게임 모드를 선택하세요!",
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
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
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(paragraph, layout[1]);
}

pub(super) fn draw_game_language_menu(f: &mut Frame, area: Rect, app: &App, game_type: GameType) {
    let type_name = match game_type {
        GameType::TimeAttack => "⏱️ 시간 제한 모드",
        GameType::Survival => "❤️ 서바이벌 모드",
        GameType::TypingRain => "🌧️ 타자 레인",
        GameType::FlashTyping => "👻 플래시 타이핑",
        GameType::DailyChallenge => "🏆 데일리 챌린지",
        GameType::LongTextRace => "🏎️ 긴 글 레이스",
    };

    let options = vec![
        "1. 한글 (Korean)",
        "2. 영어 (English)",
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
        .title(format!(" {} ", type_name))
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Magenta));

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from("언어를 선택하세요."));
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
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(layout[1]);

    f.render_widget(paragraph, vertical_layout[1]);
}

pub(super) fn draw_time_select_menu(f: &mut Frame, area: Rect, app: &App, _is_korean: bool) {
    let options = vec![
        "1. 30초 (30 seconds)",
        "2. 60초 (60 seconds)",
        "3. 120초 (120 seconds)",
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
        .title(" ⏱️ 시간 설정 ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Magenta));

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from("제한 시간을 선택하세요."));
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
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(layout[1]);

    f.render_widget(paragraph, vertical_layout[1]);
}

pub(super) fn draw_long_text_menu(f: &mut Frame, area: Rect, app: &App, is_korean: bool) {
    let titles = App::get_long_text_titles(is_korean);

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
        .title(" 🏎️ 긴 글 레이스 목록 ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Magenta));

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from("연습할 긴 글을 선택하세요."));
    lines.push(Line::from(""));

    for (idx, item) in titles.iter().enumerate() {
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
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(layout[1]);

    f.render_widget(paragraph, vertical_layout[1]);
}

