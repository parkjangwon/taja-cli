use crate::hangeul::{HangulAutomata, map_qwerty_to_jamo};
use crate::storage::{StorageManager, PracticeRecord};
use std::collections::HashMap;
use std::time::{Duration, Instant};

mod timer;
mod practice;
mod game;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
    TimeAttack,
    Survival,
    TypingRain,
    FlashTyping,
    DailyChallenge,
    LongTextRace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveScreen {
    MainMenu,
    FingerPracticeMenu, // 한/영 선택 메뉴
    FingerPracticeLevelMenu { is_korean: bool }, // 레벨 선택 메뉴
    FingerPractice { level: usize, is_korean: bool }, // 연습 스크린
    WordPracticeMenu,
    WordPractice { is_korean: bool },
    SentencePracticeMenu,
    SentencePractice { is_korean: bool },
    Stats,
    // 게임 모드 화면들
    GameModeMenu,
    GameLanguageMenu { game_type: GameType },
    GameTimeSelect { is_korean: bool },
    TimeAttack { is_korean: bool },
    Survival { is_korean: bool },
    TypingRain { is_korean: bool },
    FlashTyping { is_korean: bool },
    DailyChallenge { is_korean: bool },
    LongTextRaceMenu { is_korean: bool },
    LongTextRace { is_korean: bool, text_idx: usize },
    GameOver { game_type: GameType, is_korean: bool },
}

/// 타자 레인 모드의 떨어지는 단어
pub struct RainWord {
    pub text: String,
    pub column: u16,
    pub row: f32,
    pub typed_len: usize,
    pub active: bool,
    pub destroyed: bool,
}

pub struct App {
    pub active_screen: ActiveScreen,
    pub storage: StorageManager,
    
    // 현재 연습 중인 텍스트 관련
    pub target_text: String,
    pub input_automata: HangulAutomata,
    
    // 낱말 연습을 위한 단어 세트 관리
    pub word_list: Vec<String>,
    pub current_word_idx: usize,
    
    // 문장 연습을 위한 스크롤 및 인덱스 관리
    pub sentence_list: Vec<String>,
    pub current_sentence_idx: usize,
    
    // 메타데이터 및 분석 기록용
    pub start_time: Option<Instant>,
    pub elapsed_time: Duration,
    pub accumulated_strokes: usize,
    pub total_errors: usize,
    pub wrong_keys_map: HashMap<char, usize>,
    
    // 통계 표시용 데이터 캐시
    pub cached_frequent_errors: Vec<(char, usize)>,
    
    // 화면 메뉴 선택용 인덱스
    pub menu_selected_idx: usize,

    // ── 게임 모드 공통 상태 ──
    pub game_mode_score: usize,
    pub game_mode_combo: usize,
    pub game_mode_max_combo: usize,
    pub game_mode_lives: u8,
    pub game_mode_round: usize,
    pub game_deadline: Option<Instant>,
    pub game_time_limit_secs: u64,
    pub game_words_correct: usize,
    pub game_words_total: usize,

    // ── 플래시 타이핑 상태 ──
    pub flash_visible: bool,
    pub flash_timer: Option<Instant>,
    pub flash_duration_ms: u64,
    pub flash_answer_shown: bool,
    pub flash_was_correct: Option<bool>,
    pub flash_response_times: Vec<f64>,

    // ── 타자 레인 상태 ──
    pub rain_words: Vec<RainWord>,
    pub rain_last_tick: Option<Instant>,
    pub rain_tick_ms: u64,
    pub rain_spawn_counter: usize,
    pub rain_active_idx: Option<usize>,
    pub rain_screen_width: u16,
    pub rain_screen_height: u16,

    // ── 데일리 챌린지 상태 ──
    pub daily_challenge_completed: bool,
    pub daily_best_score: Option<usize>,

