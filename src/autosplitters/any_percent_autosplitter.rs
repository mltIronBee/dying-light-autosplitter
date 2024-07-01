use asr::settings::{Gui, gui::Title};
use asr::time::Duration;
use asr::time_util::Instant;
use asr::timer::{self, TimerState};
use asr::{signature::Signature, watcher::Watcher, Process};

use crate::autosplitters::autosplitter::AutosplittingManager;
use crate::{QuestsManager, Settings};

const MAINGAME_QUEST_TREE_SIGNATURE: Signature<11> = Signature::new("48 8B 05 ?? ?? ?? ?? 48 8B 0C F0");
const MAINGAME_QUEST_TREE_OFFSET: u64 = 3;
const MOVIE_MANAGER_SIGNATURE: Signature<14> = Signature::new("48 8B 05 ?? ?? ?? ?? 48 63 CA 48 8B 3C C8");
const MOVIE_MANAGER_OFFSET: u64 = 3;
const GAME_DLL: &str = "gamedll_x64_rwdi.dll";
const RD3D11_DLL:&str = "rd3d11_x64_rwdi.dll";
const LOADING_OFFSET: u64 = 0x7E048;
const RESET_DELAY_MS: i64 = 1000;

#[derive(Gui)]
pub struct AnyPercentSettings {
    /// Any% split settings
    _split_settings: Title,

    /// Awakening
    #[heading_level = 1]
    _awakening: Title,

    /// After speaking with Rahim
    #[default = true]
    awakening_prologue_rahim: bool, // ID 20

    /// After healing Mark
    #[default = true]
    heal_mark: bool, // 39

    /// After Speaking with Rahim after 13th floor
    #[default = true]
    rahim_after_13th_floor: bool, // 40

    /// After finishing gym course
    #[default=false]
    after_gym: bool, // 41

    /// After exiting the Tower
    #[default=true]
    after_exiting_tower: bool, // 83

    /// After taking a shot and speaking with Rahim
    #[default = true]
    after_rahim_dialogue_ends: bool, // 99

    /// First assignment
    #[heading_level = 1]
    _first_assignment: Title,

    /// After exiting Spike's hut
    #[default = true]
    after_exiting_spikes_hut: bool, // 100

    /// After GRE call
    #[default = true]
    fa_after_gre: bool, // 102

    /// After arming all the traps
    #[default = true]
    after_arming_traps: bool, // 103

    /// After sleeping in a safe zone
    #[default = true]
    fa_after_sleeping: bool, // 104

    /// After speaking with GRE
    #[default=true]
    fa_after_gre_call: bool, // 107

    /// After taking Mother's day / Speaking with Brecken
    #[default=true]
    after_mothers: bool, // 118

    /// Airdrop
    #[heading_level=1]
    _airdrop: Title,

    /// After first airdrop
    #[default = true]
    first_airdrop: bool, // 124

    /// After second airdrop
    #[default = true]
    second_airdrop: bool, // 130

    /// After reaching the Tower
    #[default = true]
    airdrp_reach_tower: bool, // 158

    /// After speaking with Brecken
    #[default = true]
    airdrp_brecken: bool, // 159

    /// Pact with Rais
    #[heading_level = 1]
    _pact_with_rais: Title,

    /// After exiting tower / warping to Holy year tunnel
    #[default = false]
    pwr_warp_out: bool, // 162

    /// After entering Rais' garrison
    #[default = true]
    pwr_enter_garrison: bool, // 164

    /// After speaking with Karim
    #[default = true]
    pwr_speak_to_karim: bool, // 165

    /// After first antenna tower
    #[default = true]
    first_antenna: bool, // 193

    /// After second antenna tower
    #[default = true]
    second_antenna: bool, // 194

    /// After reaching Rais' garrison
    #[default = false]
    pwr_reach_garrison: bool, // 195

    /// After speaking with Karim for the 2nd time
    #[default = true]
    pwr_speak_karim_courier: bool, // 197

    /// After Jaffar's Wheelstation
    #[default = true]
    jaffar_wheelstation: bool, // 236

    /// After fishermen village
    #[default = true]
    fishermen_village: bool, // 237

    /// After ferry
    #[default = true]
    ferry: bool, // 266

    /// After envelope
    #[default = true]
    envelope: bool, // 296

    /// After speaking with Rais
    #[default = true]
    pwr_speak_with_rais: bool, // 297

    /// After contacting GRE
    #[default = true]
    pwr_contact_gre: bool, // 428

    /// After cutscene with Jade
    #[default = false]
    pwr_jade_cutscene: bool, // 430

    /// After speaking with Brecken
    #[default = false]
    pwr_brecken: bool, // 432

