//! 한글 두벌식 조합 오토마타 엔진

/// 초성 리스트 (19개)
const CHOSEONG: &[char] = &[
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'
];

/// 중성 리스트 (21개)
const JUNGSEONG: &[char] = &[
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ'
];

/// 종성 리스트 (28개, 0번인 ' ' 혹은 '\0'은 종성 없음)
const JONGSEONG: &[char] = &[
    '\0', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㅈ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ', 'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'
];

/// 영문 쿼티 키를 한글 자모로 매핑
pub fn map_qwerty_to_jamo(c: char) -> Option<char> {
    let jamo = match c {
        'q' => 'ㅂ', 'w' => 'ㅈ', 'e' => 'ㄷ', 'r' => 'ㄱ', 't' => 'ㅅ',
        'y' => 'ㅛ', 'u' => 'ㅕ', 'i' => 'ㅑ', 'o' => 'ㅐ', 'p' => 'ㅔ',
        'a' => 'ㅁ', 's' => 'ㄴ', 'd' => 'ㅇ', 'f' => 'ㄹ', 'g' => 'ㅎ',
        'h' => 'ㅗ', 'j' => 'ㅓ', 'k' => 'ㅏ', 'l' => 'ㅣ',
        'z' => 'ㅋ', 'x' => 'ㅌ', 'c' => 'ㅊ', 'v' => 'ㅍ',
        'b' => 'ㅠ', 'n' => 'ㅜ', 'm' => 'ㅡ',
        'Q' => 'ㅃ', 'W' => 'ㅉ', 'E' => 'ㄸ', 'R' => 'ㄲ', 'T' => 'ㅆ',
        'O' => 'ㅒ', 'P' => 'ㅖ',
        _ => return None,
    };
    Some(jamo)
}

/// 한글 자모의 속성을 구분
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JamoType {
    Consonant, // 자음
    Vowel,     // 모음
}

pub fn get_jamo_type(c: char) -> Option<JamoType> {
    if CHOSEONG.contains(&c) || JONGSEONG.contains(&c) || c == 'ㄳ' || c == 'ㄵ' || c == 'ㄶ' || c == 'ㄺ' || c == 'ㄻ' || c == 'ㄼ' || c == 'ㄽ' || c == 'ㄾ' || c == 'ㄿ' || c == 'ㅀ' || c == 'ㅄ' {
        // 자음 확인 (초성이나 종성에 들어가는 자음들)
        // 주의: 'ㅃ', 'ㄸ', 'ㅉ'는 초성에만 들어가고 종성엔 안들어감.
        Some(JamoType::Consonant)
    } else if JUNGSEONG.contains(&c) {
        Some(JamoType::Vowel)
    } else {
        None
    }
}

/// 이중 모음 결합 처리
fn merge_vowels(v1: char, v2: char) -> Option<char> {
    match (v1, v2) {
        ('ㅗ', 'ㅏ') => Some('ㅘ'),
        ('ㅗ', 'ㅐ') => Some('ㅙ'),
        ('ㅗ', 'ㅣ') => Some('ㅚ'),
        ('ㅜ', 'ㅓ') => Some('ㅝ'),
        ('ㅜ', 'ㅔ') => Some('ㅞ'),
        ('ㅜ', 'ㅣ') => Some('ㅟ'),
        ('ㅡ', 'ㅣ') => Some('ㅢ'),
        _ => None,
    }
}

/// 이중 모음 분해 처리 (백스페이스용)
fn split_vowel(v: char) -> (char, Option<char>) {
    match v {
        'ㅘ' => ('ㅗ', Some('ㅏ')),
        'ㅙ' => ('ㅗ', Some('ㅐ')),
        'ㅚ' => ('ㅗ', Some('ㅣ')),
        'ㅝ' => ('ㅜ', Some('ㅓ')),
        'ㅞ' => ('ㅜ', Some('ㅔ')),
        'ㅟ' => ('ㅜ', Some('ㅣ')),
        'ㅢ' => ('ㅡ', Some('ㅣ')),
        other => (other, None),
    }
}

