mod engine;
mod app;

use std::time::Duration;
use android_activity::{
    AndroidApp, InputStatus, MainEvent, PollEvent,
    input::{InputEvent, MotionAction}
};

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let mut eng = engine::Engine::new();
    let mut state = app::AppState::new();

    loop {
        app.poll_events(Some(Duration::from_millis(16)), |event| {
            if let PollEvent::Main(main_event) = event {
                match main_event {
                    MainEvent::InitWindow { .. } => {
                        if let Some(win) = app.native_window() {
                            eng.init_surface(&win);
                            state.init();
                        }
                    }
                    MainEvent::TerminateWindow { .. } => {
                        eng.destroy_surface();
                    }
                    MainEvent::WindowResized { .. } => {
                        if let Some(win) = app.native_window() {
                            eng.init_surface(&win);
                        }
                    }
                    MainEvent::Destroy => return,
                    _ => {}
                }
            }
        });

        if let Ok(mut iter) = app.input_events_iter() {
            while iter.next(|event| {
                if let InputEvent::MotionEvent(motion) = event {
                    let action = motion.action();
                    for pointer in motion.pointers() {
                        let act_code = match action {
                            MotionAction::Down => 0,
                            MotionAction::Move => 2,
                            MotionAction::Up => 1,
                            _ => 3,
                        };
                        state.on_touch(act_code, pointer.x(), pointer.y(), eng.width, eng.height);
                    }
                    InputStatus::Handled
                } else {
                    InputStatus::Unhandled
                }
            }) {}
        }

        if eng.has_surface() {
            state.render(eng.width, eng.height);
            eng.swap_buffers();
        }
    }
}
