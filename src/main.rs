mod app;
mod assets;
mod controller;
mod hangeul;
mod storage;
mod ui;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, terminal::Terminal};
use std::{io, time::Duration};

fn main() -> Result<(), io::Error> {
    // 1. 터미널 환경 설정 (Raw Mode 활성화 및 Alternate Screen 진입)
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. 앱 상태 생성 및 실행
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // 3. 터미널 복원 (오류 발생 시에도 복원)
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        // UI 그리기
        terminal.draw(|f| ui::draw(f, app))?;

        // 100ms 폴링으로 키 이벤트가 없어도 실시간 통계(시간 경과) 업데이트
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if controller::handle_key_press(app, key.code) {
                        return Ok(());
                    }
                }
            }
        }

        // 활성 세션일 때만 실시간 경과 갱신 (종료 화면에서 계속 흐르는 버그 방지)
        if app.is_timer_running() {
            app.update_elapsed_time();
        }

        let size = terminal.size()?;
        controller::handle_runtime_tick(app, size.width, size.height);
    }
}
