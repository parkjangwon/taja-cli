use super::App;
use rand::seq::SliceRandom;
use rand::thread_rng;

impl App {
    // --- 자리 연습 데이터 정의 ---
    pub fn get_finger_practice_target(level: usize, is_korean: bool) -> String {
        let words = crate::assets::finger_practice_words(level, is_korean).to_vec();
        
        let mut rng = thread_rng();
        let mut selected = words.clone();
        selected.shuffle(&mut rng);
        selected.join(" ")
    }

    // --- 낱말 연습용 단어 세트 로드 ---
    pub fn setup_word_practice(&mut self, is_korean: bool) {
        let words: Vec<String> = if is_korean {
            crate::assets::word_practice_words(true).iter().map(|&s| s.to_string()).collect()
        } else {
            let mut list: Vec<String> = crate::assets::word_practice_words(false).iter().map(|&s| s.to_string()).collect();
            
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
        let sentences = crate::assets::sentence_practice_sentences(is_korean);
        
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
}
