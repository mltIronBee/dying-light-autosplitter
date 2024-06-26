use asr::{
    deep_pointer::DeepPointer, future::next_tick, settings::{gui::Title, Gui}, signature::Signature, time::Duration, time_util::Instant, timer::{self, TimerState}, watcher::Watcher, Process
};
use quests_manager::QuestsManager;

pub mod quests;
pub mod quests_manager;

asr::async_main!(stable);

#[derive(Gui)]
enum Category {
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

    /// Split settings
    _split_settings: Title,

    /// Awakening
    #[heading_level = 1]
    _awakening: Title,

    /// After speaking with Rahim
    #[default = true]
    awakening_prologue_rahim: bool, // ID 20

    /// After healing mark
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

    /// After talking to Spike
    #[default = true]
    pit_spike: bool, // 525

    #[default = false]
    pit_items: bool, // 527

    /// After reaching tunnel
    #[default = true]
    saviors_reach_tunnels: bool, // 530

    /// After reaching saviors
    #[default = false]
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

    /// After Camden dialogue / Bandits spawn
    #[default = true]
    bandits_spawned: bool, // 813

    /// After talking to Camden
    #[default = true]
    talk_to_camden: bool, // 815

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

const MAINGAME_QUEST_TREE_SIGNATURE: Signature<11> = Signature::new("48 8B 05 ?? ?? ?? ?? 48 8B 0C F0");
const MAINGAME_QUEST_TREE_OFFSET: u64 = 3;
const MOVIE_MANAGER_SIGNATURE: Signature<12> = Signature::new("48 8B 0D ?? ?? ?? ?? 33 ED 48 89 05");
const MOVIE_MANAGER_OFFSET: u64 = 3;
const GAME_DLL: &str = "gamedll_x64_rwdi.dll";
const ENGINE_DLL: &str = "engine_x64_rwdi.dll";
const RD3D11_DLL:&str = "rd3d11_x64_rwdi.dll";
const LOADING_OFFSET: u64 = 0x7E048;
const RESET_DELAY_MS: i64 = 1000;

async fn main() {
    let mut settings = Settings::register();

    loop {
        asr::set_tick_rate(1.0);

        let process = Process::wait_attach("DyingLightGame.exe").await;
        process
            .until_closes(async {
                let mut quest_tree_base_ptr: Option<asr::Address>;
                let mut movie_manager_base_ptr: Option<asr::Address>;

                loop {
                    quest_tree_base_ptr = scan_signature(&process, GAME_DLL, MAINGAME_QUEST_TREE_SIGNATURE, MAINGAME_QUEST_TREE_OFFSET);

                    if quest_tree_base_ptr.is_some() {
                        break;
                    }

                    next_tick().await;
                }

                loop {
                    movie_manager_base_ptr = scan_signature(&process, ENGINE_DLL, MOVIE_MANAGER_SIGNATURE, MOVIE_MANAGER_OFFSET);

                    if movie_manager_base_ptr.is_some() {
                        break;
                    }

                    next_tick().await;
                }
                
                let Some(quest_tree_base_ptr) = quest_tree_base_ptr else {
                    next_tick().await;
                    return;
                };

                let Some(movie_manager_base_ptr) = movie_manager_base_ptr else {
                    next_tick().await;
                    return;
                };

                asr::set_tick_rate(120.0);

                let mut loading = Watcher::<u8>::new();
                let mut main_quest_tree = Watcher::<asr::Address>::new();
                let mut movie_manager = Watcher::<asr::Address>::new();
                let mut quests_manager = QuestsManager::new();
                let mut reset_watcher = Watcher::<u8>::new();
                let mut start_watcher = Watcher::<u8>::new();
                let mut final_qte_watcher = Watcher::<u64>::new();
                let mut reset_time = Instant::now();

                quests_manager.generate_quests(&settings);

                loop {
                    settings.update();

                    if timer::state() == TimerState::NotRunning {
                        quests_manager.reset_quests();
                    }

                    if let Ok(loading_ptr) = process.get_module_address(RD3D11_DLL) {
                        if let Some(pair) = loading.update(process.read::<u8>(loading_ptr + LOADING_OFFSET).ok()) {
                            if pair.changed_to(&240) {
                                timer::pause_game_time();
                            } else if pair.changed_from(&240) {
                                timer::resume_game_time();
                            }
                        }
                    }

                    let Some(main_quest_tree) = main_quest_tree.update(get_maingame_quest_tree_ptr(&process, quest_tree_base_ptr)) else {
                        next_tick().await;

                        continue;
                    };

                    if settings.xtract_final_qte {
                        if let Some(movie_manager) = movie_manager.update(get_movie_manager_ptr(&process, movie_manager_base_ptr)) {
                            if let Some(final_qte_watcher) = final_qte_watcher.update(
                                process.read_pointer_path::<u64>(
                                    movie_manager.current + 0x10,
                                    asr::PointerSize::Bit64,
                                    &[0x0, 0x8, 0x98]
                                ).ok()
                            ) {
                                asr::print_limited::<64>(&format_args!("Final QTE status: {:X}", final_qte_watcher.current));
                                // CMovie object inside CMovieManager has 2 float values for sequence start and sequence end
                                // We can read both of the variables by reading 8 bytes at once and interpretating them as u64 in a hex format
                                if final_qte_watcher.changed_to(&0x43A9C0014352DDDF) {
                                    timer::split();
                                }
                            }
                        }
                    }

                    if settings.auto_reset {
                        let path = match settings.category {
                            // Points to status of the first phase in the game_root quest
                            Category::AnyPercent => vec![0x20, 0x0, 0x40, 0x98, 0x0, 0x6C],
                            // TODO: Add pointer paths to corresponding phases for each category
                            _ => vec![0x20, 0xA0, 0x40, 0x98, 0x0, 0x6C],
                        };

                        if let Some(reset_watcher) = reset_watcher.update(
                            process.read_pointer_path(main_quest_tree.current, asr::PointerSize::Bit64, path.as_slice()).ok()
                        ) {
                            if reset_watcher.changed_to(&1) {
                                reset_time = Instant::now();
                            }

                            if timer::state() != TimerState::NotRunning
                                && !reset_watcher.changed()
                                && reset_watcher.current == 1
                                && reset_time.elapsed() > Duration::milliseconds(RESET_DELAY_MS) {
                                timer::reset();
                                quests_manager.reset_quests();
                            }
                        }
                    }

                    if settings.auto_start {
                        let path = match settings.category {
                            Category::AnyPercent => vec![0x20, 0xA0, 0x40, 0x98, 0x0, 0x6C],
                            Category::NGPlus => vec![0x20, 0x358, 0x40, 0x98, 0x0, 0x6C],
                            _ => vec![],
                        };

                        if let Some(start_watcher) = start_watcher.update(
                            process.read_pointer_path(main_quest_tree.current, asr::PointerSize::Bit64, path.as_slice()).ok()
                        ) {
                            if start_watcher.changed_to(&1) {
                                timer::start();
                            }
                        };
                    }

                    if timer::state() != TimerState::Running {
                        quests_manager.reset_quests();
                    }

                    quests_manager.update_quests(&process, main_quest_tree.current);


                    next_tick().await;
                }
            })
            .await;
    }
}

fn scan_signature<const N: usize>(process: &Process, module_name: &str, signature: Signature<N>, offset: u64) -> Option<asr::Address> {
    let module_addr = process.get_module_address(module_name).ok()?;
    let module_size = process.get_module_size(module_name).ok()?;

    let instruction_addr = signature.scan_process_range(process, (module_addr, module_size))? + offset;

    process.read_pointer(instruction_addr + 0x4 + process.read::<i32>(instruction_addr).ok()?, asr::PointerSize::Bit64).ok()
}

fn get_maingame_quest_tree_ptr(process: &Process, base_ptr: asr::Address) -> Option<asr::Address> {
    DeepPointer::<5>::new_64bit(base_ptr, &[0x0, 0x20, 0x270, 0x8, 0x0]).deref_offsets(process).ok()
}

fn get_movie_manager_ptr(process: &Process, base_ptr: asr::Address) -> Option<asr::Address> {
    DeepPointer::<7>::new_64bit(base_ptr, &[0x28, 0xF8, 0xF0, 0x1F8, 0xC0, 0x88, 0x18]).deref_offsets(process).ok()
}