    /// After Jade call / Taking "Goodnight Mr. Bahir"
    #[default = true]
    pwr_jade_call: bool, // 433

    /// Siblings
    #[heading_level = 1]
    _siblings: Title,

    /// After reaching school
    #[default = true]
    reach_school: bool, // 435

    /// After reaching first crate
    #[default = true]
    sib_first_crate: bool, // 442

    /// After killing bandits
    #[default = false]
    sib_kill_bandits: bool, // 443

    /// After exiting school and skipping cutscene
    #[default = true]
    sib_exit_school: bool, // 444

    /// After speaking with Zere in the Tower
    #[default = true]
    sib_zere_tower: bool, // 451

    /// After reaching bolter spawn
    #[default = true]
    reach_bolter: bool, // 483

    /// After Rahim call
    #[default = false]
    rahim_distress_call: bool, // 485

    /// Reach Trainyard
    #[default = true]
    reach_trainyard: bool, // 499

    /// After escaping volatile nest
    #[default = true]
    escape_nest: bool, // 502

    /// After Rahim's death
    #[default = false]
    rahim_death: bool, // 512

    /// After Brecken and Jade cutscene
    #[default = true]
    sib_brecken_and_jade: bool, // 513

    /// After defeating bandits near trailer
    #[default = true]
    defeat_intruders: bool, // 515

    /// After reaching Rais' garrison
    #[default = false]
    sib_garrison: bool, // 518

    /// After entering the Pit
    #[default = true]
    enter_pit: bool, // 519

    /// The Pit
    #[heading_level = 1]
    _the_pit: Title,

    /// After completing arena fights
    #[default = true]
    arena_fight: bool, // 521

    /// After reaching arena exit
    #[default = true]
    reach_arena_exit: bool, // 523

    /// After passing out
    #[default = false]
    pit_pass_out: bool, // 524

    /// After talking to Quartermaster
    #[default = true]
    pit_quartermaster: bool, // 525

    #[default = false]
    pit_items: bool, // 527

    /// After reaching tunnel
    #[default = false]
    saviors_reach_tunnels: bool, // 530

    /// After reaching saviors
    #[default = true]
    reach_saviors: bool, // 534

    /// After talking to saviors guide
    #[default = true]
    talk_to_guide: bool, // 535

    /// After escaping ambush in sewers
    #[default = true]
    escape_ambush: bool, // 539

    /// After reaching sewers exit
    #[default = false]
    saviors_reach_exit: bool, // 545

    /// After entering the Old Town
    #[default = true]
    enter_old_town: bool, // 570

    /// Find the Embers
    #[heading_level = 1]
    _find_the_embers: Title,

    /// After finding Troy
    #[default = true]
    fte_reach_troy: bool, // 634

    /// After entering the University
    #[default = false]
    enter_university: bool, // 684

    /// After talking to Fidan
    #[default = true]
    he_talk_to_fidan: bool, // 694

    /// After getting back to Troy
    #[default = true]
    he_speak_to_troy: bool, // 729

    /// Public face
    #[heading_level = 1]
    _public_face: Title,

    /// Reach sewers entrance
    #[default = true]
    pf_reach_sewers: bool, // 730

    /// After picking up explosives
    #[default = true]
    pickup_explosives: bool, // 736

    /// After reaching sewers exit
    #[default = false]
    pf_reach_sewers_exit: bool, // 758

    /// After detonation cutscene
    #[default = true]
    pf_exit_cutscene: bool, // 759

    /// Rendezvous
    #[heading_level = 1]
    _rendezvous: Title,

    /// After rendezvous with Jade
    #[default = true]
    rendezvous_with_jade: bool, // 761

    /// The Museum
    #[heading_level = 1]
    _the_museum: Title,

    /// After meetign Tariq
    #[default = true]
    meet_tariq: bool, // 762

    /// After reachin the Museum
    #[default = true]
    reach_museum: bool, // 763

    /// After defending Jade from zombies
    #[default = true]
    defend_jade: bool, // 765

    /// After navigating through maze
    #[default = true]
    dream_maze: bool, // 782

    /// After dream end
    #[default = true]
    dream_end: bool, // 783

    /// After exiting museum
    #[default = true]
    exit_museum: bool, // 784

    /// After speaking with Troy
    #[default = true]
    tm_speak_to_troy: bool, // 786

    /// Broadcast
    #[heading_level = 1]
    _broadcast: Title,

    /// After reaching start of broadcast quest (Broadcast becomes warpable)
    #[default = false]
    reach_broadcast_sewers: bool, // 787

    /// After exiting the sewers
    #[default = true]
    brdcst_exit_sewers: bool, // 800

    /// After activating broadcast antenna
    #[default = true]
    brdcst_finished: bool, // 801

