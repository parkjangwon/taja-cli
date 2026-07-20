use super::App;
use std::time::Instant;

impl App {
    /// 타이머 시작 (누적 elapsed_time 유지하며 재개)
    pub fn ensure_timer_started(&mut self) {
        if self.start_time.is_none() {
            let now = Instant::now();
            // 이미 누적된 elapsed_time이 있으면 그만큼 과거 시점에서 시작
            self.start_time = Some(now - self.elapsed_time);
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

    /// 타이머 일시정지: 누적 시간을 보존하고 start_time을 제거해 진행을 멈춤
    pub fn pause_timer(&mut self) {
        self.update_elapsed_time();
        self.start_time = None;
    }

    /// 타이머 정지: 최종 경과를 고정하고 start_time을 제거해 이후 갱신을 막는다.
    /// 세션/게임 종료·중단 시 반드시 호출한다.
    pub fn stop_timer(&mut self) {
        if let Some(start) = self.start_time.take() {
            self.elapsed_time = start.elapsed();
        }
    }
}