/// 이중 종성 결합 처리
fn merge_jongseong(j1: char, j2: char) -> Option<char> {
    match (j1, j2) {
        ('ㄱ', 'ㅅ') => Some('ㄳ'),
        ('ㄴ', 'ㅈ') => Some('ㄵ'),
        ('ㄴ', 'ㅎ') => Some('ㄶ'),
        ('ㄹ', 'ㄱ') => Some('ㄺ'),
        ('ㄹ', 'ㅁ') => Some('ㄻ'),
        ('ㄹ', 'ㅂ') => Some('ㄼ'),
        ('ㄹ', 'ㅅ') => Some('ㄽ'),
        ('ㄹ', 'ㅌ') => Some('ㄾ'),
        ('ㄹ', 'ㅍ') => Some('ㄿ'),
        ('ㄹ', 'ㅎ') => Some('ㅀ'),
        ('ㅂ', 'ㅅ') => Some('ㅄ'),
        _ => None,
    }
}

/// 이중 종성 분해 처리 (백스페이스 및 자음 이동용)
fn split_jongseong(j: char) -> (char, Option<char>) {
    match j {
        'ㄳ' => ('ㄱ', Some('ㅅ')),
        'ㄵ' => ('ㄴ', Some('ㅈ')),
        'ㄶ' => ('ㄴ', Some('ㅎ')),
        'ㄺ' => ('ㄹ', Some('ㄱ')),
        'ㄻ' => ('ㄹ', Some('ㅁ')),
        'ㄼ' => ('ㄹ', Some('ㅂ')),
        'ㄽ' => ('ㄹ', Some('ㅅ')),
        'ㄾ' => ('ㄹ', Some('ㅌ')),
        'ㄿ' => ('ㄹ', Some('ㅍ')),
        'ㅀ' => ('ㄹ', Some('ㅎ')),
        'ㅄ' => ('ㅂ', Some('ㅅ')),
        other => (other, None),
    }
}

/// 조합 중인 상태
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HangulState {
    Empty,
    /// 단독 자음/모음 (초성이 될 수 없거나 단독 입력된 자모)
    Single(char),
    /// 초성 상태
    Cho { cho: char },
    /// 초성 + 중성 상태
    ChoJung { cho: char, jung: char },
    /// 초성 + 중성 + 종성 상태 (종성은 1개 또는 이중 종성)
    ChoJungJong { cho: char, jung: char, jong: char },
}

impl HangulState {
    /// 상태를 유니코드 완성형 문자로 변환
    pub fn to_char(&self) -> Option<char> {
        match self {
            HangulState::Empty => None,
            HangulState::Single(c) => Some(*c),
            HangulState::Cho { cho } => Some(*cho),
            HangulState::ChoJung { cho, jung } => {
                let cho_idx = CHOSEONG.iter().position(|&x| x == *cho)?;
                let jung_idx = JUNGSEONG.iter().position(|&x| x == *jung)?;
                let code = 0xAC00 + (cho_idx * 21 * 28) + (jung_idx * 28);
                std::char::from_u32(code as u32)
            }
            HangulState::ChoJungJong { cho, jung, jong } => {
                let cho_idx = CHOSEONG.iter().position(|&x| x == *cho)?;
                let jung_idx = JUNGSEONG.iter().position(|&x| x == *jung)?;
                let jong_idx = JONGSEONG.iter().position(|&x| x == *jong)?;
                let code = 0xAC00 + (cho_idx * 21 * 28) + (jung_idx * 28) + jong_idx;
                std::char::from_u32(code as u32)
            }
        }
    }
}

pub struct HangulAutomata {
    /// 완성되어 고정된 텍스트
    completed: String,
    /// 현재 조합 중인 상태
    current: HangulState,
    /// 지금까지의 누적 타수 (Stroke)
    strokes: usize,
    /// 자모 결합 없이 키 매핑된 한글 자모를 그대로 입력받는 모드 (단독 자모 연습용)
    pub raw_jamo_mode: bool,
    /// 영문 입력 상태 그대로 바이패스하는 모드 (영문 연습용)
    pub english_mode: bool,
}

impl HangulAutomata {
    pub fn new() -> Self {
        Self {
            completed: String::new(),
            current: HangulState::Empty,
            strokes: 0,
            raw_jamo_mode: false,
            english_mode: false,
        }
    }

