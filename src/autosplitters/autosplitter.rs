use::asr::Process;

use crate::{Category, Settings};

use super::{any_percent_autosplitter::AnyPercentAutosplitter, placeholder_autosplitter::PlaceholderAutosplitter};

pub trait AutosplittingManager {
    fn is_initialized(&self) -> bool;
    fn is_loading(&self, process: &Process) -> bool;
    fn tick(&mut self, process: &Process, global_settings: &Settings);
    fn init(&mut self, process: &Process) -> bool;
}

pub struct Autosplitter {
}

impl Autosplitter {
    pub fn create_autosplitter(category: Category) -> Box<dyn AutosplittingManager> {
        match category {
            Category::AnyPercent => Box::new(AnyPercentAutosplitter::new()),
            _ => Box::new(PlaceholderAutosplitter::new()),
        }
    }
}
