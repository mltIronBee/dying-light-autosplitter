use std::cmp::max;

use asr::{watcher::{Pair, Watcher}, Address, Process};

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum QuestStatus {
    NotStarted,
    Active,
    Finished,
}

#[derive(Clone)]
pub struct Quest {
    pub quest_id: u16,
    pub quest_status: Watcher<QuestStatus>,
}

impl Quest
{
    pub fn new(quest_id: u16, status: Option<QuestStatus>,) -> Self {
        let mut quest_status = Watcher::<QuestStatus>::new();

        quest_status.update(status);

        Quest {
            quest_id,
            quest_status,
        }
    }

    pub fn reset(&mut self) {
        self.quest_status.update(Some(QuestStatus::NotStarted));
    }

    pub fn update_status(&mut self, process: &Process, quest_tree_ptr: Address) -> Option<&Pair<QuestStatus>> {
        let current_status = self.quest_status.clone().pair?.current;

        if current_status == QuestStatus::Finished {
            return None;
        }

        let Ok(quest_id) = process.read_pointer_path::<u16>(
            quest_tree_ptr,
            asr::PointerSize::Bit64,
            &[0x20, (0x8 * self.quest_id).into(), 0x10],
        ) else {
            return self.quest_status.pair.as_ref();
        };

        if quest_id != self.quest_id {
            return self.quest_status.pair.as_ref();
        }

        let status = match process.read_pointer_path::<u8>(
            quest_tree_ptr,
            asr::PointerSize::Bit64,
            &[0x20, (0x8 * self.quest_id).into(), 0x3C]
        ).ok()? {
            0 => QuestStatus::NotStarted,
            1 => QuestStatus::Active,
            2 => QuestStatus::Finished,
            _ => QuestStatus::NotStarted
        };

        let pair = self.quest_status.update(Some(max(status, current_status)));

        pair
    }
}