    // ── 긴 글 레이스 상태 ──
    pub long_text_paragraphs: Vec<String>,
    pub long_text_current_para_idx: usize,
    pub long_text_title: String,
    pub long_text_cpm_history: Vec<usize>,
    pub long_text_selected_idx: usize,
    /// 마지막으로 CPM 히스토리를 기록한 경과 초 (중복 샘플 방지)
    pub long_text_last_cpm_sec: u64,
    /// 타자 레인 난이도 가속을 마지막으로 적용한 라운드
    pub rain_last_speed_round: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_screen: ActiveScreen::MainMenu,
            storage: StorageManager::new(),
            target_text: String::new(),
            input_automata: HangulAutomata::new(),
            word_list: Vec::new(),
            current_word_idx: 0,
            sentence_list: Vec::new(),
            current_sentence_idx: 0,
            start_time: None,
            elapsed_time: Duration::default(),
            accumulated_strokes: 0,
            total_errors: 0,
            wrong_keys_map: HashMap::new(),
            cached_frequent_errors: Vec::new(),
            menu_selected_idx: 0,
            // 게임 모드 공통
            game_mode_score: 0,
            game_mode_combo: 0,
            game_mode_max_combo: 0,
            game_mode_lives: 5,
            game_mode_round: 0,
            game_deadline: None,
            game_time_limit_secs: 60,
            game_words_correct: 0,
            game_words_total: 0,
            // 플래시 타이핑
            flash_visible: false,
            flash_timer: None,
            flash_duration_ms: 2000,
            flash_answer_shown: false,
            flash_was_correct: None,
            flash_response_times: Vec::new(),
            // 타자 레인
            rain_words: Vec::new(),
            rain_last_tick: None,
            rain_tick_ms: 150,
            rain_spawn_counter: 0,
            rain_active_idx: None,
            rain_screen_width: 80,
            rain_screen_height: 20,
            // 데일리 챌린지
            daily_challenge_completed: false,
            daily_best_score: None,
            // 긴 글 레이스
            long_text_paragraphs: Vec::new(),
            long_text_current_para_idx: 0,
            long_text_title: String::new(),
            long_text_cpm_history: Vec::new(),
            long_text_selected_idx: 0,
            long_text_last_cpm_sec: 0,
            rain_last_speed_round: 0,
        }
    }

    /// 공통 입력: 자모 접두사 기반 오타 판정.
    /// 유효→무효로 바뀌는 순간에만 오타 1회 기록 (연타 시 정확도 급락 방지).
    pub fn push_game_char(&mut self, c: char) {
        let before = self.input_automata.get_text();
        let was_valid = before.is_empty()
            || crate::hangeul::is_input_prefix_of(&before, &self.target_text);

        let idx = self.input_automata.expected_char_index();
        let expected_char = self.target_text.chars().nth(idx);
        self.input_automata.push_char(c, expected_char);

        let typed = self.input_automata.get_text();
        let now_valid = crate::hangeul::is_input_prefix_of(&typed, &self.target_text);
        if was_valid && !now_valid {
            self.record_error(expected_char.unwrap_or(c), c);
        }
    }

    /// 다음 타깃으로 넘어가되 타이머·오타 통계는 유지 (연속 연습 세션용)
    pub fn advance_practice_target(&mut self, target: String) {
        self.accumulated_strokes += self.input_automata.get_strokes();
        self.target_text = target;
        self.input_automata.clear();
        self.update_automata_modes();
    }

    /// 목표 텍스트를 정확히 모두 입력했는지 (자모 완전 일치)
    pub fn is_target_fully_typed(&self) -> bool {
        crate::hangeul::is_input_exact_match(&self.input_automata.get_text(), &self.target_text)
    }

    /// 게임 중 지울 입력이 있는지 (Esc로 초기화 가능 여부)
    pub fn has_pending_game_input(&self) -> bool {
        !self.input_automata.get_text().is_empty() || self.rain_active_idx.is_some()
    }

    /// 게임 모드 현재 입력 초기화 (Esc). 타자 레인이면 활성 단어도 해제.
    /// 목표 단어/문단 자체는 유지한다 (레인 제외).
    pub fn clear_game_input(&mut self) {
        let is_rain = matches!(self.active_screen, ActiveScreen::TypingRain { .. });
        self.input_automata.clear();

        if is_rain {
            if let Some(idx) = self.rain_active_idx {
                if let Some(word) = self.rain_words.get_mut(idx) {
                    word.active = false;
                    word.typed_len = 0;
                }
            }
            self.rain_active_idx = None;
            self.target_text.clear();
        } else {
            // clear()가 english_mode 등을 리셋하므로 현재 타깃 기준으로 복구
            self.update_automata_modes();
        }
    }

    /// 현재 target_text 에 따라 오토마타의 모드(raw_jamo_mode, english_mode)를 갱신
    pub fn update_automata_modes(&mut self) {
        let target = &self.target_text;
        
        // 완성형 한글 마디(가~힣)가 포함되어 있는지 검사
        let has_complete_hangul = target.chars().any(|c| {
            let code = c as u32;
            (0xAC00..=0xD7A3).contains(&code)
        });
        
        // 단독 한글 자모가 포함되어 있는지 검사 (호환 한글 자모 영역: 0x3130 ~ 0x318F)
        let has_jamo = target.chars().any(|c| {
            let code = c as u32;
            (0x3130..=0x318F).contains(&code)
        });

        // 완성형 한글은 없는데 자모가 포함된 경우 (예: 자리 연습 Level 1 등) 결합 없는 모드 활성화
        self.input_automata.raw_jamo_mode = !has_complete_hangul && has_jamo;

        // 한글 글자마디도 없고 단독 자모도 없는 경우 (순수 영문/숫자/기호 연습 등) 영문 바이패스 활성화
        self.input_automata.english_mode = !has_complete_hangul && !has_jamo;
    }

    /// 타자 연습 세션 시작 시 초기화
    pub fn start_practice_session(&mut self, target: String) {
        self.target_text = target;
        self.input_automata.clear();
        
        self.update_automata_modes();

        self.start_time = None;
        self.elapsed_time = Duration::default();
        self.accumulated_strokes = 0;
        self.total_errors = 0;
        self.wrong_keys_map.clear();
    }

    /// 분당 타수 (CPM) 계산
    pub fn get_cpm(&self) -> usize {
        let secs = self.elapsed_time.as_secs_f64();
        if secs < 0.5 {
            return 0;
        }
        let total_strokes = self.accumulated_strokes + self.input_automata.get_strokes();
        ((total_strokes as f64 / secs) * 60.0) as usize
    }

    /// 정확도(%) 계산
    pub fn get_accuracy(&self) -> f64 {
        let total_strokes = self.accumulated_strokes + self.input_automata.get_strokes();
        if total_strokes == 0 {
            return 100.0;
        }
        let correct_strokes = total_strokes.saturating_sub(self.total_errors);
        (correct_strokes as f64 / total_strokes as f64) * 100.0
    }

    /// 키가 틀렸을 때의 오타 카운트 및 틀린 키 분석
    pub fn record_error(&mut self, _expected: char, typed_raw: char) {
        self.total_errors += 1;
        
        // 영문 키 -> 한글 매핑하여 한글 글자 기준으로 틀린 키 기록
        let key_char = if let Some(jamo) = map_qwerty_to_jamo(typed_raw) {
            jamo
        } else {
            typed_raw
        };
        
        *self.wrong_keys_map.entry(key_char).or_insert(0) += 1;
    }

    /// 연습 세션 종료 후 기록 저장
    pub fn save_session_record(&mut self, mode_name: &str, lang_name: &str) {
        self.update_elapsed_time();
        let date_str = chrono::Local::now().to_rfc3339();
        let record = PracticeRecord {
            date: date_str,
            mode: mode_name.to_string(),
            language: lang_name.to_string(),
            cpm: self.get_cpm(),
            accuracy: self.get_accuracy(),
            duration_secs: self.elapsed_time.as_secs(),
            wrong_keys: self.wrong_keys_map.clone(),
        };

        if let Err(e) = self.storage.add_record(record) {
            eprintln!("[taja-cli] 기록 저장 실패: {}", e);
        }
    }
}
