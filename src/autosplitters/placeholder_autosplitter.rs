use asr::Process;

use crate::{autosplitters::autosplitter::AutosplittingManager, Settings};

#[derive(Default)]
pub struct PlaceholderAutosplitter {}

impl PlaceholderAutosplitter {
    pub fn new() -> Self {
        Self {}
    }
}

impl AutosplittingManager for PlaceholderAutosplitter {
    fn is_initialized(&self) -> bool {
        true
    }
    fn is_loading(&self, _: &Process) -> bool {
        false
    }

    fn tick(&mut self, _process: &Process, _global_settings: &Settings) {}
    fn init(&mut self, _process: &Process) -> bool { true }
}