use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use directories::BaseDirs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PracticeRecord {
    pub date: String,
    pub mode: String,
    pub language: String,
    pub cpm: usize,
    pub accuracy: f64,
    pub duration_secs: u64,
    pub wrong_keys: HashMap<char, usize>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct History {
    pub records: Vec<PracticeRecord>,
}

pub struct StorageManager {
    file_path: PathBuf,
}

impl StorageManager {
    pub fn new() -> Self {
        let mut path = if let Some(base_dirs) = BaseDirs::new() {
            base_dirs.home_dir().to_path_buf()
        } else {
            PathBuf::from(".")
        };
        
        path.push(".typing-practice");
        
        // 디렉토리가 없으면 생성
        if !path.exists() {
            let _ = fs::create_dir_all(&path);
        }
        
        path.push("history.json");
        
        Self { file_path: path }
    }

    /// 전체 기록 로드
    pub fn load_history(&self) -> History {
        if !self.file_path.exists() {
            return History::default();
        }
        
        let data = match fs::read_to_string(&self.file_path) {
            Ok(content) => content,
            Err(_) => return History::default(),
        };
        
        serde_json::from_str(&data).unwrap_or_default()
    }

    /// 새로운 연습 기록 추가 및 저장
    pub fn add_record(&self, record: PracticeRecord) -> Result<(), std::io::Error> {
        let mut history = self.load_history();
        history.records.push(record);
        
        let serialized = serde_json::to_string_pretty(&history)?;
        fs::write(&self.file_path, serialized)?;
        
        Ok(())
    }

    /// 자주 틀리는 키 상위 N개 분석
    pub fn get_frequent_errors(&self, limit: usize) -> Vec<(char, usize)> {
        let history = self.load_history();
        let mut errors: HashMap<char, usize> = HashMap::new();
        
        for record in history.records {
            for (c, count) in record.wrong_keys {
                *errors.entry(c).or_insert(0) += count;
            }
        }
        
        let mut error_list: Vec<(char, usize)> = errors.into_iter().collect();
        error_list.sort_by(|a, b| b.1.cmp(&a.1)); // 내림차순 정렬
        
        error_list.truncate(limit);
        error_list
    }
}
