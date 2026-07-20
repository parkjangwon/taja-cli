use super::{ActiveScreen, App};
use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{Duration, Instant};

impl App {
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
        let words = crate::assets::game_words(is_korean);
        let mut rng = thread_rng();
        let mut shuffled: Vec<String> = words.iter().map(|s| (*s).to_string()).collect();
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
        self.rain_words.push(super::RainWord {
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

        let words = crate::assets::daily_seed_words(is_korean);

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
        crate::assets::long_text_titles(is_korean).to_vec()
    }

    pub fn setup_long_text_race(&mut self, is_korean: bool, text_idx: usize) {
        self.reset_game_state();
        self.long_text_selected_idx = text_idx;
        let titles = Self::get_long_text_titles(is_korean);
        self.long_text_title = titles.get(text_idx).copied().unwrap_or("알 수 없는 긴 글").to_string();

        let raw_text = crate::assets::long_text_body(is_korean, text_idx);

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