    /// After re-entering the sewers
    #[default = true]
    brdcst_re_enter: bool, // 811

    /// The Clinic
    #[heading_level = 1]
    _the_clinic: Title,

    /// After Entering the Clinic
    #[default = true]
    tc_enter_clinic: bool, // 815

    /// After talking to Camden
    #[default = true]
    talk_to_camden: bool, // 825

    /// After GRE (Rais)
    #[default = true]
    tc_gre_call: bool, // 826

    // Extraction
    #[heading_level = 1]
    _extraction: Title,

    /// After reaching Rais' tower
    #[default = true]
    reach_finale: bool, // 828

    /// After dropping down to the sewers
    #[default = true]
    xtract_sewers: bool, // 829

    /// After escaping volatiles chase
    #[default = false]
    xtract_exit_sewers: bool, // 830

    /// After entering arena
    #[default = true]
    xtract_enter_arena: bool, // 831

    /// After jumping and climbing up from destroyed crane
    #[default = true]
    xtract_after_crane: bool, // 833

    /// After final QTE
    #[default = true]
    xtract_final_qte: bool, // Movie manager hook

    /// Start of credits
    #[default = false]
    xtract_roll_credits: bool // 834
}

pub struct AnyPercentAutosplitter {
    is_initialized: bool,
    quest_tree_base_ptr: asr::Address,
    movie_manager_base_ptr: asr::Address,
    settings: AnyPercentSettings,
    main_quest_tree: Watcher<asr::Address>,
    movie_manager: Watcher::<asr::Address>,
    quests_manager: QuestsManager,
    reset_watcher: Watcher::<u8>,
    start_watcher: Watcher::<u8>,
    final_qte_watcher: Watcher::<u64>,
    reset_time: Instant,
}