    /// 현재까지 입력된 전체 문자열 반환 (완성된 텍스트 + 현재 조합 문자)
    pub fn get_text(&self) -> String {
        let mut result = self.completed.clone();
        if let Some(c) = self.current.to_char() {
            result.push(c);
        }
        result
    }

    /// 현재 조합 중인 임시 글자 반환
    pub fn get_current_char(&self) -> Option<char> {
        self.current.to_char()
    }

    /// 현재 누적 타수 반환
    pub fn get_strokes(&self) -> usize {
        self.strokes
    }

    /// 상태 초기화 (연습 리셋 등)
    pub fn clear(&mut self) {
        self.completed.clear();
        self.current = HangulState::Empty;
        self.strokes = 0;
        self.english_mode = false;
    }

    /// 텍스트 전체를 직접 설정 (완성형 상태로 설정할 때 사용)
    #[allow(dead_code)]
    pub fn set_text(&mut self, text: &str) {
        self.completed = text.to_string();
        self.current = HangulState::Empty;
        self.strokes = text.chars().map(|c| {
            if c == ' ' { 1 }
            else if c.is_ascii() { 1 }
            else { 3 } // 대략 한글 한 글자 3타
        }).sum();
    }

    /// 문자 하나 입력 처리 (영문 키보드 자모 매핑값 혹은 스페이스 등)
    pub fn push_char(&mut self, input_char: char, expected_char: Option<char>) {
        if self.english_mode {
            self.completed.push(input_char);
            self.strokes += 1;
            return;
        }

        let is_expected_jamo = match expected_char {
            Some(ec) => {
                let code = ec as u32;
                (0x3130..=0x318F).contains(&code)
            }
            None => false,
        };

        if self.raw_jamo_mode || is_expected_jamo {
            let ch = map_qwerty_to_jamo(input_char).unwrap_or(input_char);
            self.completed.push(ch);
            self.strokes += 1;
            return;
        }

        // 스페이스나 개행, 특수문자 등 한글 자모가 아니면 완성 후 고정
        let Some(jamo) = map_qwerty_to_jamo(input_char) else {
            // 한글 조합이 불가능한 일반 문자(공백, 숫자, 영문 기호 등)
            self.commit_current();
            self.completed.push(input_char);
            self.strokes += 1;
            return;
        };

        self.strokes += 1;
        let jamo_type = get_jamo_type(jamo).unwrap_or(JamoType::Consonant);

        match self.current.clone() {
            HangulState::Empty => {
                if jamo_type == JamoType::Consonant {
                    if CHOSEONG.contains(&jamo) {
                        self.current = HangulState::Cho { cho: jamo };
                    } else {
                        self.current = HangulState::Single(jamo);
                    }
                } else {
                    self.current = HangulState::Single(jamo);
                }
            }
            HangulState::Single(_) => {
                self.commit_current();
                if jamo_type == JamoType::Consonant && CHOSEONG.contains(&jamo) {
                    self.current = HangulState::Cho { cho: jamo };
                } else {
                    self.current = HangulState::Single(jamo);
                }
            }
            HangulState::Cho { cho } => {
                if jamo_type == JamoType::Vowel {
                    self.current = HangulState::ChoJung { cho, jung: jamo };
                } else {
                    self.commit_current();
                    if CHOSEONG.contains(&jamo) {
                        self.current = HangulState::Cho { cho: jamo };
                    } else {
                        self.current = HangulState::Single(jamo);
                    }
                }
            }
            HangulState::ChoJung { cho, jung } => {
                if jamo_type == JamoType::Vowel {
                    if let Some(merged) = merge_vowels(jung, jamo) {
                        self.current = HangulState::ChoJung { cho, jung: merged };
                    } else {
                        self.commit_current();
                        self.current = HangulState::Single(jamo);
                    }
                } else {
                    if JONGSEONG.contains(&jamo) {
                        self.current = HangulState::ChoJungJong { cho, jung, jong: jamo };
                    } else {
                        self.commit_current();
                        self.current = HangulState::Cho { cho: jamo };
                    }
                }
            }
            HangulState::ChoJungJong { cho, jung, jong } => {
                if jamo_type == JamoType::Consonant {
                    if let Some(merged) = merge_jongseong(jong, jamo) {
                        self.current = HangulState::ChoJungJong { cho, jung, jong: merged };
                    } else {
                        self.commit_current();
                        if CHOSEONG.contains(&jamo) {
                            self.current = HangulState::Cho { cho: jamo };
                        } else {
                            self.current = HangulState::Single(jamo);
                        }
                    }
                } else {
                    let (j1, j2_opt) = split_jongseong(jong);
                    if let Some(j2) = j2_opt {
                        if CHOSEONG.contains(&j2) {
                            let prev_char = HangulState::ChoJungJong { cho, jung, jong: j1 }.to_char().unwrap();
                            self.completed.push(prev_char);
                            self.current = HangulState::ChoJung { cho: j2, jung: jamo };
                        } else {
                            self.commit_current();
                            self.current = HangulState::Single(jamo);
                        }
                    } else {
                        if CHOSEONG.contains(&jong) {
                            let prev_char = HangulState::ChoJung { cho, jung }.to_char().unwrap();
                            self.completed.push(prev_char);
                            self.current = HangulState::ChoJung { cho: jong, jung: jamo };
                        } else {
                            self.commit_current();
                            self.current = HangulState::Single(jamo);
                        }
                    }
                }
            }
        }
    }

