use asr::{Address, Process};

use crate::quests::{Quest, QuestStatus};

#[derive(Default)]
pub struct QuestsManager {
    quests: Vec<Quest>,
}

impl QuestsManager {
    pub fn new() -> Self {
        Self {
            quests: vec![],
        }
    }

    pub fn generate_quests(&mut self, quest_ids: &[u16]) -> &Vec<Quest> {
        self.quests.resize(0, Quest::new(0, Some(QuestStatus::NotStarted)));

        for quest_id in quest_ids {
            self.quests.push(Quest::new(*quest_id, Some(QuestStatus::NotStarted)));
        }

        &self.quests
    }

    pub fn regenerate_quests(&mut self, quest_ids: &[u16]) -> &Vec<Quest> {
        if self.quests.len() != quest_ids.len() {
            asr::print_message("Quest Manager: Quest settings changed. Regenerating quests list");
            return self.generate_quests(quest_ids);
        }

        for i in 0..self.quests.len() {
            if self.quests[i].quest_id != quest_ids[i] {
                asr::print_message("Quest Manager: Quest settings changed. Regenerating quests list");
                return self.generate_quests(quest_ids);
            }
        }

        &self.quests
    }

    pub fn reset_quests(&mut self) {
        for quest in self.quests.iter_mut() {
            quest.reset();
        }
    }

    pub fn update_quests(&mut self, process: &Process, quest_tree_ptr: Address) {
        for quest in self.quests.iter_mut() {
            if let Some(status) = quest.update_status(process, quest_tree_ptr) {
                if status.changed() &&status.changed_from_to(&QuestStatus::Active, &QuestStatus::Finished) {
                    asr::print_limited::<32>(&format_args!("Quest {} is finished", quest.quest_id));
                    asr::timer::split();
                }
            }
        }
    }
}