impl AnyPercentAutosplitter {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            quest_tree_base_ptr: asr::Address::new(0x0),
            movie_manager_base_ptr: asr::Address::new(0x0),
            settings: AnyPercentSettings::register(),
            main_quest_tree: Watcher::new(),
            movie_manager: Watcher::new(),
            quests_manager: QuestsManager::new(),
            reset_watcher: Watcher::new(),
            start_watcher: Watcher::new(),
            final_qte_watcher: Watcher::new(),
            reset_time: Instant::now(),
        }
    }

    fn get_maingame_quest_tree_ptr(&self, process: &Process, base_ptr: asr::Address) -> Option<asr::Address> {
        let addr = process.read_pointer_path::<u64>(base_ptr, asr::PointerSize::Bit64, &[0x0, 0x20, 0x270, 0x8]).ok()?;

        Some(asr::Address::new(addr))
    }

    fn get_movie_manager_ptr(&self, process: &Process, base_ptr: asr::Address) -> Option<asr::Address> {
        let addr = process.read_pointer_path::<u64>(base_ptr, asr::PointerSize::Bit64, &[0x0, 0x8, 0x78, 0x0, 0x8, 0x2C0]).ok()?;

        Some(asr::Address::new(addr + 0x18))
    }

    fn scan_signature<const N: usize>(&self, process: &Process, module_name: &str, signature: Signature<N>, offset: u64) -> Option<asr::Address> {
        let module_addr = process.get_module_address(module_name).ok()?;
        let module_size = process.get_module_size(module_name).ok()?;
    
        let instruction_addr = signature.scan_process_range(process, (module_addr, module_size))? + offset;
    
        process.read_pointer(instruction_addr + 0x4 + process.read::<i32>(instruction_addr).ok()?, asr::PointerSize::Bit64).ok()
    }

    fn generate_quest_ids(&self) -> Vec<u16> {
        let mut quest_ids: Vec<u16> = vec![];

        macro_rules! map_settings {
            ($($property:ident, $id:expr),*) => {
                $(if self.settings.$property {
                    quest_ids.push($id)
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
            airdrp_reach_tower, 158,
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
            pit_quartermaster, 525,
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
            pf_exit_cutscene, 759,
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
            tc_enter_clinic, 815,
            talk_to_camden, 825,
            tc_gre_call, 826,
            reach_finale, 828,
            xtract_sewers, 829,
            xtract_exit_sewers, 830,
            xtract_enter_arena, 831,
            xtract_after_crane, 833,
            xtract_roll_credits, 834
        );

        quest_ids
    }
}

impl Default for AnyPercentAutosplitter {
    fn default() -> Self {
        Self {
            is_initialized: false,
            quest_tree_base_ptr: asr::Address::new(0x0),
            movie_manager_base_ptr: asr::Address::new(0x0),
            settings: AnyPercentSettings::register(),
            main_quest_tree: Watcher::new(),
            movie_manager: Watcher::new(),
            quests_manager: QuestsManager::new(),
            reset_watcher: Watcher::new(),
            start_watcher: Watcher::new(),
            final_qte_watcher: Watcher::new(),
            reset_time: Instant::now(),
        }
    }
}

impl AutosplittingManager for AnyPercentAutosplitter {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    fn is_loading(&self, process: &Process) -> bool {
        if let Ok(loading_ptr) = process.get_module_address(RD3D11_DLL) {
            if let Ok(loading) = process.read::<u8>(loading_ptr + LOADING_OFFSET) {
                return loading == 240
            }
        }

        false
    }

    fn init(&mut self, process: &Process) -> bool {
        let Some(quest_tree_base_ptr) = self.scan_signature(process, GAME_DLL, MAINGAME_QUEST_TREE_SIGNATURE, MAINGAME_QUEST_TREE_OFFSET) else {
            return false;
        };

        self.quest_tree_base_ptr = quest_tree_base_ptr;

        let Some(main_quest_tree_ptr) = self.get_maingame_quest_tree_ptr(process, self.quest_tree_base_ptr) else {
            return false;
        };

        if self.main_quest_tree.update(Some(main_quest_tree_ptr)).is_none() {
            return false;
        };

        let Some(movie_manager_base_ptr) = self.scan_signature(process, GAME_DLL, MOVIE_MANAGER_SIGNATURE, MOVIE_MANAGER_OFFSET) else {
            return false;
        };
 
        self.movie_manager_base_ptr = movie_manager_base_ptr;

        if self.movie_manager.update(self.get_movie_manager_ptr(process, movie_manager_base_ptr)).is_none() {
            return false;
        }

        self.quests_manager.generate_quests(&self.generate_quest_ids());

        self.is_initialized = true;

        true
    }

    fn tick(&mut self, process: &Process, global_settings: &Settings) {
        self.settings.update();

        let watched_quests = self.generate_quest_ids();

        self.quests_manager.regenerate_quests(&watched_quests);

        if timer::state() == TimerState::NotRunning {
            self.quests_manager.reset_quests();
        }

        let movie_manager_ptr = self.get_movie_manager_ptr(process, self.movie_manager_base_ptr);
        let Some(main_quest_tree) = self.main_quest_tree.update(self.get_maingame_quest_tree_ptr(process, self.quest_tree_base_ptr)) else {
            return;
        };

        if self.settings.xtract_final_qte {
            if let Some(movie_manager) = self.movie_manager.update(movie_manager_ptr) {
                if let Some(final_qte_watcher) = self.final_qte_watcher.update(
                    process.read_pointer_path::<u64>(
                        movie_manager.current + 0x10,
                        asr::PointerSize::Bit64,
                        &[0x0, 0x8, 0x98]
                    ).ok()
                ) {
                    // CMovie object inside CMovieManager has 2 float values for sequence start and sequence end
                    // We can read both of the variables by reading 8 bytes at once and interpretating them as u64 in a hex format
                    if final_qte_watcher.changed_to(&0x43A9C0014352DDDF) {
                        asr::print_message("Rais is dead");
                        timer::split();
                    }
                }
            }

        
        }

        if global_settings.auto_reset {
            if let Some(reset_watcher) = self.reset_watcher.update(
                process.read_pointer_path(main_quest_tree.current, asr::PointerSize::Bit64, &[0x20, 0x0, 0x40, 0x98, 0x0, 0x6C]).ok()
            ) {
                // When save warping, game loads all of the quests one by one, flipping the status flag to 1 then to 2
                // creating false positive scenario for a reset. If reset status flag stays 1 for continous amount of time
                // we can be sure, that the run is actually being reset
                if reset_watcher.changed_to(&1) {
                    self.reset_time = Instant::now();
                }

                if timer::state() != TimerState::NotRunning
                    && !reset_watcher.changed()
                    && reset_watcher.current == 1
                    && self.reset_time.elapsed() > Duration::milliseconds(RESET_DELAY_MS) {
                    timer::reset();
                    self.quests_manager.reset_quests();
                }
            }
        }

        if global_settings.auto_start {
            if let Some(start_watcher) = self.start_watcher.update(
                process.read_pointer_path(main_quest_tree.current, asr::PointerSize::Bit64, &[0x20, 0xA0, 0x40, 0x98, 0x0, 0x6C]).ok()
            ) {
                if start_watcher.changed_to(&1) {
                    timer::start();
                }
            };
        }

        if timer::state() != TimerState::Running {
            self.quests_manager.reset_quests();
        }

        if timer::state() == TimerState::Running {
            self.quests_manager.update_quests(process, main_quest_tree.current);
        }
    }
}