    /// 백스페이스 입력 처리 (자모 단위 역분해)
    pub fn backspace(&mut self) -> bool {
        if self.english_mode || self.raw_jamo_mode {
            if self.completed.pop().is_some() {
                self.strokes = self.strokes.saturating_sub(1);
                return true;
            }
            return false;
        }

        if self.current == HangulState::Empty {
            if let Some(c) = self.completed.pop() {
                self.strokes = self.strokes.saturating_sub(1);
                self.restore_last_char_to_state(c);
                return true;
            }
            return false;
        }

        self.strokes = self.strokes.saturating_sub(1);
        match self.current.clone() {
            HangulState::Empty => unreachable!(),
            HangulState::Single(_) => {
                self.current = HangulState::Empty;
            }
            HangulState::Cho { .. } => {
                self.current = HangulState::Empty;
            }
            HangulState::ChoJung { cho, jung } => {
                let (v1, v2_opt) = split_vowel(jung);
                if v2_opt.is_some() {
                    self.current = HangulState::ChoJung { cho, jung: v1 };
                } else {
                    self.current = HangulState::Cho { cho };
                }
            }
            HangulState::ChoJungJong { cho, jung, jong } => {
                let (j1, j2_opt) = split_jongseong(jong);
                if j2_opt.is_some() {
                    self.current = HangulState::ChoJungJong { cho, jung, jong: j1 };
                } else {
                    self.current = HangulState::ChoJung { cho, jung };
                }
            }
        }
        true
    }

    /// 현재 조합 중인 상태를 강제로 완료 버퍼에 커밋
    pub fn commit_current(&mut self) {
        if let Some(c) = self.current.to_char() {
            self.completed.push(c);
        }
        self.current = HangulState::Empty;
    }

    /// 완성된 버퍼에서 글자 하나를 꺼내 조합 상태로 되돌림 (백스페이스 백트래킹용)
    fn restore_last_char_to_state(&mut self, c: char) {
        let code = c as u32;
        if !(0xAC00..=0xD7A3).contains(&code) {
            if CHOSEONG.contains(&c) {
                self.current = HangulState::Cho { cho: c };
            } else {
                self.current = HangulState::Single(c);
            }
            return;
        }

        let offset = code - 0xAC00;
        let jong_idx = (offset % 28) as usize;
        let temp = offset / 28;
        let jung_idx = (temp % 21) as usize;
        let cho_idx = (temp / 21) as usize;

        let cho = CHOSEONG[cho_idx];
        let jung = JUNGSEONG[jung_idx];
        let jong = JONGSEONG[jong_idx];

        if jong == '\0' {
            self.current = HangulState::ChoJung { cho, jung };
        } else {
            self.current = HangulState::ChoJungJong { cho, jung, jong };
        }
    }
}

