use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Widget},
};

pub struct KeyboardWidget {
    /// 현재 입력해야 하는 대상 문자 (예: 'ㄱ', 'a', '1')
    pub target_char: Option<char>,
    /// 한글 자판인지 여부 (false 이면 QWERTY 자판)
    pub is_korean: bool,
}

impl KeyboardWidget {
    pub fn new(target_char: Option<char>, is_korean: bool) -> Self {
        Self { target_char, is_korean }
    }

    /// 한글 자모에 매핑되는 손가락 가이드 텍스트 반환
    pub fn get_finger_guide(&self) -> String {
        let Some(c) = self.target_char else {
            return "준비 완료. 입력을 시작하세요.".to_string();
        };

        let finger = match c {
            // 왼손 새끼손가락 (L5)
            'ㅂ' | 'ㅃ' | 'ㅁ' | 'ㅋ' | '1' | '!' | '`' | '~' | 'q' | 'Q' | 'a' | 'A' | 'z' | 'Z' => "왼손 새끼손가락 (L5)",
            
            // 왼손 약지손가락 (L4)
            'ㅈ' | 'ㅉ' | 'ㄴ' | 'ㅌ' | '2' | '@' | 'w' | 'W' | 's' | 'S' | 'x' | 'X' => "왼손 약지손가락 (L4)",
            
            // 왼손 중지손가락 (L3)
            'ㄷ' | 'ㄸ' | 'ㅇ' | 'ㅊ' | '3' | '#' | 'e' | 'E' | 'd' | 'D' | 'c' | 'C' => "왼손 중지손가락 (L3)",
            
            // 왼손 검지손가락 (L2)
            'ㄱ' | 'ㄲ' | 'ㅅ' | 'ㅆ' | 'ㄹ' | 'ㅎ' | 'ㅍ' | 'ㅠ' | '4' | '$' | '5' | '%' | 'r' | 'R' | 'f' | 'F' | 'v' | 'V' | 't' | 'T' | 'g' | 'G' | 'b' | 'B' => "왼손 검지손가락 (L2)",
            
            // 오른손 검지손가락 (R2)
            'ㅛ' | 'ㅕ' | 'ㅗ' | 'ㅓ' | 'ㅜ' | 'ㅡ' | '6' | '^' | '7' | '&' | 'y' | 'Y' | 'h' | 'H' | 'n' | 'N' | 'u' | 'U' | 'j' | 'J' | 'm' | 'M' => "오른손 검지손가락 (R2)",
            
            // 오른손 중지손가락 (R3)
            '양' | 'ㅑ' | 'ㅏ' | '8' | '*' | ',' | '<' | 'i' | 'I' | 'k' | 'K' => "오른손 중지손가락 (R3)",
            
            // 오른손 약지손가락 (R4)
            'ㅐ' | 'ㅒ' | 'ㅣ' | '9' | '(' | '.' | '>' | 'o' | 'O' | 'l' | 'L' => "오른손 약지손가락 (R4)",
            
            // 오른손 새끼손가락 (R5)
            'ㅔ' | 'ㅖ' | ';' | ':' | '0' | ')' | '-' | '_' | '=' | '+' | '[' | '{' | ']' | '}' | '\\' | '|' | '\'' | '"' | '/' | '?' | 'p' | 'P' => "오른손 새끼손가락 (R5)",
            
            // 공백
            ' ' => "양손 엄지손가락 (Space)",
            _ => "기타 특수 키 (자판 가이드를 확인하세요)",
        };
        format!("다음 입력 키 [ {} ] ➔ 담당: {}", c, finger)
    }
}

// 한글 두벌식 키보드 자판 레이아웃
const KOREAN_KEYBOARD_ROWS: [&[char]; 3] = [
    &['ㅂ', 'ㅈ', 'ㄷ', 'ㄱ', 'ㅅ', 'ㅛ', 'ㅕ', 'ㅑ', 'ㅐ', 'ㅔ'],
    &['ㅁ', 'ㄴ', 'ㅇ', 'ㄹ', 'ㅎ', 'ㅗ', 'ㅓ', 'ㅏ', 'ㅣ', ';'],
    &['ㅋ', 'ㅌ', 'ㅊ', 'ㅍ', 'ㅠ', 'ㅜ', 'ㅡ', ',', '.', '/'],
];

