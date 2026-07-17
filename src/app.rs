use crate::hangeul::{HangulAutomata, map_qwerty_to_jamo};
use crate::storage::{StorageManager, PracticeRecord};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

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

    /// 타이머 시작
    pub fn ensure_timer_started(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    /// 타이머가 아직 돌아가는 중인지
    pub fn is_timer_running(&self) -> bool {
        self.start_time.is_some()
    }

    /// 경과 시간 갱신 (타이머가 돌아가는 중에만 반영)
    pub fn update_elapsed_time(&mut self) {
        if let Some(start) = self.start_time {
            self.elapsed_time = start.elapsed();
        }
    }

    /// 타이머 정지: 최종 경과를 고정하고 start_time을 제거해 이후 갱신을 막는다.
    /// 세션/게임 종료·중단 시 반드시 호출한다.
    pub fn stop_timer(&mut self) {
        if let Some(start) = self.start_time.take() {
            self.elapsed_time = start.elapsed();
        }
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

    // --- 자리 연습 데이터 정의 ---
    pub fn get_finger_practice_target(level: usize, is_korean: bool) -> String {
        let words = if is_korean {
            match level {
                1 => vec!["ㅁㄴㅇㄹ", "ㅓㅏㅣ;", "ㄹㅇㄴㅁ", "ㅓㅏㅣ;", "ㅁㄴㅇㄹㅓㅏㅣ;", "ㅇㄹㄴㅁㅣㅏㅓ;"],
                2 => vec!["ㅎㅗㅜ", "ㅁㄴㅇㄹㅎ", "ㅓㅏㅣ;ㅗㅜ", "하하", "호호", "우우", "나나", "다다", "로로"],
                3 => vec!["ㅂㅈㄷㄱㅅ", "ㅛㅕㅑㅐㅔ", "ㅂㄱㄷㅈㅅ", "ㅕㅛㅐㅑㅔ", "가방", "사자", "도끼", "벼루", "새조개"],
                4 => vec!["ㅋㅌㅊㅍ", "ㅠㅜㅡ", "ㅋㅍㅌㅊ", "ㅡㅠㅜ", "카드", "파도", "코트", "토끼", "참새", "하늘"],
                5 => vec!["12345", "67890", "-=[]\\", ";'", ",./", "102938", "4756", "sys.argv[0]", "1 + 2 = 3"],
                6 => vec!["ㅃㅉㄸㄲㅆ", "ㅒㅖ", "아빠", "짜장면", "떡꼬치", "쓰레기", "꼬마", "얘기", "계란"],
                _ => vec!["ㅁㄴㅇㄹ"],
            }
        } else {
            match level {
                1 => vec!["asdf", "jkl;", "fdsa", "jkl;", "ask", "dad", "lad", "salad", "sad", "fall", "flask"],
                2 => vec!["qwer", "uiop", "quiet", "power", "write", "wire", "peak", "keep", "peer", "route", "trip"],
                3 => vec!["zxcv", "m,./", "zoom", "cave", "zero", "move", "class", "voice", "music", "box", "zone"],
                4 => vec!["Hello", "World", "Rust", "Linux", "Code", "Type", "System", "Dynamic", "Static", "Gemini"],
                5 => vec!["12345", "67890", "!@#$", "%^&*", "()_+", "{}|", ":\"<>?", "[]\\;',./"],
                _ => vec!["asdf"],
            }
        };
        
        let mut rng = thread_rng();
        let mut selected = words.clone();
        selected.shuffle(&mut rng);
        selected.join(" ")
    }

    // --- 낱말 연습용 단어 세트 로드 ---
    pub fn setup_word_practice(&mut self, is_korean: bool) {
        let words = if is_korean {
            vec![
                // 기존 단어
                "하늘", "바다", "나무", "구름", "태양", "바람", "사람", "우주", "도시", "강물",
                "노래", "마음", "봄날", "여름", "가을", "겨울", "행복", "사랑", "미래", "우정",
                "아침", "저녁", "시간", "기억", "평화", "희망", "노력", "열정", "도전", "성공",
                "세계", "지도", "기쁨", "슬픔", "위로", "용기", "감사", "약속", "여행", "예술",
                "소리", "빛깔", "향기", "미소", "눈물", "길목", "바위", "모래", "조개", "낙엽",
                "학교", "친구", "공부", "책장", "연필", "노트", "교실", "운동장", "농구", "축구",
                "야구", "취미", "영화", "소설", "시인", "낭만", "달빛", "별빛", "은하수", "성운",
                "태양계", "행성", "혜성", "지구", "대륙", "섬나라", "파도", "수평선", "갈매기", "고래",
                "상어", "돌고래", "물고기", "산호초", "조약돌", "모닥불", "캠핑", "텐트", "배낭", "숲길",
                "다람쥐", "토끼", "사슴", "호랑이", "독수리", "비둘기", "참새", "제비", "나비", "벌꿀",
                "꽃잎", "정원", "공원", "분수", "벤치", "산책", "자전거", "자동차", "기차", "비행기",
                "돛단배", "항구", "선장", "지도자", "영웅", "역사", "유적", "박물관", "도서관", "책방",
                "그림", "조각", "음악", "피아노", "바이올린", "드럼", "기타", "노래방", "콘서트", "연극",
                "뮤지컬", "축제", "불꽃놀이", "명절", "추억", "일기", "소원", "꿈나라", "천사", "동화",
                "신화", "전설", "보물", "모험", "신비", "기적", "환상", "현실", "진실", "정의",
                "자유", "평등", "협동", "나눔", "배려", "이해", "용서", "화해", "존중", "신뢰",
                "성실", "책임", "지혜", "지식", "생각", "사색", "명상", "휴식", "건강", "웃음",
                "행운", "환희", "설렘", "고독", "분노", "극복", "달성", "보람", "만족", "축하",
                "응원", "격려", "칭찬", "승리", "아기", "가족", "부모", "형제", "이웃", "마을",
                // 신규 확장 단어 (자연 & 동물 & 식물)
                "사자", "코끼리", "기린", "여우", "늑대", "원숭이", "팬더", "펭귄", "부엉이", "까마귀",
                "두루미", "오리", "거위", "백조", "앵무새", "개구리", "거북이", "악어", "도마뱀", "달팽이",
                "잠자리", "매미", "무당벌레", "사마귀", "장수풍뎅이", "반딧불이", "장미", "튤립", "백합", "해바라기",
                "무궁화", "벚꽃", "진달래", "개나리", "단풍나무", "은행나무", "소나무", "대나무", "버드나무", "코스모스",
                "민들레", "나팔꽃", "연꽃", "선인장", "안개꽃", "카네이션", "국화", "라벤더", "허브", "이끼",
                // 신규 확장 단어 (음식 & 일상)
                "밥상", "김치", "비빔밥", "불고기", "삼겹살", "갈비", "냉면", "떡볶이", "순대", "튀김",
                "라면", "국수", "짜장면", "짬뽕", "탕수육", "만두", "피자", "햄버거", "치킨", "샐러드",
                "샌드위치", "스파게티", "스테이크", "초밥", "우동", "라멘", "돈가스", "카레", "빵집", "과자",
                "초콜릿", "사탕", "젤리", "아이스크림", "커피", "녹차", "홍차", "우유", "두유", "주스",
                "사과", "배", "귤", "감", "밤", "대추", "호두", "땅콩", "아몬드", "수박",
                "참외", "멜론", "딸기", "포도", "복숭아", "자두", "살구", "체리", "토마토", "바나나",
                "파인애플", "망고", "키위", "블루베리", "레몬", "오렌지", "자몽", "코코넛", "무화과", "석류",
                // 신규 확장 단어 (의류 & 가구 & 물건)
                "모자", "안경", "셔츠", "바지", "치마", "원피스", "코트", "패딩", "양말", "신발",
                "구두", "운동화", "슬리퍼", "장갑", "목도리", "우산", "가방", "지갑", "시계", "벨트",
                "침대", "이불", "베개", "책상", "의자", "옷장", "서랍장", "화장대", "책장", "소파",
                "식탁", "싱크대", "냉장고", "세탁기", "건조기", "청소기", "에어컨", "선풍기", "텔레비전", "컴퓨터",
                "노트북", "키보드", "마우스", "스피커", "이어폰", "헤드폰", "스마트폰", "태블릿", "카메라", "렌즈",
                // 신규 확장 단어 (도시 & 과학 & 교통)
                "도서관", "미술관", "박물관", "체육관", "수영장", "영화관", "공연장", "경기장", "놀이공원", "동물원",
                "식물원", "백화점", "마트", "시장", "약국", "병원", "은행", "우체국", "소방서", "경찰서",
                "구청", "시청", "법원", "대학교", "고등학교", "중학교", "초등학교", "유치원", "학원", "독서실",
                "버스", "택시", "지하철", "열차", "고속열차", "오토바이", "킥보드", "헬리콥터", "전투기", "우주선",
                "인공위성", "로켓", "잠수함", "크루즈선", "화물선", "소방차", "경찰차", "구급차", "포클레인", "트럭",
                "엔진", "모터", "배터리", "발전소", "태양광", "풍력", "수력", "원자력", "전기", "전자",
                "반도체", "인공지능", "로봇", "드론", "네트워크", "인터넷", "소프트웨어", "프로그램", "알고리즘", "데이터",
                // 신규 확장 단어 (학문 & 직업)
                "수학", "물리학", "화학", "생물학", "지구과학", "천문학", "의학", "약학", "공학", "컴퓨터공학",
                "국어", "영어", "역사학", "지리학", "철학", "심리학", "사회학", "정치학", "경제학", "경영학",
                "법학", "행정학", "교육학", "체육학", "예술학", "디자인", "건축학", "고고학", "인류학", "통계학",
                "교사", "교수", "의사", "간호사", "약사", "수의사", "변호사", "판사", "검사", "변리사",
                "회계사", "세무사", "건축가", "디자이너", "화가", "조각가", "음악가", "작곡가", "지휘자", "가수",
                "배우", "감독", "작가", "시인", "소설가", "기자", "아나운서", "프로듀서", "엔지니어", "개발자",
                "프로그래머", "기획자", "연구원", "과학자", "요리사", "제빵사", "바리스타", "소믈리에", "승무원", "조종사",
                "군인", "경찰관", "소방관", "공무원", "정치인", "외교관", "통역사", "번역가", "농부", "어부",
                // 신규 확장 단어 (감정 & 가치관 & 형용사)
                "즐거움", "신바람", "두근두근", "설레임", "안도감", "평온함", "유쾌함", "상쾌함", "상상력", "창의성",
                "자긍심", "성취감", "자신감", "열정적", "헌신적", "도전 정신", "모험심", "호기심", "동정심", "애국심",
                "시민 의식", "도덕성", "투명성", "공정함", "정직함", "겸손함", "배려심", "협동심", "인내심", "자제력",
                "집중력", "기억력", "이해력", "창조적", "혁신적", "합리적", "논리적", "과학적", "예술적", "대중적",
                "전통적", "현대적", "글로벌", "로컬", "자연 친화", "환경 보호", "지속 가능", "다양성", "포용성", "개방성",
                "행복감", "안정감", "친밀감", "신뢰도", "투명도", "만족도", "기여도", "참여도", "성장통", "희망 사항",
                // 추가 250단어 (일상, 물건, 감정, 생각 등 추가)
                "가위", "풀", "테이프", "클립", "자석", "핀", "압정", "칠판", "분필", "지우개",
                "형광펜", "볼펜", "샤프", "연필깎이", "필통", "자", "각도기", "컴퍼스", "돋보기", "현미경",
                "망원경", "나침반", "지도", "지구본", "저울", "온도계", "기압계", "습도계", "시계바늘", "초침",
                "거울", "유리창", "커튼", "블라인드", "카펫", "매트", "쿠션", "방석", "벽지", "장판",
                "천장", "기둥", "대문", "창문", "현관", "베란다", "보일러", "라디에이터", "난로", "벽난로",
                "가스레인지", "인덕션", "전자레인지", "식기세척기", "토스터", "믹서기", "전기포트", "압력밥솥", "냄비", "프라이팬",
                "칼", "도마", "국자", "뒤집개", "집게", "수저", "젓가락", "숟가락", "포크", "나이프",
                "접시", "대접", "공기", "종지", "컵", "머그잔", "텀블러", "물병", "찬장", "식탁보",
                "수건", "비누", "샴푸", "린스", "바디워시", "치약", "칫솔", "면도기", "화장지", "물티슈",
                "빗", "헤어드라이어", "고데기", "화장품", "로션", "스킨", "선크림", "향수", "손톱깎이", "귀이개",
                "바늘", "실", "단추", "지퍼", "가위질", "바느질", "재봉틀", "다리미", "다리미판", "빨래집게",
                "빨래건조대", "옷걸이", "수납함", "상자", "바구니", "비닐봉지", "쓰레기통", "분리수거함", "빗자루", "쓰레받기",
                "먼지털이", "걸레", "대걸레", "양동이", "호스", "샤워기", "수도꼭지", "배수구", "환풍기", "도어락",
                "열쇠", "자물쇠", "체인", "로프", "끈", "고무줄", "테이프", "본드", "풀칠", "페인트",
                "붓", "롤러", "사포", "망치", "못", "나사", "드라이버", "스패너", "펜치", "니퍼",
                "톱", "대패", "송곳", "줄자", "수평계", "사다리", "리어카", "수레", "지게차", "크레인",
                "기쁨", "즐거움", "유쾌", "통쾌", "상쾌", "명랑", "활기", "생기", "용기", "자신감",
                "기대", "설렘", "희망", "바람", "소망", "소원", "꿈", "이상", "동경", "그리움",
                "사랑", "자비", "은혜", "감사", "고마움", "만족", "보람", "긍지", "자부심", "안도",
                "평온", "안정", "여유", "휴식", "위안", "위로", "동정", "연민", "이해", "용서",
                "화해", "협동", "우정", "신뢰", "믿음", "의리", "충성", "효도", "공경", "배려",
                "양보", "친절", "온정", "따뜻함", "순수", "솔직", "정직", "진실", "성실", "근면",
                "인내", "끈기", "노력", "열정", "집념", "의지", "지혜", "슬기", "재치", "유머",
                "독창성", "개성", "매력", "아름다움", "우아함", "세련됨", "자연스러움", "평범함", "특별함", "소중함",
                "영원함", "순간", "추억", "기억", "흔적", "발자국", "그림자", "메아리", "울림", "기적"
            ].iter().map(|&s| s.to_string()).collect::<Vec<_>>()
        } else {
            // 영문 단어 연습 - 개발자 이스터에그 포함!
            let mut list = vec![
                // 기존 단어
                "apple", "banana", "cherry", "orange", "grape", "melon", "peach", "berry", "lemon", "lime",
                "computer", "keyboard", "monitor", "mouse", "screen", "terminal", "console", "network", "system", "database",
                "science", "history", "subject", "object", "english", "korean", "travel", "nature", "forest", "mountain",
                "summer", "winter", "spring", "autumn", "weather", "yellow", "purple", "orange", "silver", "golden",
                "sunshine", "starlight", "moonlight", "galaxy", "universe", "planet", "earth", "ocean", "island", "beach",
                "dolphin", "whale", "seagull", "pebble", "campfire", "camping", "backpack", "pathway", "squirrel", "rabbit",
                "tiger", "eagle", "butterfly", "flower", "garden", "park", "fountain", "bicycle", "vehicle", "train",
                "airplane", "harbor", "captain", "leader", "history", "museum", "library", "bookstore", "painting", "sculpture",
                "music", "piano", "violin", "guitar", "concert", "theater", "festival", "fireworks", "memory", "diary",
                "wish", "dream", "angel", "fairytale", "myth", "legend", "treasure", "adventure", "mystery", "miracle",
                "fantasy", "reality", "truth", "justice", "freedom", "equality", "peace", "sharing", "respect", "trust",
                "wisdom", "knowledge", "thinking", "meditation", "relax", "health", "laughter", "happiness", "fortune", "excitement",
                "solitude", "sadness", "anger", "victory", "reward", "satisfaction", "gratitude", "welcome", "support", "courage",
                "helper", "partner", "colleague", "family", "parent", "sibling", "teacher", "student", "doctor", "engineer",
                "artist", "writer", "poet", "novel", "story", "drama", "stage", "actors", "camera", "picture", "album",
                "photo", "frame", "mirror", "window", "door", "house", "room", "kitchen", "table", "chair", "clock",
                "phone", "internet", "website", "email", "message", "letter", "stamp", "paper", "book", "pen", "pencil",
                // 신규 확장 단어 (자연 & 생물 & 우주)
                "lion", "elephant", "giraffe", "fox", "wolf", "monkey", "panda", "penguin", "owl", "crow",
                "swan", "duck", "goose", "parrot", "frog", "turtle", "crocodile", "lizard", "snail", "dragonfly",
                "beetle", "butterfly", "rose", "tulip", "lily", "sunflower", "maple", "pine", "bamboo", "willow",
                "cosmos", "dandelion", "cactus", "orchid", "carnation", "lavender", "rosemary", "basil", "mint", "moss",
                "star", "nebula", "comet", "asteroid", "orbit", "gravity", "eclipse", "mercury", "venus", "mars",
                "jupiter", "saturn", "uranus", "neptune", "pluto", "crater", "telescope", "astronaut", "satellite", "galaxy",
                // 신규 확장 단어 (음식 & 주방)
                "bread", "butter", "cheese", "yogurt", "cream", "ice", "juice", "water", "tea", "coffee",
                "chocolate", "candy", "cookie", "cake", "pie", "donut", "pizza", "burger", "pasta", "salad",
                "soup", "steak", "sushi", "rice", "noodle", "curry", "stew", "sauce", "spices", "pepper",
                "salt", "sugar", "honey", "onion", "garlic", "potato", "carrot", "tomato", "cucumber", "cabbage",
                "spinach", "broccoli", "mushroom", "pumpkin", "bean", "corn", "nut", "fruit", "berry", "grape",
                // 신규 확장 단어 (의류 & 리빙 & 가전)
                "hat", "glasses", "shirt", "pants", "skirt", "dress", "coat", "jacket", "socks", "shoes",
                "boots", "sneakers", "slippers", "gloves", "scarf", "umbrella", "bag", "wallet", "watch", "belt",
                "bed", "blanket", "pillow", "desk", "chair", "closet", "drawer", "sofa", "table", "cabinet",
                "fridge", "dryer", "cleaner", "aircon", "fan", "tv", "laptop", "tablet", "phone", "camera",
                "battery", "charger", "cable", "plug", "socket", "switch", "lamp", "bulb", "mirror", "window",
                // 신규 확장 단어 (도시 & 기관 & 사회)
                "city", "town", "village", "street", "road", "highway", "bridge", "tunnel", "station", "airport",
                "harbor", "port", "office", "factory", "store", "shop", "market", "mall", "bank", "hospital",
                "pharmacy", "clinic", "school", "college", "academy", "museum", "gallery", "library", "theater", "cinema",
                "park", "square", "court", "senate", "embassy", "police", "firehouse", "post", "hotel", "restaurant",
                // 신규 확장 단어 (학문 & 산업 & 기술)
                "math", "physics", "chemistry", "biology", "geology", "astronomy", "medicine", "pharmacy", "engineering", "coding",
                "history", "geography", "philosophy", "psychology", "sociology", "politics", "economy", "law", "education", "design",
                "engine", "motor", "robot", "drone", "network", "internet", "software", "program", "algorithm", "data",
                "silicon", "hardware", "firmware", "sensor", "laser", "radar", "sonar", "optics", "acoustics", "statics",
                // 추가 250단어 (일상, 사물, 감정 등)
                "scissor", "glue", "tape", "clip", "magnet", "pin", "chalk", "eraser", "pencil", "ruler",
                "compass", "scale", "sensor", "lens", "glass", "window", "door", "wall", "floor", "ceiling",
                "roof", "gate", "fence", "yard", "garden", "flowerpot", "fountain", "bench", "path", "streetlamp",
                "kettle", "teapot", "cup", "mug", "glass", "bottle", "plate", "bowl", "spoon", "fork",
                "knife", "napkin", "tablecloth", "tray", "apron", "pot", "pan", "oven", "grill", "toaster",
                "soap", "shampoo", "toothpaste", "toothbrush", "razor", "towel", "comb", "dryer", "perfume", "lotion",
                "needle", "thread", "button", "zipper", "sewing", "iron", "hanger", "box", "basket", "bag",
                "broom", "dustpan", "mop", "bucket", "hose", "shower", "tap", "valve", "lock", "key",
                "chain", "rope", "string", "wire", "cable", "screw", "bolt", "nut", "nail", "hammer",
                "saw", "drill", "file", "ruler", "level", "ladder", "wagon", "cart", "crane", "loader",
                "joy", "pleasure", "delight", "glee", "bliss", "cheer", "vigor", "courage", "boldness", "trust",
                "hope", "wish", "desire", "dream", "vision", "aspiration", "nostalgia", "longing", "memory", "trace",
                "love", "mercy", "grace", "gratitude", "thanks", "satisfaction", "pride", "glory", "honor", "peace",
                "calm", "serenity", "rest", "relief", "comfort", "sympathy", "pity", "empathy", "pardon", "mercy",
                "harmony", "friendship", "loyalty", "faith", "devotion", "piety", "respect", "care", "warmth", "kindness",
                "honesty", "truth", "sincerity", "diligence", "patience", "grit", "passion", "willpower", "wisdom", "wits",
                "humor", "talent", "genius", "charm", "beauty", "elegance", "grace", "novelty", "miracle", "wonder",
                "moment", "instant", "decade", "century", "epoch", "era", "future", "past", "present", "eternal"
            ].iter().map(|&s| s.to_string()).collect::<Vec<_>>();
            
            // 개발자 용어 이스터 에그들
            let dev_easter_eggs = vec![
                "String", "Array", "Java", "JavaScript", "SQL", "nullptr", "async", "await",
                "struct", "impl", "let_mut", "panic!", "unwrap", "println!", "Option", "Result",
                "git_commit", "stack_overflow", "NullPointerException", "undefined", "const", "vector"
            ];
            
            for &egg in dev_easter_eggs.iter() {
                if rand::random::<f64>() < 0.3 {
                    list.push(egg.to_string());
                }
            }
            list
        };
        
        let mut rng = thread_rng();
        let mut shuffled = words;
        shuffled.shuffle(&mut rng);
        
        self.word_list = shuffled.into_iter().take(20).collect(); // 20단어 한 세트
        self.current_word_idx = 0;
        
        let first_word = self.word_list[0].clone();
        self.start_practice_session(first_word);
    }

    /// 낱말 연습에서 다음 단어로 넘어가기
    pub fn next_word(&mut self) -> bool {
        // 현재 단어 타수 및 경과 시간 합산
        self.accumulated_strokes += self.input_automata.get_strokes();
        
        self.current_word_idx += 1;
        if self.current_word_idx >= self.word_list.len() {
            return false; // 세션 끝
        }
        
        let next_w = self.word_list[self.current_word_idx].clone();
        self.input_automata.clear();
        self.target_text = next_w;
        self.update_automata_modes();
        true
    }

    // --- 문장 연습용 데이터 로드 ---
    pub fn setup_sentence_practice(&mut self, is_korean: bool) {
        let sentences = if is_korean {
            vec![
                // 1~10
                "동해 물과 백두산이 마르고 닳도록 하느님이 보우하사 우리나라 만세",
                "남산 위에 저 소나무 철갑을 두른 듯 바람 서리 불변함은 우리 기상일세",
                "가을 하늘 공활한데 높고 구름 없이 밝은 달은 우리 가슴 일편단심일세",
                "이 기상과 이 맘으로 충성을 다하여 괴로우나 즐거우나 나라 사랑하세",
                "나랏말싸미 듕귁에 달아 문자와로 서르 사맛디 아니할쎄 이런 젼차로 어린 백셩이 니르고져 홇배이셔도",
                "아름다운 이 땅에 금수강산에 단군할아버지가 터 잡으시고 홍익인간 뜻으로 나라 세우니 대대손손 훌륭한 인물도 많아",
                "별을 노래하는 마음으로 모든 죽어가는 것들을 사랑해야지 그리고 나한테 주어진 길을 걸어가야겠다",
                "삶이 그대를 속일지라도 슬퍼하거나 노하지 말라 우울한 날들을 견디면 믿으라 기쁨의 날이 오리니",
                "대한민국은 민주공화국이다 대한민국의 주권은 국민에게 있고 모든 권력은 국민으로부터 나온다",
                "가는 말이 고와야 오는 말이 곱다 소 잃고 외양간 고친다 돌다리도 두들겨 보고 건너라",
                // 11~20
                "천 리 길도 한 걸음부터 시작된다 시작이 반이다 호랑이도 제 말 하면 온다",
                "벼는 익을수록 고개를 숙인다 아는 길도 물어가라 백지장도 맞들면 낫다",
                "인생은 짧고 예술은 길다 자신을 아는 것이 가장 위대한 지식이다",
                "어둠이 깊을수록 별은 더욱 빛난다 고난 뒤에 오는 낙이 진정한 기쁨이다",
                "오늘 할 일을 내일로 미루지 말라 시간은 아무도 기다려주지 않는다",
                "서로를 존중하고 배려하는 마음이 아름다운 공동체를 만든다",
                "실패는 성공의 어머니이다 포기하지 않는 자에게 기회가 온다",
                "배움에는 끝이 없다 매일 조금씩 성장하는 나를 발견해 보자",
                "가을바람에 흔들리는 코스모스처럼 우리의 마음도 가끔 흔들릴 때가 있다",
                "하늘 높이 날아오르는 새처럼 우리의 꿈도 드넓은 세상을 향해 뻗어가길",
                // 21~30
                "봄바람 휘날리며 흩날리는 벚꽃 잎이 울려 퍼질 이 거리를 둘이 걸어요",
                "기회는 노크하지 않는다 당신이 기회의 문을 두드려야 한다",
                "가장 어두운 밤도 언젠가는 끝나고 해는 다시 떠오를 것이다",
                "흔들리지 않고 피는 꽃이 어디 있으랴 이 세상 그 어떤 아름다운 꽃들도 다 흔들리며 피었나니",
                "자세히 보아야 예쁘다 오래 보아야 사랑스럽다 너도 그렇다",
                "네 장미꽃이 그토록 소중한 것은 네가 그 꽃을 위해 잃어버린 시간 때문이다",
                "중요한 것은 꺾이지 않는 마음이다 끝까지 포기하지 마라",
                "시간은 우리가 가진 가장 소중한 자산이며 결코 되돌릴 수 없다",
                "행복은 깊이 생각하는 자의 몫이며 나눌수록 커지는 신비한 보물이다",
                "성공한 사람이 되려 하기보다 가치 있는 사람이 되려고 노력하라",
                // 31~40
                "우리가 걷는 이 길이 비록 험난할지라도 끝내 웃으며 맞이할 내일이 있다",
                "작은 변화가 모여 큰 기적을 만든다 오늘부터 조금씩 나아가자",
                "나 자신을 사랑하는 법을 배우는 것이 세상에서 가장 훌륭한 사랑이다",
                "바다를 바라보며 넓은 마음을 배우고 산을 오르며 굳건한 의지를 다진다",
                "따뜻한 말 한마디가 누군가의 인생을 바꾸는 큰 힘이 될 수 있다",
                "바람이 불어오는 곳 그곳으로 가네 그대 머릿결 같은 나무 아래로",
                "행복의 한쪽 문이 닫히면 다른 쪽 문이 열린다 그러나 흔히 우리는 닫힌 문만 바라본다",
                "꿈을 꿀 수 있다면 그 꿈을 이룰 수도 있다 시작하는 용기를 가져라",
                "인간은 생각하는 갈대이다 비록 약하지만 사색을 통해 우주를 담는다",
                "오늘의 고난은 내일의 영광을 위한 밑거름이 될 뿐이다 기죽지 마라",
                // 41~50
                "푸른 하늘 은하수 하얀 쪽배에 계수나무 한 나무 토끼 한 마리",
                "강물이 흘러 흘러 바다로 가듯이 우리의 노력도 언젠가 결실을 맺는다",
                "가장 위대한 승리는 타인을 이기는 것이 아니라 자신을 극복하는 것이다",
                "독서는 앉아서 하는 가장 위대한 여행이며 생각의 지평을 넓혀준다",
                "친구는 제2의 자신이다 서로의 아픔을 보듬고 기쁨을 함께 나누자",
                "아침 이슬 머금은 풀잎처럼 우리의 마음도 늘 맑고 싱그럽게 유지되길",
                "인생이라는 도화지에 당신만의 아름다운 색깔로 꿈을 그려 나가라",
                "건강한 육체에 건강한 정신이 깃든다 매일 스스로를 소중히 가꾸자",
                "고요한 숲속의 물소리처럼 마음을 차분히 내려놓고 명상에 잠겨보자",
                "모든 순간이 꽃봉오리인 것을 내 열심에 따라 꽃피어날 것들을",
                // 51~60
                "정의는 반드시 승리하며 진실은 가려져도 언젠가 빛을 발하게 된다",
                "지혜로운 사람은 행동으로 증명하고 어리석은 사람은 말로만 과시한다",
                "시간을 아끼는 것은 인생을 사랑하는 법을 실천하는 가장 확실한 길이다",
                "따뜻한 미소 하나가 차가운 세상을 밝히는 등불이 될 수 있다",
                "어떤 분야든 매일 한 시간씩 투자하면 누구나 전문가가 될 수 있다",
                "새벽이 오기 직전이 가장 어둡다 조금만 더 버티면 빛이 올 것이다",
                "배려하는 마음은 향기로운 꽃과 같아서 주변을 모두 아름답게 물들인다",
                "포기하지 않는 집념이야말로 모든 위대한 업적의 공통된 열쇠이다",
                "매일 아침 눈을 뜰 때마다 새로운 기회가 주어짐에 감사하자",
                "당신이 오늘 심은 작은 씨앗이 내일 울창한 숲이 될 것입니다",
                // 61~70
                "자연은 서두르지 않지만 모든 것을 이룬다 우리도 조급해하지 말자",
                "사랑은 온전히 자신을 내어주는 것이며 대가를 바라지 않는 신비다",
                "역사를 잊은 민족에게 미래는 없다 선조들의 발자취를 돌아보라",
                "박물관의 오래된 유물 속에서 우리는 과거와 현재의 연결고리를 찾는다",
                "아름다운 시 구절 하나가 가슴 깊이 파고들어 평생의 위로가 된다",
                "피아노의 건반이 조화를 이루듯 우리의 삶도 일과 휴식의 균형이 필요하다",
                "콘서트홀을 가득 메운 열기처럼 우리의 청춘도 뜨겁게 타오르길",
                "어릴 적 읽던 동화책 속 모험은 여전히 우리 가슴속에 살아 숨 쉰다",
                "상상할 수 있는 모든 것은 현실이 될 수 있다 꿈을 제한하지 마라",
                "겸손은 사람을 가장 아름답게 만드는 향기이며 깊은 울림을 준다",
                // 71~80
                "지속 가능한 발전을 위해 우리는 자연과 공존하는 길을 찾아야 한다",
                "도전하지 않는 삶은 정체될 뿐이다 날개를 펴고 드넓은 하늘로 날아라",
                "서로 존중하는 마음이 가득할 때 비로소 진정한 평화가 싹튼다",
                "신뢰는 쌓기는 어렵지만 무너지기는 쉽다 매 순간 성실히 임하자",
                "책임감 있는 행동이 성숙한 어른을 만들고 사회를 올바르게 이끈다",
                "오늘 하루도 수고한 나 자신에게 격려와 찬사의 미소를 보내주자",
                "가족의 따뜻한 품은 세상 어떤 거친 풍파도 막아주는 방패가 된다",
                "어부의 그물에 걸린 물고기처럼 우리의 삶도 얽혀 있지만 풀 수 있다",
                "소방관들의 헌신적인 노고 덕분에 우리는 오늘도 안전한 하루를 보낸다",
                "반도체 강국의 위상을 넘어 인공지능 시대의 주역으로 우뚝 서자",
                // 81~90
                "소프트웨어 코딩은 논리적인 사색의 결과이며 새로운 세상을 창조한다",
                "매일 쓰는 일기 속에 나의 생각과 성장의 흔적들이 고스란히 담긴다",
                "우리가 꿈꾸는 기적은 멀리 있지 않다 매일의 작은 노력이 기적이다",
                "화가의 화폭에 담긴 찬란한 색채처럼 인생을 아름답게 칠해보자",
                "도서관의 고요한 공기 속에서 책장을 넘기는 소리가 마음을 채운다",
                "가을바람에 떨어지는 낙엽은 끝이 아니라 새로운 시작을 위한 준비다",
                "수평선 너머로 붉게 타오르는 노을을 바라보며 깊은 사색에 잠긴다",
                "우리가 나누는 작은 따뜻함이 모여 세상을 훈훈하게 변화시킬 것이다",
                "실패를 두려워하여 멈추기보다 전진하며 배우는 자세가 훌륭하다",
                "모든 배움은 호기심에서 출발하며 호기심은 우리를 성장하게 만든다",
                // 91~100
                "정직함은 가장 확실한 자산이며 인생의 길을 밝히는 나침반이다",
                "지나온 발자취를 돌아보며 현재를 점검하고 더 나은 미래를 설계하자",
                "자전거 페달을 밟으며 스치는 싱그러운 바람을 온몸으로 느껴보자",
                "여객선 창밖으로 펼쳐진 드넓은 바다를 보며 웅장한 꿈을 품는다",
                "매 순간 집중하며 최선을 다하는 태도가 결국 빛나는 성공을 이끈다",
                "따뜻한 온정이 넘치는 이웃들과 함께할 때 삶은 더욱 행복해진다",
                "조각가의 섬세한 손길을 거쳐 평범한 돌멩이가 예술작품으로 태어난다",
                "신비로운 밤하늘의 별자리들을 바라보며 드넓은 우주를 꿈꿔본다",
                "용기란 두려움이 없는 것이 아니라 두려움에도 불구하고 나아가는 것이다",
                "타인을 용서하는 것은 나 자신을 얽매인 사슬에서 자유롭게 풀어주는 길이다"
            ]
        } else {
            vec![
                // 1~10
                "The quick brown fox jumps over the lazy dog.",
                "To be or not to be, that is the question.",
                "In the beginning God created the heaven and the earth.",
                "All that glitters is not gold, often have you heard that told.",
                "I think, therefore I am. Cogito, ergo sum.",
                "Ask not what your country can do for you, ask what you can do for your country.",
                "That's one small step for a man, one giant leap for mankind.",
                "Live as if you were to die tomorrow. Learn as if you were to live forever.",
                "Success is not final, failure is not fatal: it is the courage to continue that counts.",
                "The only way to do great work is to love what you do.",
                // 11~20
                "In the middle of difficulty lies opportunity. Keep moving forward.",
                "Time is money, but it is also the most precious thing we can spend.",
                "Actions speak louder than words. Well begun is half done.",
                "Don't count the days, make the days count. Every moment is a fresh beginning.",
                "A friend in need is a friend indeed. Kindness is a language everyone understands.",
                "To love and be loved is to feel the sun from both sides.",
                "Believe you can and you're halfway there. Keep your dreams alive.",
                "Education is the most powerful weapon which you can use to change the world.",
                "Nothing is impossible, the word itself says I'm possible!",
                "Life is what happens when you're busy making other plans.",
                // 21~30
                "The journey of a thousand miles begins with a single step.",
                "Good things come to those who wait, but better things to those who go and get them.",
                "It is during our darkest moments that we must focus to see the light.",
                "Do not go where the path may lead, go instead where there is no path and leave a trail.",
                "Many of life's failures are people who did not realize how close they were to success.",
                "You only live once, but if you do it right, once is enough.",
                "Be yourself; everyone else is already taken. Stay true to your heart.",
                "Two roads diverged in a wood, and I took the one less traveled by.",
                "In three words I can sum up everything I've learned about life: it goes on.",
                "The best and most beautiful things in the world cannot be seen or touched.",
                // 31~40
                "Keep your face always toward the sunshine and shadows will fall behind you.",
                "Go confidently in the direction of your dreams. Live the life you've imagined.",
                "The only limit to our realization of tomorrow will be our doubts of today.",
                "If you want to live a happy life, tie it to a goal, not to people or things.",
                "Do not let making a living prevent you from making a life.",
                "Life is either a daring adventure or nothing at all.",
                "Strive not to be a success, but rather to be of value.",
                "You miss one hundred percent of the shots you don't take.",
                "The power of imagination makes us infinite and drives progress.",
                "Happiness is not something ready-made. It comes from your own actions.",
                // 41~50
                "It always seems impossible until it is done. Keep striving.",
                "It does not matter how slowly you go as long as you do not stop.",
                "Our greatest glory is not in never falling, but in rising every time we fall.",
                "The supreme art of war is to subdue the enemy without fighting.",
                "The truth is rarely pure and never simple. Strive to find it.",
                "Try to be a rainbow in someone else's cloud. Bring them hope.",
                "We may encounter many defeats but we must not be defeated.",
                "You must be the change you wish to see in the world.",
                "What we think, we become. Cultivate positive thoughts.",
                "A warm smile is the universal language of kindness and love.",
                // 51~60
                "A room without books is like a body without a soul. Read more.",
                "All generalizations are false, including this one. Think deeply.",
                "An unexamined life is not worth living. Explore your mind.",
                "Art is the lie that enables us to realize the truth.",
                "Be kind, for everyone you meet is fighting a harder battle.",
                "Beauty is in the eye of the beholder. Appreciate diversity.",
                "Change is the law of life. And those who look only to the past are certain to miss the future.",
                "Courage is grace under pressure. Stand tall against the storm.",
                "Creativity is intelligence having fun. Let your mind run free.",
                "Do what you can, with what you have, where you are.",
                // 61~70
                "Don't cry because it's over, smile because it happened.",
                "Dream big and dare to fail. Great achievements require great risks.",
                "Everything you can imagine is real. The mind is a canvas.",
                "Genius is one percent inspiration and ninety-nine percent perspiration.",
                "He who has a why to live can bear almost any how.",
                "If you tell the truth, you don't have to remember anything.",
                "Innovation distinguishes between a leader and a follower.",
                "Integrity is doing the right thing, even when no one is watching.",
                "Knowledge speaks, but wisdom listens. Learn from everyone.",
                "Life is ten percent what happens to you and ninety percent how you respond.",
                // 71~80
                "Logic will get you from A to B. Imagination will take you everywhere.",
                "Love all, trust a few, do wrong to none. Live in peace.",
                "Make each day your masterpiece. Paint your life with hope.",
                "No legacy is so rich as honesty. Be upright in all ways.",
                "Nothing is permanent in this wicked world, not even our troubles.",
                "One child, one teacher, one book, one pen can change the world.",
                "Patience is bitter, but its fruit is sweet. Endure and grow.",
                "Simplicity is the ultimate sophistication. Keep it clean and clear.",
                "The best way to predict your future is to create it.",
                "The only true wisdom is in knowing you know nothing.",
                // 81~90
                "There is no path to peace. Peace is the path. Walk with love.",
                "Think of all the beauty still left around you and be happy.",
                "Those who cannot change their minds cannot change anything.",
                "To love oneself is the beginning of a lifelong romance.",
                "Well done is better than well said. Let actions show.",
                "What lies behind us and what lies before us are tiny matters compared to what lies within us.",
                "Whatever you are, be a good one. Strive for excellence.",
                "Whenever you find yourself on the side of the majority, it is time to pause and reflect.",
                "With the new day comes new strength and new thoughts.",
                "Yesterday is history, tomorrow is a mystery, today is a gift."
            ]
        };
        
        let mut rng = thread_rng();
        let mut shuffled: Vec<String> = sentences.iter().map(|&s| s.to_string()).collect();
        shuffled.shuffle(&mut rng);
        
        self.sentence_list = shuffled;
        self.current_sentence_idx = 0;
        
        let first_sentence = self.sentence_list[0].clone();
        self.start_practice_session(first_sentence);
    }

    /// 문장 연습에서 다음 문장으로 넘어가기
    pub fn next_sentence(&mut self) -> bool {
        self.accumulated_strokes += self.input_automata.get_strokes();
        
        self.current_sentence_idx += 1;
        if self.current_sentence_idx >= self.sentence_list.len() {
            return false;
        }
        
        let next_s = self.sentence_list[self.current_sentence_idx].clone();
        self.input_automata.clear();
        self.target_text = next_s;
        self.update_automata_modes();
        true
    }

    // ════════════════════════════════════════════════════════
    // 게임 모드 공통 헬퍼
    // ════════════════════════════════════════════════════════

    /// 게임 상태 초기화
    pub fn reset_game_state(&mut self) {
        self.game_mode_score = 0;
        self.game_mode_combo = 0;
        self.game_mode_max_combo = 0;
        self.game_mode_lives = 5;
        self.game_mode_round = 0;
        self.game_deadline = None;
        self.game_words_correct = 0;
        self.game_words_total = 0;
        self.flash_visible = false;
        self.flash_timer = None;
        self.flash_duration_ms = 2000;
        self.flash_answer_shown = false;
        self.flash_was_correct = None;
        self.flash_response_times.clear();
        self.rain_words.clear();
        self.rain_last_tick = None;
        self.rain_tick_ms = 150;
        self.rain_spawn_counter = 0;
        self.rain_active_idx = None;
        self.daily_challenge_completed = false;
        self.daily_best_score = None;
        self.long_text_paragraphs.clear();
        self.long_text_current_para_idx = 0;
        self.long_text_title.clear();
        self.long_text_cpm_history.clear();
        self.long_text_last_cpm_sec = 0;
        self.rain_last_speed_round = 0;
        self.start_time = None;
        self.elapsed_time = Duration::default();
        self.accumulated_strokes = 0;
        self.total_errors = 0;
        self.wrong_keys_map.clear();
    }

    /// 게임 모드용 단어 목록 가져오기
    pub fn get_game_words(is_korean: bool, count: usize) -> Vec<String> {
        let words: Vec<&str> = if is_korean {
            vec![
                "하늘", "바다", "나무", "구름", "태양", "바람", "사람", "우주", "도시", "강물",
                "노래", "마음", "봄날", "여름", "가을", "겨울", "행복", "사랑", "미래", "우정",
                "아침", "저녁", "시간", "기억", "평화", "희망", "노력", "열정", "도전", "성공",
                "세계", "지도", "기쁨", "슬픔", "위로", "용기", "감사", "약속", "여행", "예술",
                "소리", "향기", "미소", "눈물", "바위", "모래", "조개", "낙엽", "학교", "친구",
                "공부", "연필", "운동장", "농구", "축구", "야구", "영화", "달빛", "별빛", "파도",
                "갈매기", "고래", "상어", "돌고래", "캠핑", "텐트", "배낭", "토끼", "사슴", "나비",
                "꽃잎", "정원", "공원", "산책", "자전거", "기차", "비행기", "항구", "역사", "그림",
                "음악", "피아노", "기타", "축제", "추억", "소원", "보물", "모험", "자유", "지혜",
                "건강", "웃음", "응원", "승리", "가족", "이웃", "마을", "김치", "비빔밥", "떡볶이",
                "라면", "초밥", "커피", "녹차", "사과", "딸기", "포도", "수박", "바나나", "망고",
            ]
        } else {
            vec![
                "apple", "banana", "cherry", "orange", "grape", "melon", "peach", "lemon", "lime",
                "computer", "keyboard", "monitor", "screen", "network", "system", "database",
                "science", "history", "english", "travel", "nature", "forest", "mountain",
                "summer", "winter", "spring", "weather", "yellow", "purple", "golden",
                "sunshine", "galaxy", "universe", "planet", "ocean", "island", "beach",
                "dolphin", "whale", "campfire", "backpack", "rabbit", "tiger", "butterfly",
                "garden", "bicycle", "train", "airplane", "library", "painting", "music",
                "piano", "guitar", "concert", "festival", "memory", "dream", "treasure",
                "adventure", "miracle", "freedom", "courage", "wisdom", "knowledge",
                "happiness", "fortune", "victory", "gratitude", "family", "teacher", "student",
                "doctor", "artist", "writer", "camera", "phone", "internet", "email", "letter",
            ]
        };
        let mut rng = thread_rng();
        let mut shuffled: Vec<String> = words.iter().map(|&s| s.to_string()).collect();
        shuffled.shuffle(&mut rng);
        shuffled.truncate(count);
        shuffled
    }

    /// 난이도별 게임 단어 가져오기 (서바이벌 모드에서 라운드에 따라 길이 필터)
    pub fn get_game_words_by_difficulty(is_korean: bool, min_len: usize) -> Vec<String> {
        let all_words = Self::get_game_words(is_korean, 200);
        let filtered: Vec<String> = all_words
            .into_iter()
            .filter(|w| w.chars().count() >= min_len)
            .collect();
        if filtered.is_empty() {
            Self::get_game_words(is_korean, 20)
        } else {
            filtered
        }
    }

    // ════════════════════════════════════════════════════════
    // 시간 제한 모드 (Time Attack)
    // ════════════════════════════════════════════════════════

    pub fn setup_time_attack(&mut self, is_korean: bool, time_secs: u64) {
        self.reset_game_state();
        self.game_time_limit_secs = time_secs;
        self.word_list = Self::get_game_words(is_korean, 100);
        self.current_word_idx = 0;
        let first = self.word_list[0].clone();
        self.start_practice_session(first);
        self.game_deadline = Some(Instant::now() + Duration::from_secs(time_secs));
        self.start_time = Some(Instant::now());
    }

    /// 시간 제한 모드에서 시간 초과 여부 확인
    pub fn is_time_attack_expired(&self) -> bool {
        if let Some(deadline) = self.game_deadline {
            Instant::now() >= deadline
        } else {
            false
        }
    }

    /// 시간 제한 모드 남은 시간 (초)
    pub fn time_attack_remaining_secs(&self) -> f64 {
        if let Some(deadline) = self.game_deadline {
            let now = Instant::now();
            if now >= deadline {
                0.0
            } else {
                (deadline - now).as_secs_f64()
            }
        } else {
            0.0
        }
    }

    /// 현재 입력이 목표와 일치하는지 (커밋 후 판정)
    pub fn is_current_word_success(&mut self) -> bool {
        self.input_automata.commit_current();
        crate::hangeul::is_input_exact_match(
            self.input_automata.get_text().trim(),
            self.target_text.trim(),
        )
    }

    /// 시간 제한 모드에서 다음 단어로 넘어가기 (성공 여부에 따라 콤보/점수 분기)
    pub fn time_attack_next_word(&mut self, success: bool) -> bool {
        self.game_words_total += 1;
        if success {
            self.game_mode_combo += 1;
            if self.game_mode_combo > self.game_mode_max_combo {
                self.game_mode_max_combo = self.game_mode_combo;
            }
            // 점수 = 콤보 배수 (최대 5x)
            let multiplier = std::cmp::min(self.game_mode_combo, 5);
            self.game_mode_score += 10 * multiplier;
            self.game_words_correct += 1;
        } else {
            self.game_mode_combo = 0;
        }
        self.accumulated_strokes += self.input_automata.get_strokes();

        self.current_word_idx += 1;
        if self.current_word_idx >= self.word_list.len() {
            // 단어 목록 리필
            let is_korean = match self.active_screen {
                ActiveScreen::TimeAttack { is_korean } => is_korean,
                _ => true,
            };
            let mut more = Self::get_game_words(is_korean, 100);
            self.word_list.append(&mut more);
        }
        let next_w = self.word_list[self.current_word_idx].clone();
        self.input_automata.clear();
        self.target_text = next_w;
        self.update_automata_modes();
        true
    }

    // ════════════════════════════════════════════════════════
    // 서바이벌 모드 (Survival)
    // ════════════════════════════════════════════════════════

    pub fn setup_survival(&mut self, is_korean: bool) {
        self.reset_game_state();
        self.game_mode_lives = 5;
        self.word_list = Self::get_game_words(is_korean, 100);
        self.current_word_idx = 0;
        let first = self.word_list[0].clone();
        self.start_practice_session(first);
        self.start_time = Some(Instant::now());
    }

    /// 서바이벌에서 다음 단어로 (오류 여부에 따라 라이프 감소)
    pub fn survival_next_word(&mut self, had_error: bool) -> bool {
        self.game_words_total += 1;
        if had_error {
            self.game_mode_lives = self.game_mode_lives.saturating_sub(1);
            self.game_mode_combo = 0;
        } else {
            self.game_mode_combo += 1;
            if self.game_mode_combo > self.game_mode_max_combo {
                self.game_mode_max_combo = self.game_mode_combo;
            }
            self.game_mode_score += 10;
            self.game_words_correct += 1;
        }
        self.game_mode_round += 1;
        self.accumulated_strokes += self.input_automata.get_strokes();

        if self.game_mode_lives == 0 {
            return false; // 게임 오버
        }

        self.current_word_idx += 1;
        // 10라운드마다 난이도 상승: 최소 단어 길이 증가
        if self.current_word_idx >= self.word_list.len() {
            let is_korean = match self.active_screen {
                ActiveScreen::Survival { is_korean } => is_korean,
                _ => true,
            };
            // 한글 단어 풀 길이가 짧아 min_len 상한을 둔다
            let min_len = std::cmp::min(4, 2 + (self.game_mode_round / 10));
            let mut more = Self::get_game_words_by_difficulty(is_korean, min_len);
            self.word_list.append(&mut more);
        }
        let next_w = self.word_list[self.current_word_idx].clone();
        self.input_automata.clear();
        self.target_text = next_w;
        self.update_automata_modes();
        true
    }

    // ════════════════════════════════════════════════════════
    // 타자 레인 모드 (Typing Rain)
    // ════════════════════════════════════════════════════════

    pub fn setup_typing_rain(&mut self, is_korean: bool) {
        self.reset_game_state();
        self.game_mode_lives = 5;
        self.word_list = Self::get_game_words(is_korean, 200);
        self.current_word_idx = 0;
        self.rain_last_tick = Some(Instant::now());
        self.rain_tick_ms = 150;
        self.input_automata.clear();
        self.input_automata.english_mode = !is_korean;
        self.start_time = Some(Instant::now());
        // 초기 단어 3개 스폰
        for _ in 0..3 {
            self.spawn_rain_word();
        }
    }

    /// 타자 레인 단어 스폰
    pub fn spawn_rain_word(&mut self) {
        if self.current_word_idx >= self.word_list.len() {
            // 리필
            let is_korean = match self.active_screen {
                ActiveScreen::TypingRain { is_korean } => is_korean,
                _ => true,
            };
            let mut more = Self::get_game_words(is_korean, 200);
            self.word_list.append(&mut more);
        }
        let text = self.word_list[self.current_word_idx].clone();
        self.current_word_idx += 1;
        let mut rng = thread_rng();
        let max_col = self.rain_screen_width.saturating_sub(text.chars().count() as u16 + 2);
        let col = if max_col > 2 { rng.gen_range(1..max_col) } else { 1 };
        self.rain_words.push(RainWord {
            text,
            column: col,
            row: 0.0,
            typed_len: 0,
            active: false,
            destroyed: false,
        });
    }

    /// 타자 레인 틱 (단어를 아래로 이동, 바닥 도달 시 라이프 감소)
    pub fn tick_rain(&mut self) {
        let now = Instant::now();
        let should_tick = if let Some(last) = self.rain_last_tick {
            now.duration_since(last).as_millis() >= self.rain_tick_ms as u128
        } else {
            true
        };

        if !should_tick {
            return;
        }
        self.rain_last_tick = Some(now);

        let bottom = self.rain_screen_height as f32;
        // 이동
        for word in self.rain_words.iter_mut() {
            if !word.destroyed {
                word.row += 0.5;
            }
        }

        // 바닥 도달 체크
        let mut lost_indices = Vec::new();
        for (i, word) in self.rain_words.iter().enumerate() {
            if !word.destroyed && word.row >= bottom {
                lost_indices.push(i);
            }
        }
        for i in lost_indices {
            self.rain_words[i].destroyed = true;
            self.game_mode_lives = self.game_mode_lives.saturating_sub(1);
            self.game_mode_combo = 0;
            self.game_words_total += 1; // 실패 시도도 total에 반영
            // 활성 단어가 바닥에 닿으면 활성 해제
            if self.rain_active_idx == Some(i) {
                self.rain_active_idx = None;
                self.input_automata.clear();
                self.target_text.clear();
            }
        }

        // destroyed 단어 제거 (인덱스 재정렬)
        self.prune_destroyed_rain_words();

        // 스폰 카운터
        self.rain_spawn_counter += 1;
        let spawn_interval = std::cmp::max(3, 8usize.saturating_sub(self.game_mode_round / 5));
        if self.rain_spawn_counter >= spawn_interval {
            self.rain_spawn_counter = 0;
            self.spawn_rain_word();
        }

        // 난이도 상승: 10라운드마다 한 번만 틱 간격 단축 (매 틱 가속 버그 방지)
        if self.game_mode_round > 0
            && self.game_mode_round % 10 == 0
            && self.rain_last_speed_round != self.game_mode_round
        {
            self.rain_last_speed_round = self.game_mode_round;
            self.rain_tick_ms = std::cmp::max(60, self.rain_tick_ms.saturating_sub(10));
        }
    }

    /// 타자 레인: 활성 단어 입력 진행도(완전히 맞은 글자 수) 갱신
    pub fn update_rain_typed_progress(&mut self) {
        if let Some(idx) = self.rain_active_idx {
            let typed = self.input_automata.get_text();
            let target = &self.rain_words[idx].text;
            self.rain_words[idx].typed_len = crate::hangeul::fully_matched_chars(&typed, target);
        }
    }

    /// 타자 레인: 활성 단어가 정확히 완성되었는지
    pub fn is_rain_word_complete(&self) -> bool {
        if let Some(idx) = self.rain_active_idx {
            crate::hangeul::is_input_exact_match(
                &self.input_automata.get_text(),
                &self.rain_words[idx].text,
            )
        } else {
            false
        }
    }

    /// destroyed 레인 단어를 제거하고 active 인덱스를 재계산
    pub fn prune_destroyed_rain_words(&mut self) {
        self.rain_words.retain(|w| !w.destroyed);
        self.rain_active_idx = self.rain_words.iter().position(|w| w.active);
    }

    // ════════════════════════════════════════════════════════
    // 플래시 타이핑 모드 (Flash Typing)
    // ════════════════════════════════════════════════════════

    pub fn setup_flash_typing(&mut self, is_korean: bool) {
        self.reset_game_state();
        self.flash_duration_ms = 2000;
        self.word_list = Self::get_game_words(is_korean, 50);
        self.current_word_idx = 0;
        let first = self.word_list[0].clone();
        self.start_practice_session(first);
        self.flash_visible = true;
        self.flash_timer = Some(Instant::now());
        self.start_time = Some(Instant::now());
    }

    /// 플래시 타이핑에서 노출 시간 경과 후 단어 숨기기 확인
    pub fn check_flash_visibility(&mut self) {
        if self.flash_visible {
            if let Some(timer) = self.flash_timer {
                if timer.elapsed().as_millis() >= self.flash_duration_ms as u128 {
                    self.flash_visible = false;
                }
            }
        }
    }

    /// 플래시 타이핑에서 정답 제출
    pub fn flash_submit_answer(&mut self) {
        self.input_automata.commit_current();
        let typed = self.input_automata.get_text();
        let expected = &self.target_text;
        // 자모 완전 일치 (단순 문자열 비교는 정규화 차이에 취약할 수 있음)
        let correct = crate::hangeul::is_input_exact_match(typed.trim(), expected.trim());
        self.flash_was_correct = Some(correct);
        self.flash_answer_shown = true;
        self.game_words_total += 1;

        if correct {
            self.game_mode_score += 10;
            self.game_words_correct += 1;
            self.game_mode_combo += 1;
            if self.game_mode_combo > self.game_mode_max_combo {
                self.game_mode_max_combo = self.game_mode_combo;
            }
        } else {
            self.game_mode_combo = 0;
        }

        // 응답 시간 기록
        if let Some(timer) = self.flash_timer {
            self.flash_response_times.push(timer.elapsed().as_secs_f64());
        }
    }

    /// 플래시 타이핑 다음 라운드
    pub fn flash_next_round(&mut self) -> bool {
        self.accumulated_strokes += self.input_automata.get_strokes();
        self.game_mode_round += 1;
        self.current_word_idx += 1;

        if self.current_word_idx >= self.word_list.len() {
            return false; // 전체 라운드 종료
        }

        // 난이도 상승: 표시 시간 감소
        self.flash_duration_ms = match self.game_mode_round {
            0..=4 => 2000,
            5..=9 => 1500,
            10..=19 => 1000,
            20..=29 => 700,
            _ => 500,
        };

        let next_w = self.word_list[self.current_word_idx].clone();
        self.input_automata.clear();
        self.target_text = next_w;
        self.update_automata_modes();
        self.flash_visible = true;
        self.flash_timer = Some(Instant::now());
        self.flash_answer_shown = false;
        self.flash_was_correct = None;
        true
    }

    // ════════════════════════════════════════════════════════
    // 데일리 챌린지 모드 (Daily Challenge)
    // ════════════════════════════════════════════════════════

    pub fn setup_daily_challenge(&mut self, is_korean: bool) {
        self.reset_game_state();
        self.word_list = Self::get_daily_seed_words(is_korean);
        self.current_word_idx = 0;
        let first = self.word_list[0].clone();
        self.start_practice_session(first);
        self.daily_best_score = Self::load_daily_best();
        self.start_time = Some(Instant::now());
    }

    /// 날짜 기반 시드로 매일 동일한 10개 단어 생성
    pub fn get_daily_seed_words(is_korean: bool) -> Vec<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut hasher = DefaultHasher::new();
        today.hash(&mut hasher);
        let seed = hasher.finish();

        let words: Vec<&str> = if is_korean {
            vec![
                "하늘", "바다", "나무", "구름", "태양", "바람", "사람", "우주", "도시", "강물",
                "노래", "마음", "봄날", "여름", "가을", "겨울", "행복", "사랑", "미래", "우정",
                "아침", "저녁", "시간", "기억", "평화", "희망", "노력", "열정", "도전", "성공",
                "세계", "기쁨", "용기", "감사", "약속", "여행", "예술", "소리", "향기", "미소",
                "학교", "친구", "공부", "연필", "축구", "야구", "영화", "달빛", "별빛", "파도",
            ]
        } else {
            vec![
                "apple", "banana", "cherry", "ocean", "galaxy", "planet", "mountain", "forest",
                "butterfly", "dolphin", "treasure", "adventure", "freedom", "courage", "wisdom",
                "knowledge", "happiness", "sunshine", "universe", "miracle", "guitar", "painting",
                "library", "concert", "festival", "memory", "garden", "bicycle", "victory",
                "student", "teacher", "doctor", "artist", "camera", "internet", "digital",
                "science", "history", "english", "nature", "summer", "winter", "spring", "weather",
                "golden", "silver", "island", "dream", "letter", "story",
            ]
        };

        let len = words.len();
        let mut selected = Vec::new();
        for i in 0..10 {
            let idx = ((seed.wrapping_add(i as u64).wrapping_mul(6364136223846793005)) % len as u64) as usize;
            selected.push(words[idx].to_string());
        }
        selected
    }

    /// 데일리 챌린지 최고 점수 로드
    pub fn load_daily_best() -> Option<usize> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut path = if let Some(base_dirs) = directories::BaseDirs::new() {
            base_dirs.home_dir().to_path_buf()
        } else {
            std::path::PathBuf::from(".")
        };
        path.push(".typing-practice");
        path.push("daily_best.json");

        if !path.exists() {
            return None;
        }
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
                if data.get("date").and_then(|v| v.as_str()) == Some(&today) {
                    return data.get("score").and_then(|v| v.as_u64()).map(|v| v as usize);
                }
            }
        }
        None
    }

    /// 데일리 챌린지 최고 점수 저장
    pub fn save_daily_best(score: usize) {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut path = if let Some(base_dirs) = directories::BaseDirs::new() {
            base_dirs.home_dir().to_path_buf()
        } else {
            std::path::PathBuf::from(".")
        };
        path.push(".typing-practice");
        let _ = std::fs::create_dir_all(&path);
        path.push("daily_best.json");

        let data = serde_json::json!({
            "date": today,
            "score": score,
        });
        let _ = std::fs::write(&path, serde_json::to_string_pretty(&data).unwrap_or_default());
    }

    /// 데일리 챌린지에서 다음 단어
    pub fn daily_next_word(&mut self) -> bool {
        self.accumulated_strokes += self.input_automata.get_strokes();
        self.game_words_total += 1;

        let typed = self.input_automata.get_text();
        if crate::hangeul::is_input_exact_match(typed.trim(), self.target_text.trim()) {
            self.game_words_correct += 1;
            self.game_mode_combo += 1;
            if self.game_mode_combo > self.game_mode_max_combo {
                self.game_mode_max_combo = self.game_mode_combo;
            }
        } else {
            self.game_mode_combo = 0;
        }

        self.current_word_idx += 1;
        self.game_mode_round += 1;

        if self.current_word_idx >= self.word_list.len() {
            // 데일리 챌린지 완료
            self.stop_timer();
            let cpm = self.get_cpm();
            let acc = self.get_accuracy();
            self.game_mode_score = ((cpm as f64) * (acc / 100.0)) as usize;
            self.daily_challenge_completed = true;

            // 최고 점수 갱신 체크
            let current_best = Self::load_daily_best();
            if current_best.map_or(true, |b| self.game_mode_score > b) {
                Self::save_daily_best(self.game_mode_score);
                self.daily_best_score = Some(self.game_mode_score);
            }
            return false;
        }

        let next_w = self.word_list[self.current_word_idx].clone();
        self.input_automata.clear();
        self.target_text = next_w;
        self.update_automata_modes();
        true
    }

    // ════════════════════════════════════════════════════════
    // 긴 글 레이스 모드 (Long Text Race)
    // ════════════════════════════════════════════════════════

    pub fn get_long_text_titles(is_korean: bool) -> Vec<&'static str> {
        if is_korean {
            vec![
                "1. 애국가 (1~4절)",
                "2. 훈민정음 어제 서문",
                "3. 진달래꽃 (김소월)",
                "4. 대한민국 헌법 제1장",
            ]
        } else {
            vec![
                "1. Gettysburg Address (아브라함 링컨)",
                "2. I Have a Dream (마틴 루터 킹)",
                "3. The Road Not Taken (로버트 프로스트)",
            ]
        }
    }

    pub fn setup_long_text_race(&mut self, is_korean: bool, text_idx: usize) {
        self.reset_game_state();
        self.long_text_selected_idx = text_idx;
        let titles = Self::get_long_text_titles(is_korean);
        self.long_text_title = titles.get(text_idx).copied().unwrap_or("알 수 없는 긴 글").to_string();

        let raw_text = if is_korean {
            match text_idx {
                0 => "\
동해 물과 백두산이 마르고 닳도록 하느님이 보우하사 우리나라 만세.
남산 위에 저 소나무 철갑을 두른 듯 바람 서리 불변함은 우리 기상일세.
가을 하늘 공활한데 높고 구름 없이 밝은 달은 우리 가슴 일편단심일세.
이 기상과 이 맘으로 충성을 다하여 괴로우나 즐거우나 나라 사랑하세.
무궁화 삼천리 화려 강산 대한 사람 대한으로 길이 보전하세.",
                1 => "\
나랏말싸미 듕귁에 달아 문자와로 서르 사맛디 아니할쎄
이런 젼차로 어린 백셩이 니르고져 홇배이셔도
마참내 제 뜨들 시러 펴디 못할 노미 하니라
내 이랄 위하야 어여삐 너겨 새로 스물여덟 자랄 맹가노니
사람마다 해여 수비 니겨 날로 쑤메 편안케 하고져 할 따름이니라",
                2 => "\
나 보기가 역겨워 가실 때에는 말없이 고이 보내 드리우리다.
영변에 약산 진달래꽃 아름 따다 가실 길에 뿌리우리다.
가시는 걸음 걸음 놓인 그 꽃을 사뿐히 즈려밟고 가시옵소서.
나 보기가 역겨워 가실 때에는 죽어도 아니 눈물 흘리우리다.",
                _ => "\
대한민국은 민주공화국이다. 대한민국의 주권은 국민에게 있고, 모든 권력은 국민으로부터 나온다.
대한민국 국민이 되는 요건은 법률로 정한다. 국가는 재외국민을 보호할 의무를 진다.
대한민국의 영토는 한반도와 그 부속도서로 한다. 대한민국은 통일을 지향하며 평화적 통일 정책을 수립하고 추진한다.
대한민국은 국제평화의 유지에 노력하고 침략적 전쟁을 부인한다. 국군은 국가의 안전보장과 국토방위의 의무를 수행한다.",
            }
        } else {
            match text_idx {
                0 => "\
Four score and seven years ago our fathers brought forth on this continent, a new nation, conceived in Liberty, and dedicated to the proposition that all men are created equal.
Now we are engaged in a great civil war, testing whether that nation, or any nation so conceived and so dedicated, can long endure.
We are met on a great battle-field of that war. We have come to dedicate a portion of that field, as a final resting place for those who here gave their lives that that nation might live.",
                1 => "\
I say to you today, my friends, so even though we face the difficulties of today and tomorrow, I still have a dream.
It is a dream deeply rooted in the American dream.
I have a dream that one day this nation will rise up and live out the true meaning of its creed: We hold these truths to be self-evident, that all men are created equal.
I have a dream that my four little children will one day live in a nation where they will not be judged by the color of their skin but by the content of their character.",
                _ => "\
Two roads diverged in a yellow wood, and sorry I could not travel both and be one traveler, long I stood.
And looked down one as far as I could to where it bent in the undergrowth; then took the other, as just as fair.
And having perhaps the better claim, because it was grassy and wanted wear; though as for that the passing there had worn them really about the same.
I shall be telling this with a sigh somewhere ages and ages hence: Two roads diverged in a wood, and I took the one less traveled by, and that has made all the difference.",
            }
        };

        self.long_text_paragraphs = raw_text.lines().map(|s| s.to_string()).collect();
        self.long_text_current_para_idx = 0;
        self.game_words_total = self.long_text_paragraphs.len();
        self.game_words_correct = 0;

        let first_para = self.long_text_paragraphs[0].clone();
        self.start_practice_session(first_para);
        self.start_time = Some(Instant::now());
    }

    /// 긴 글 레이스에서 다음 문단(줄)으로 전환
    pub fn long_text_next_paragraph(&mut self) -> bool {
        self.accumulated_strokes += self.input_automata.get_strokes();
        
        // 현재 라인의 오타 누적
        let typed = self.input_automata.get_text();
        let expected = &self.target_text;
        let exp_len = expected.chars().count();
        let typ_len = typed.chars().count();
        if typ_len < exp_len {
            self.total_errors += exp_len - typ_len;
        }

        // 문단 진행도를 결과 화면 통계에 재사용
        self.game_words_correct = self.long_text_current_para_idx + 1;
        self.game_words_total = self.long_text_paragraphs.len();

        self.long_text_current_para_idx += 1;
        if self.long_text_current_para_idx >= self.long_text_paragraphs.len() {
            // 모든 문단 타이핑 완료
            self.stop_timer();
            let cpm = self.get_cpm();
            let acc = self.get_accuracy();
            // 완주 스코어 계산
            self.game_mode_score = ((cpm as f64) * (acc / 100.0)) as usize;
            self.game_words_correct = self.long_text_paragraphs.len();
            self.game_words_total = self.long_text_paragraphs.len();
            return false;
        }

        let next_para = self.long_text_paragraphs[self.long_text_current_para_idx].clone();
        self.input_automata.clear();
        self.target_text = next_para;
        self.update_automata_modes();
        true
    }

    /// 실시간 CPM 추이 기록용 헬퍼 (경과 초가 바뀔 때 1회만 샘플)
    pub fn record_cpm_history(&mut self) {
        let sec = self.elapsed_time.as_secs();
        if sec == 0 || sec == self.long_text_last_cpm_sec {
            return;
        }
        self.long_text_last_cpm_sec = sec;

        let current_cpm = self.get_cpm();
        if current_cpm > 0 {
            self.long_text_cpm_history.push(current_cpm);
            // 너무 많이 쌓이지 않도록 최근 30개 기록만 보존
            if self.long_text_cpm_history.len() > 30 {
                self.long_text_cpm_history.remove(0);
            }
        }
    }
}