/// 단독 자모(이중 자모 포함) 해체
fn decompose_single_jamo(c: char) -> Vec<char> {
    let (v1, v2_opt) = split_vowel(c);
    if v2_opt.is_some() {
        return vec![v1, v2_opt.unwrap()];
    }
    
    let (j1, j2_opt) = split_jongseong(c);
    if j2_opt.is_some() {
        return vec![j1, j2_opt.unwrap()];
    }
    
    vec![c]
}

/// 한글 글자를 자모 단위로 완전히 쪼갬 (이중 자모 해체 포함)
pub fn fully_decompose_hangul(c: char) -> Vec<char> {
    let code = c as u32;
    if !(0xAC00..=0xD7A3).contains(&code) {
        return decompose_single_jamo(c);
    }
    
    let offset = code - 0xAC00;
    let jong_idx = (offset % 28) as usize;
    let temp = offset / 28;
    let jung_idx = (temp % 21) as usize;
    let cho_idx = (temp / 21) as usize;
    
    let cho = CHOSEONG[cho_idx];
    let jung = JUNGSEONG[jung_idx];
    let jong = JONGSEONG[jong_idx];
    
    let mut result = vec![cho];
    
    let (v1, v2_opt) = split_vowel(jung);
    result.push(v1);
    if let Some(v2) = v2_opt {
        result.push(v2);
    }
    
    if jong != '\0' {
        let (j1, j2_opt) = split_jongseong(jong);
        result.push(j1);
        if let Some(j2) = j2_opt {
            result.push(j2);
        }
    }
    
    result
}

/// 실시간 타자 판정 완화 알고리즘
/// typed_char가 expected_char로 가기 위해 조합 중인 올바른 중간 단계인지 검사
pub fn is_typing_valid(typed_char: char, expected_char: char) -> bool {
    if typed_char == expected_char {
        return true;
    }
    
    let typed_jamos = fully_decompose_hangul(typed_char);
    let expected_jamos = fully_decompose_hangul(expected_char);
    
    if typed_jamos.len() > expected_jamos.len() {
        return false;
    }
    
    // 입력 중인 자모 배열이 타깃 글자 자모 배열의 접두사(prefix)인지 확인
    for (t, e) in typed_jamos.iter().zip(expected_jamos.iter()) {
        if t != e {
            return false;
        }
    }
    true
}