// QWERTY 영문 키보드 자판 레이아웃
const ENGLISH_KEYBOARD_ROWS: [&[char]; 3] = [
    &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
    &['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
    &['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
];

impl Widget for KeyboardWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // 자판 전체를 감싸는 블록
        let title_str = if self.is_korean {
            " 키보드 자리 가이드 (두벌식 자판) "
        } else {
            " 키보드 자리 가이드 (QWERTY 자판) "
        };
        
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(title_str)
            .style(Style::default().fg(Color::Cyan));
        
        let inner_area = block.inner(area);
        block.render(area, buf);

        if inner_area.width < 50 || inner_area.height < 9 {
            let text = "화면이 너무 작아 키보드 가이드를 그릴 수 없습니다.";
            buf.set_string(
                inner_area.x + (inner_area.width.saturating_sub(text.len() as u16) / 2),
                inner_area.y + (inner_area.height / 2),
                text,
                Style::default().fg(Color::Red),
            );
            return;
        }

        let start_x = inner_area.x + (inner_area.width.saturating_sub(44) / 2);
        let start_y = inner_area.y + (inner_area.height.saturating_sub(7) / 2);

        let guide_text = self.get_finger_guide();
        buf.set_string(
            inner_area.x + (inner_area.width.saturating_sub(guide_text.chars().count() as u16) / 2),
            start_y,
            &guide_text,
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        );

        let rows = if self.is_korean {
            &KOREAN_KEYBOARD_ROWS
        } else {
            &ENGLISH_KEYBOARD_ROWS
        };

        // 키보드 자판 드로잉 (3행)
        for (row_idx, row) in rows.iter().enumerate() {
            let y = start_y + 2 + (row_idx as u16 * 2);
            
            let row_offset = match row_idx {
                0 => 0,
                1 => 2,
                2 => 4,
                _ => 0,
            };

            for (col_idx, &key_char) in row.iter().enumerate() {
                let x = start_x + row_offset + (col_idx as u16 * 4);
                
                // 해당 키가 현재 입력해야 하는 target인지 확인
                let is_target = match self.target_char {
                    Some(tc) => {
                        let tc_lower = tc.to_ascii_lowercase();
                        tc == key_char || 
                        tc_lower == key_char ||
                        (tc == '!' && key_char == '1') ||
                        (tc == '@' && key_char == '2') ||
                        (tc == '#' && key_char == '3') ||
                        (tc == '$' && key_char == '4') ||
                        (tc == '%' && key_char == '5') ||
                        (tc == '^' && key_char == '6') ||
                        (tc == '&' && key_char == '7') ||
                        (tc == '*' && key_char == '8') ||
                        (tc == '(' && key_char == '9') ||
                        (tc == ')' && key_char == '0') ||
                        (tc == '_' && key_char == '-') ||
                        (tc == '+' && key_char == '=') ||
                        (tc == '{' && key_char == '[') ||
                        (tc == '}' && key_char == ']') ||
                        (tc == '|' && key_char == '\\') ||
                        (tc == ':' && key_char == ';') ||
                        (tc == '<' && key_char == ',') ||
                        (tc == '>' && key_char == '.') ||
                        (tc == '?' && key_char == '/') ||
                        (tc == 'ㅃ' && key_char == 'ㅂ') ||
                        (tc == 'ㅉ' && key_char == 'ㅈ') ||
                        (tc == 'ㄸ' && key_char == 'ㄷ') ||
                        (tc == 'ㄲ' && key_char == 'ㄱ') ||
                        (tc == 'ㅆ' && key_char == 'ㅅ') ||
                        (tc == 'ㅒ' && key_char == 'ㅐ') ||
                        (tc == 'ㅖ' && key_char == 'ㅔ')
                    }
                    None => false,
                };

                let key_style = if is_target {
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                let char_style = if is_target {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                buf.set_string(x, y, "[", key_style);
                buf.set_string(x + 1, y, &key_char.to_string(), char_style);
                buf.set_string(x + 2, y, "]", key_style);
            }
        }
    }
}
