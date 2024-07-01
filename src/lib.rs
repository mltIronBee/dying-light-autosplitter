use asr::{
    future::next_tick, settings::{gui::Title, Gui}, timer::{self, TimerState}, watcher::Watcher, Process
};
use autosplitters::autosplitter::Autosplitter;
use quests_manager::QuestsManager;

pub mod autosplitters;
pub mod quests;
pub mod quests_manager;

asr::async_main!(stable);

#[derive(Gui)]
pub enum Category {
    /// Any%
    #[default]
    AnyPercent,

    /// NG+
    NGPlus,

    /// The Following (not supported)
    TheFollowing,
}

#[derive(Gui)]
pub struct Settings {
    /// General settings
    _general_settings: Title,

    /// Run category
    category: Category,

    /// Auto start
    #[default = true]
    auto_start: bool,

    /// Auto reset
    #[default = true]
    auto_reset: bool,
}

async fn main() {
    let mut settings = Settings::register();
    let mut any_percent_autosplitter = Autosplitter::create_autosplitter(Category::AnyPercent);
    let mut ngplus_autosplitter = Autosplitter::create_autosplitter(Category::NGPlus);
    let mut tf_autosplitter = Autosplitter::create_autosplitter(Category::TheFollowing);

    loop {
        let process = Process::wait_attach("DyingLightGame.exe").await;
        process.until_closes(async {
            loop {
                let autosplitters = [
                    &mut any_percent_autosplitter,
                    &mut ngplus_autosplitter,
                    &mut tf_autosplitter,
                ];
                let mut init_count: usize = 0;
                let autosplitters_len = autosplitters.len();

                for autosplitter in autosplitters {
                    if !autosplitter.is_initialized() && autosplitter.init(&process) || autosplitter.is_initialized() {
                        init_count += 1;
                    }
                }

                if init_count == autosplitters_len {
                    break;
                }
            }

            let mut loading = Watcher::<bool>::new();
            
            loop {
                settings.update();

                let autosplitter = match settings.category {
                    Category::AnyPercent => &mut any_percent_autosplitter,
                    Category::NGPlus => &mut ngplus_autosplitter,
                    Category::TheFollowing => &mut tf_autosplitter,
                };

                if timer::state() == TimerState::Running {
                    if let Some(loading) = loading.update(Some(autosplitter.is_loading(&process))) {
                        if loading.changed_to(&true) {
                            timer::pause_game_time();
                        } else if loading.changed_to(&false) {
                            timer::resume_game_time();
                        }
                    }
                }

                autosplitter.tick(&process, &settings);

                next_tick().await;
            }
        }).await;
    }
}