/// 입력 텍스트와 목표 텍스트를 대조하여 실제로 완료(또는 완료형 오타)된 글자 수를 카운트.
/// 조합 중인 마지막 미완성 자모(예: 'ㄲ'을 치고 아직 'ㅣ'를 안 친 상태)는 글자 수에서 제외하여 조기 종료 방지.
pub fn count_completed_chars(typed: &str, expected: &str) -> usize {
    let typed_chars: Vec<char> = typed.chars().collect();
    let expected_chars: Vec<char> = expected.chars().collect();
    
    let mut completed_count = 0;
    for i in 0..expected_chars.len() {
        if i >= typed_chars.len() {
            break;
        }
        let tc = typed_chars[i];
        let ec = expected_chars[i];
        
        if tc == ec {
            completed_count += 1;
        } else {
            let is_last_typed = i == typed_chars.len() - 1;
            if !is_last_typed {
                completed_count += 1;
            } else {
                let is_composing_hangul = {
                    let tc_code = tc as u32;
                    let ec_code = ec as u32;
                    (0x3130..=0x318F).contains(&tc_code) && (0xAC00..=0xD7A3).contains(&ec_code)
                };
                
                if !is_composing_hangul {
                    completed_count += 1;
                }
            }
        }
    }
    completed_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hangul_composition() {
        let mut automata = HangulAutomata::new();
        
        // '한' 입력 테스트: gks -> ㅎ, ㅏ, ㄴ
        automata.push_char('g', None); // ㅎ
        assert_eq!(automata.get_text(), "ㅎ");
        automata.push_char('k', None); // ㅏ
        assert_eq!(automata.get_text(), "하");
        automata.push_char('s', None); // ㄴ
        assert_eq!(automata.get_text(), "한");
        
        // '글' 입력 테스트: rm -> ㄱ, ㅡ, ㄹ
        automata.push_char('r', None); // ㄱ -> '한'이 확정되고 'ㄱ'이 조합되기 시작
        assert_eq!(automata.get_text(), "한ㄱ");
        automata.push_char('m', None); // ㅡ
        assert_eq!(automata.get_text(), "한그");
        automata.push_char('f', None); // ㄹ
        assert_eq!(automata.get_text(), "한글");
    }

    #[test]
    fn test_jongseong_splitting() {
        let mut automata = HangulAutomata::new();
        
        // '강' + '아' -> '가' + '아'
        automata.push_char('r', None); // ㄱ
        automata.push_char('k', None); // ㅏ
        automata.push_char('d', None); // ㅇ -> 강
        assert_eq!(automata.get_text(), "강");
        automata.push_char('k', None); // ㅏ -> '가' 확정 후 'ㅇ'과 'ㅏ'가 만나 '아'가 됨
        assert_eq!(automata.get_text(), "가아");
    }

    #[test]
    fn test_backspace() {
        let mut automata = HangulAutomata::new();
        
        // '한' 입력 후 백스페이스
        automata.push_char('g', None); // ㅎ
        automata.push_char('k', None); // 하
        automata.push_char('s', None); // 한
        
        assert_eq!(automata.get_text(), "한");
        
        automata.backspace(); // 한 -> 하
        assert_eq!(automata.get_text(), "하");
        
        automata.backspace(); // 하 -> ㅎ
        assert_eq!(automata.get_text(), "ㅎ");
        
        automata.backspace(); // ㅎ -> 빈 문자열
        assert_eq!(automata.get_text(), "");
    }

    #[test]
    fn test_raw_jamo_mode() {
        let mut automata = HangulAutomata::new();
        automata.raw_jamo_mode = true;

        automata.push_char('r', None); // ㄱ
        assert_eq!(automata.get_text(), "ㄱ");
        
        automata.push_char('k', None); // ㅏ -> 결합되지 않고 'ㄱㅏ'가 되어야 함
        assert_eq!(automata.get_text(), "ㄱㅏ");
        
        automata.push_char('s', None); // ㄴ -> 결합되지 않고 'ㄱㅏㄴ'이 되어야 함
        assert_eq!(automata.get_text(), "ㄱㅏㄴ");

        automata.backspace(); // 'ㄱㅏ'
        assert_eq!(automata.get_text(), "ㄱㅏ");

        automata.backspace(); // 'ㄱ'
        assert_eq!(automata.get_text(), "ㄱ");
    }

    #[test]
    fn test_count_completed_chars() {
        // 도끼
        assert_eq!(count_completed_chars("ㄷ", "도끼"), 0);
        assert_eq!(count_completed_chars("도", "도끼"), 1);
        assert_eq!(count_completed_chars("도ㄲ", "도끼"), 1); // ㄲ 조합 중 제외
        assert_eq!(count_completed_chars("도끼", "도끼"), 2); // 완료
        assert_eq!(count_completed_chars("도까", "도끼"), 2); // 오타이지만 글자는 다 채움

        // 단독 자모
        assert_eq!(count_completed_chars("ㅃ", "ㅃㅉㄸ"), 1);
        assert_eq!(count_completed_chars("ㅃㅈ", "ㅃㅉㄸ"), 2); // 오타
    }

    #[test]
    fn test_is_typing_valid() {
        // '강'을 기대할 때
        assert!(is_typing_valid('ㄱ', '강')); // 접두사 일치
        assert!(is_typing_valid('가', '강')); // 접두사 일치
        assert!(is_typing_valid('강', '강')); // 완전 일치
        assert!(!is_typing_valid('개', '강')); // 접두사 불일치
        assert!(!is_typing_valid('감', '강')); // 접두사 불일치
        
        // '과'를 기대할 때
        assert!(is_typing_valid('ㄱ', '과'));
        assert!(is_typing_valid('고', '과')); // 'ㅗ'는 'ㅘ'의 접두사
        assert!(is_typing_valid('과', '과'));
        assert!(!is_typing_valid('구', '과'));

        // 일반 영문
        assert!(is_typing_valid('a', 'a'));
        assert!(!is_typing_valid('a', 'b'));
    }
}
