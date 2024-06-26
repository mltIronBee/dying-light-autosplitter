use asr::{Address, Process};

use crate::{quests::{Quest, QuestStatus}, Settings};

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

    pub fn generate_quests(&mut self, settings: &Settings) -> &Vec<Quest> {
        macro_rules! map_settings {
            ($($property:ident, $id:expr),*) => {
                $(if settings.$property {
                    self.quests.push(Quest::new($id, Some(QuestStatus::NotStarted)))
                })*
            };
        }

        map_settings!(
            awakening_prologue_rahim, 20,
            heal_mark, 39,
            rahim_after_13th_floor, 40,
            after_gym, 41,
            after_exiting_tower, 83,
            after_rahim_dialogue_ends, 99,
            after_exiting_spikes_hut, 100,
            fa_after_gre, 102,
            after_arming_traps, 103,
            fa_after_sleeping, 104,
            fa_after_gre_call, 107,
            after_mothers, 118,
            first_airdrop, 124,
            second_airdrop, 130,
            airdrp_brecken, 159,
            pwr_warp_out, 162,
            pwr_enter_garrison, 164,
            pwr_speak_to_karim, 165,
            first_antenna, 193,
            second_antenna, 194,
            pwr_reach_garrison, 195,
            pwr_speak_karim_courier, 236,
            fishermen_village, 237,
            ferry, 266,
            envelope, 296,
            pwr_speak_with_rais, 297,
            pwr_contact_gre, 428,
            pwr_jade_cutscene, 430,
            pwr_brecken, 432,
            pwr_jade_call, 433,
            reach_school, 435,
            sib_first_crate, 442,
            sib_kill_bandits, 443,
            sib_exit_school, 444,
            sib_zere_tower, 451,
            reach_bolter, 483,
            rahim_distress_call, 485,
            reach_trainyard, 499,
            escape_nest, 502,
            rahim_death, 512,
            sib_brecken_and_jade, 513,
            defeat_intruders, 515,
            sib_garrison, 518,
            enter_pit, 519,
            arena_fight, 521,
            reach_arena_exit, 523,
            pit_pass_out, 524,
            pit_spike, 525,
            pit_items, 527,
            saviors_reach_tunnels, 530,
            reach_saviors, 534,
            talk_to_guide, 535,
            escape_ambush, 539,
            saviors_reach_exit, 545,
            enter_old_town, 570,
            fte_reach_troy, 634,
            enter_university, 684,
            he_talk_to_fidan, 694,
            he_speak_to_troy, 729,
            pf_reach_sewers, 730,
            pickup_explosives, 736,
            pf_reach_sewers_exit, 758,
            pf_exit_cutscene, 750,
            rendezvous_with_jade, 761,
            meet_tariq, 762,
            reach_museum, 763,
            defend_jade, 765,
            dream_maze, 782,
            dream_end, 783,
            exit_museum, 784,
            tm_speak_to_troy, 786,
            reach_broadcast_sewers, 787,
            brdcst_exit_sewers, 800,
            brdcst_finished, 801,
            brdcst_re_enter, 811,
            bandits_spawned, 813,
            talk_to_camden, 815,
            tc_gre_call, 826,
            reach_finale, 828,
            xtract_sewers, 829,
            xtract_exit_sewers, 830,
            xtract_enter_arena, 831,
            xtract_after_crane, 833,
            xtract_roll_credits, 834
        );

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


