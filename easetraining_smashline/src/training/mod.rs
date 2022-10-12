use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::*;
use smash::lib::L2CValue;
use smash::lib::L2CAgent;
use smash::hash40;
use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::app::sv_battle_object::entry_id;
use skyline::hooks::{getRegionAddress, Region};

static mut PAISY : [i32; 8] = [0; 8];
static mut DIDDY : [i32; 8] = [0; 8];
static mut SAMUSCHARGE : [i32; 8] = [-1; 8];
static mut PAC : [i32; 8] = [-1; 8];
static mut SEPH : [i32; 8] = [0; 8];
static mut TINK : [i32; 8] = [0; 8];
static mut LINK : [i32; 8] = [0; 8];
static mut RESTART : bool = false;
static mut RESET_PLACE : i32 = 0;
static mut SELFDAMAGE : [f32; 8] = [0.0; 8.0 as usize];
static mut ARSENE : [i32; 8] = [0; 8];
static mut FLOAT_OFFSET : usize = 0x4e40a0;
static mut INT_OFFSET : usize = 0x4e4060;
static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];
static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];
macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

#[skyline::hook(offset = FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
		let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
		let ret = original!()(module_accessor, param_type, param_hash);
		let fighter_kind = get_kind(boma);
		if smash::app::smashball::is_training_mode() == true {
			//Wario
			if param_hash == hash40("gass_middle_time") && fighter_kind == *FIGHTER_KIND_WARIO  {
				return 1.0;
			}
			if param_hash == hash40("gass_large_time") && fighter_kind == *FIGHTER_KIND_WARIO {
				return 1.0;
			}
			if param_hash == hash40("gass_max_time") && fighter_kind == *FIGHTER_KIND_WARIO {
				return 3.0;
			}
			//Sephiroth
			if param_hash == hash40("activate_point_init") && fighter_kind == *FIGHTER_KIND_EDGE {
				return 40.0;
			}
			if param_hash == hash40("stock_activate_point") && fighter_kind == *FIGHTER_KIND_EDGE {
				return 0.0;
			}
			if param_hash == hash40("deactivate_point_kill") && fighter_kind == *FIGHTER_KIND_EDGE {
				return 0.0;
			}
			//Inkling
			if param_hash == hash40("erosion_rate") && fighter_kind == *FIGHTER_KIND_INKLING  {
				return 0.0;
			}
			if param_hash == hash40("sub_ink_attack_100") && fighter_kind == *FIGHTER_KIND_INKLING {
				return 0.0;
			}
			if param_hash == hash40("sub_ink_attack_s4") && fighter_kind == *FIGHTER_KIND_INKLING {
				return 0.0;
			}
			if param_hash == hash40("sub_ink_attack_hi4") && fighter_kind == *FIGHTER_KIND_INKLING {
				return 0.0;
			}
			if param_hash == hash40("sub_ink_attack_lw4") && fighter_kind == *FIGHTER_KIND_INKLING {
				return 0.0;
			}
			if param_hash == hash40("sub_ink_throw_f") && fighter_kind == *FIGHTER_KIND_INKLING {
				return 0.0;
			}
			if param_hash == 0x1140fae8a9 && fighter_kind == *FIGHTER_KIND_INKLING {
				return 0.0;
			}
			if param_hash == hash40("sub_ink_special_lw") && fighter_kind == *FIGHTER_KIND_INKLING {
				return 0.0;
			}
			if param_hash == hash40("sub_ink_special_s") && fighter_kind == *FIGHTER_KIND_INKLING  {
				return 0.0;
			}
			//Shulk
			if param_hash == hash40("active_time_jump") && fighter_kind == *FIGHTER_KIND_SHULK  {
				return 99999.0;
			}
			if param_hash == hash40("active_time_speed") && fighter_kind == *FIGHTER_KIND_SHULK  {
				return 99999.0;
			}
			if param_hash == hash40("active_time_shield") && fighter_kind == *FIGHTER_KIND_SHULK  {
				return 99999.0;
			}
			if param_hash == hash40("active_time_buster") && fighter_kind == *FIGHTER_KIND_SHULK  {
				return 99999.0;
			}
			if param_hash == hash40("active_time_smash") && fighter_kind == *FIGHTER_KIND_SHULK  {
				return 99999.0;
			}
			if param_hash == hash40("unavailable_time_jump") && fighter_kind == *FIGHTER_KIND_SHULK  {
				return 0.1;
			}
			if param_hash == hash40("unavailable_time_speed") && fighter_kind == *FIGHTER_KIND_SHULK  {
				return 0.1;
			}
			if param_hash == hash40("unavailable_time_shield") && fighter_kind == *FIGHTER_KIND_SHULK  {
				return 0.1;
			}
			if param_hash == hash40("unavailable_time_buster") && fighter_kind == *FIGHTER_KIND_SHULK  {
				return 0.1;
			}
			if param_hash == hash40("unavailable_time_smash") && fighter_kind == *FIGHTER_KIND_SHULK  {
				return 0.1;
			}
			//Cloud
			if param_hash == hash40("limit_gauge_add") && fighter_kind == *FIGHTER_KIND_CLOUD  {
				return 99999.0;
			}
			//Hero
			if param_hash == hash40("sp_max") && fighter_kind == *FIGHTER_KIND_BRAVE {
				return 99999.0;
			}
			if param_hash == hash40("recover_sp") && fighter_kind == *FIGHTER_KIND_BRAVE {
				return 999.0;
			}
			if param_hash == hash40("sp_special_n1") && fighter_kind == *FIGHTER_KIND_BRAVE {
				return 0.0;
			}
			if param_hash == hash40("sp_special_n2") && fighter_kind == *FIGHTER_KIND_BRAVE {
				return 0.0;
			}
			if param_hash == hash40("sp_special_n3") && fighter_kind == *FIGHTER_KIND_BRAVE {
				return 0.0;
			}
			if param_hash == hash40("sp_special_s1") && fighter_kind == *FIGHTER_KIND_BRAVE {
				return 0.0;
			}
			if param_hash == hash40("sp_special_s2") && fighter_kind == *FIGHTER_KIND_BRAVE {
				return 0.0;
			}
			if param_hash == hash40("sp_special_s3") && fighter_kind == *FIGHTER_KIND_BRAVE {
				return 0.0;
			}
			if param_hash == hash40("sp_special_hi1") && fighter_kind == *FIGHTER_KIND_BRAVE {
				return 0.0;
			}
			if param_hash == hash40("sp_special_hi2") && fighter_kind == *FIGHTER_KIND_BRAVE {
				return 0.0;
			}
			if param_hash == hash40("sp_special_hi3") && fighter_kind == *FIGHTER_KIND_BRAVE {
				return 0.0;
			}
			//Terry
			if param_hash == hash40("super_special_damage") && fighter_kind == *FIGHTER_KIND_DOLLY {
				return 0.0;
			}
			//Mac
			if param_hash == hash40("special_n_hit_damage_mul_") && fighter_kind == *FIGHTER_KIND_LITTLEMAC {
				return 99999999.0;
			}
			if param_hash == hash40("special_n_atk_damage_mul_") && fighter_kind == *FIGHTER_KIND_LITTLEMAC {
				return 99999999.0;
			}
			//Luigi
			if param_hash == hash40("discharge_prob") && fighter_kind == *FIGHTER_KIND_LUIGI {
				return 50.0;
			}
			//Steve
			if param_hash == 0x1a0a95e89a && fighter_kind == *FIGHTER_KIND_PICKEL {
				return 0.4;
			}
			if param_hash == hash40("decrease_craft_weapon_durability") && fighter_kind == *FIGHTER_KIND_PICKEL {
				return -1.0;
			}
			if param_hash == 0x1a1bb90606 && fighter_kind == *FIGHTER_KIND_PICKEL {
				return 0.4;
			}
			if param_hash == 0x1b78f1e855 && fighter_kind == *FIGHTER_KIND_PICKEL {
				return 0.4;
			}
			if param_hash == 0x1aa714f1ca && fighter_kind == *FIGHTER_KIND_PICKEL {
				return 0.4;
			}
			if param_hash == 0x1da2122271 && fighter_kind == *FIGHTER_KIND_PICKEL {
				return 0.4;
			}
			//Rob
			if param_hash == hash40("miss_shoot_energy") && fighter_kind == *FIGHTER_KIND_ROBOT {
				return 1.0;
			}
			if param_hash == hash40("strong_shoot_energy") && fighter_kind == *FIGHTER_KIND_ROBOT {
				return 180.0;
			}
			if param_hash == hash40("start_energy") && fighter_kind == *FIGHTER_KIND_ROBOT {
				return 180.0;
			}
			//Megaman
			if param_hash == hash40("stick_percent") && fighter_kind == *FIGHTER_KIND_ROCKMAN {
				return 100.0;
			}
			else {
				return ret;
			}
		} else {
			return ret;
		}
}
#[skyline::hook(offset = INT_OFFSET)]
pub unsafe fn get_param_int_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
		let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
		let ret = original!()(module_accessor, param_type, param_hash);
		let fighter_kind = get_kind(boma);
		if smash::app::smashball::is_training_mode() == true {
			//Megaman
			if param_hash == hash40("stick_percent") && fighter_kind == *FIGHTER_KIND_ROCKMAN {
				return 100;
			}
			//Joker
			if param_hash == hash40("doyle_frame") && fighter_kind == *FIGHTER_KIND_JACK {
				return 9999999;
			}
			//Wii Fit
			if param_hash == 0x202e02971b && fighter_kind == *FIGHTER_KIND_WIIFIT {
				return 9999999;
			}
			if param_hash == hash40("breathing_waza_restore_frame") && fighter_kind == *FIGHTER_KIND_WIIFIT {
				return 1;
			}
			//GNW
			if param_hash == hash40("panel_9") && fighter_kind == *FIGHTER_KIND_GAMEWATCH {
				return 99;
			}
			//Cloud
			if param_hash == hash40("limit_break_clear_frame") && fighter_kind == *FIGHTER_KIND_CLOUD {
				return 9999;
			}
			//Robin
			if param_hash == hash40("thunder_sword_revival_time") && fighter_kind == *FIGHTER_KIND_REFLET {
				return 1;
			}
			if param_hash == hash40("grimoire_thunder_revival_time") && fighter_kind == *FIGHTER_KIND_REFLET {
				return 1;
			}
			if param_hash == hash40("grimoire_giga_fire_revival_time") && fighter_kind == *FIGHTER_KIND_REFLET {
				return 1;
			}
			if param_hash == hash40("grimoire_el_window_revival_time") && fighter_kind == *FIGHTER_KIND_REFLET {
				return 1;
			}
			if param_hash == hash40("grimoire_rizaia_usage_count_max") && fighter_kind == *FIGHTER_KIND_REFLET {
				return 1;
			}
			if param_hash == hash40("grimoire_rizaia_revival_time") && fighter_kind == *FIGHTER_KIND_REFLET {
				return 1;
			}
			if param_hash == hash40("thunder_sword_revival_time") && fighter_kind == *FIGHTER_KIND_REFLET {
				return 1;
			}
			//Steve
			if param_hash == hash40("mining_gold_get_num") && fighter_kind == *FIGHTER_KIND_PICKEL {
				return 12;
			}
			if param_hash == hash40("mining_red_stone_get_num") && fighter_kind == *FIGHTER_KIND_PICKEL {
				return 15;
			}
			if param_hash == hash40("mining_diamond_get_num") && fighter_kind == *FIGHTER_KIND_PICKEL {
				return 5;
			}
			if param_hash == hash40("craft_material_num_stone") && fighter_kind == *FIGHTER_KIND_PICKEL {
				return 0;
			}
			//Pit / Dpit
			if param_hash == hash40("no_shield_frame") && fighter_kind == *FIGHTER_KIND_PIT {
				return 1;
			}
			if param_hash == hash40("no_shield_frame") && fighter_kind == *FIGHTER_KIND_PITB {
				return 1;
			}
			//KROOL
			if param_hash == hash40("spawn_frame") && fighter_kind == *FIGHTER_KIND_KROOL {
				return 1;
			}
			if param_hash == hash40("spawn_frame_add") && fighter_kind == *FIGHTER_KIND_KROOL {
				return 1;
			}
			if param_hash == hash40("spawn_frame_max") && fighter_kind == *FIGHTER_KIND_KROOL {
				return 1;
			}
			//D3
			if param_hash == 0x097ffb6297 && fighter_kind == *FIGHTER_KIND_DEDEDE {
				return 100;
			}
			else {
				return ret;
			}
		} else {
			return ret;
		}
}
#[skyline::hook(replace = smash::app::sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
	if smash::app::smashball::is_training_mode() == true {
		let fighter_kind = smash::app::utility::get_kind(boma);
		let mut l2c_agent = L2CAgent::new(lua_state);
		let mut hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
		l2c_agent.clear_lua_stack();
		for i in 0..36 {
			if i == 21 && hitbox_params[i].get_f32() > 0.0 {
				l2c_agent.push_lua_stack(&mut L2CValue::new_num(1.0));
			} else {
				l2c_agent.push_lua_stack(&mut hitbox_params[i]);
			}
		}
	};
    original!()(lua_state);
}
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}
#[fighter_frame_callback]
pub fn get_item(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		if smash::app::smashball::is_training_mode() == true {
			let fighter_instance = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
			let mut fighter_num = 0;
			if fighter_instance == 0 {
				fighter_num = 0;
			} else if fighter_instance == 1 {
				fighter_num = 1;
			} else if fighter_instance == 2 {
				fighter_num = 2;
			} else if fighter_instance == 3 {
				fighter_num = 3;
			} else if fighter_instance == 4 {
				fighter_num = 4;
			} else if fighter_instance == 5 {
				fighter_num = 5;
			} else if fighter_instance == 6 {
				fighter_num = 6;
			} else if fighter_instance == 7 {
				fighter_num = 7;
			};
			if fighter_kind == *FIGHTER_KIND_DIDDY { //Donesy!
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)){
					DIDDY[fighter_num] = 1;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
					if DIDDY[fighter_num] == 1 {
						MotionModule::set_rate(boma, 4.0);
						DIDDY[fighter_num] = 0;
					}
				};
			};
			if fighter_kind == *FIGHTER_KIND_MURABITO { //Donesy!
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && ItemModule::is_have_item(boma, *ITEM_KIND_WOOD) == false {
					ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_WOOD), 0, 0, false, false);
					CancelModule::enable_cancel(boma);
				};
			};
			if fighter_kind == *FIGHTER_KIND_ROBOT { 
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && ItemModule::is_have_item(boma, *ITEM_KIND_WOOD) == false {
					ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_ROBOTGYRO), 0, 0, false, false);
					CancelModule::enable_cancel(boma);
				};
			};
			if fighter_kind == *FIGHTER_KIND_SHEIK { 
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) {
					WorkModule::set_flag(boma, true, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_CHARGE_MAX);
					WorkModule::set_int(boma, 6, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_NEEDLE_COUNT);
					CancelModule::enable_cancel(boma);
				};
			};
			if fighter_kind == *FIGHTER_KIND_SAMUS || fighter_kind == *FIGHTER_KIND_SAMUSD { 
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) {
					SAMUSCHARGE[fighter_num] = -1;
					WorkModule::set_int(boma, 0, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
					CancelModule::enable_cancel(boma);
				};
				if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)  {
					SAMUSCHARGE[fighter_num] = WorkModule::get_int(boma, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
				};
				if SAMUSCHARGE[fighter_num] != -1 {
					WorkModule::set_int(boma, SAMUSCHARGE[fighter_num], *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
				};
			};
			if fighter_kind == *FIGHTER_KIND_MIIGUNNER { 
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) {
					SAMUSCHARGE[fighter_num] = -1;
					WorkModule::set_int(boma, 0, *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
					CancelModule::enable_cancel(boma);
				};
				if [*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_HOLD].contains(&status_kind) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)  {
					SAMUSCHARGE[fighter_num] = WorkModule::get_int(boma, *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
				};
				if SAMUSCHARGE[fighter_num] != -1 {
					WorkModule::set_int(boma, SAMUSCHARGE[fighter_num], *FIGHTER_MIIGUNNER_INSTANCE_WORK_ID_INT_GUNNER_CHARGE_COUNT);
				};
			};
			if fighter_kind == *FIGHTER_KIND_PACMAN { 
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) {
					PAC[fighter_num] = -1;
					WorkModule::set_int(boma, 0, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
					CancelModule::enable_cancel(boma);
				};
				if [*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_HOLD].contains(&status_kind) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)  {
					PAC[fighter_num] = WorkModule::get_int(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
				};
				if PAC[fighter_num] != -1 {
					WorkModule::set_int(boma, PAC[fighter_num], *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
				};
			};
			if fighter_kind == *FIGHTER_KIND_MEWTWO { 
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) {
					SAMUSCHARGE[fighter_num] = -1;
					WorkModule::set_int(boma, 0, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME);
					CancelModule::enable_cancel(boma);
				};
				if [*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD].contains(&status_kind) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)  {
					SAMUSCHARGE[fighter_num] = WorkModule::get_int(boma, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
				};
				if SAMUSCHARGE[fighter_num] != -1 && [*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD].contains(&status_kind) {
					MotionModule::set_frame(boma, SAMUSCHARGE[fighter_num] as f32, true);
				};
			};
			if fighter_kind == *FIGHTER_KIND_JACK { //
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) == 1.0{
					DamageModule::add_damage(boma, 110.0, 0);
					ARSENE[fighter_num] = 5;
					CancelModule::enable_cancel(boma);
				};
				if ARSENE[fighter_num] > 0{
					ARSENE[fighter_num] -= 1;
					if ARSENE[fighter_num] == 0 {
						DamageModule::add_damage(boma, -110.0, 0);
					};
				};
			};
			if fighter_kind == *FIGHTER_KIND_TOONLINK { 
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)){
					TINK[fighter_num] = 1;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
					if TINK[fighter_num] == 1 {
						MotionModule::set_rate(boma, 4.0);
						TINK[fighter_num] = 0;
					}
				};
			};
			if fighter_kind == *FIGHTER_KIND_LINK { 
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)){
					LINK[fighter_num] = 1;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
					if LINK[fighter_num] == 1 {
						MotionModule::set_rate(boma, 4.0);
						LINK[fighter_num] = 0;
					}
				};
			};
			if fighter_kind == *FIGHTER_KIND_RICHTER {
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && ItemModule::is_have_item(boma, *ITEM_KIND_RICHTERHOLYWATER) == false {
					ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_RICHTERHOLYWATER), 0, 0, false, false);
					CancelModule::enable_cancel(boma);
				};
			};
			if fighter_kind == *FIGHTER_KIND_SIMON {
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && ItemModule::is_have_item(boma, *ITEM_KIND_SIMONHOLYWATER) == false {
					ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_SIMONHOLYWATER), 0, 0, false, false);
					CancelModule::enable_cancel(boma);
				};
			};
			if fighter_kind == *FIGHTER_KIND_YOUNGLINK {
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && ItemModule::is_have_item(boma, *ITEM_KIND_YOUNGLINKBOMB) == false {
					ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_YOUNGLINKBOMB), 0, 0, false, false);
					CancelModule::enable_cancel(boma);
				};
			};
			if fighter_kind == *FIGHTER_KIND_DAISY  {
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)){
					PAISY[fighter_num] = 1;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
					if PAISY[fighter_num] == 1 {
						MotionModule::set_rate(boma, 4.0);
						PAISY[fighter_num] = 0;
					}
				};
			};
			if fighter_kind == *FIGHTER_KIND_PEACH  {
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) {
					PAISY[fighter_num] = 1;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
					if PAISY[fighter_num] == 1 {
						MotionModule::set_rate(boma, 4.0);
						PAISY[fighter_num] = 0;
					}
				};
			};
			if fighter_kind == *FIGHTER_KIND_ROCKMAN { //works, lots of metal blades 
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && ItemModule::is_have_item(boma, *ITEM_KIND_METALBLADE) == false && WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE) == false {
					ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_METALBLADE), 0, 0, false, false);
					CancelModule::enable_cancel(boma);
				};
			};
			if fighter_kind == *FIGHTER_KIND_BUDDY { //Spawns
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) < 2.0 && ArticleModule::is_exist(boma, *ITEM_KIND_BUDDYBOMB) == false {
					ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_BUDDYBOMB), 0, 0, false, false);
					CancelModule::enable_cancel(boma);
				};
			};
			if fighter_kind == *FIGHTER_KIND_KOOPAJR { //Spawns
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && ItemModule::is_have_item(boma, *ITEM_KIND_MECHAKOOPA) == false {
					ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_MECHAKOOPA), 0, 0, false, false);
					CancelModule::enable_cancel(boma);
				};
			};
			if fighter_kind == *FIGHTER_KIND_EDGE { //
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) == 1.0{
					DamageModule::add_damage(boma, 130.0, 0);
					SEPH[fighter_num] = 5;
					CancelModule::enable_cancel(boma);
				};
				if SEPH[fighter_num] > 0{
					SEPH[fighter_num] -= 1;
					if SEPH[fighter_num] == 0 {
						DamageModule::add_damage(boma, -130.0, 0);
					};
				};
			};
			if fighter_kind == *FIGHTER_KIND_MIIFIGHTER  {
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) == 1.0 {
					WorkModule::set_int(boma, 1, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_100KICK_REACTION_MUL_RECOVER_FRAME);
					WorkModule::set_int(boma, 1, *FIGHTER_MIIFIGHTER_INSTANCE_WORK_ID_INT_100KICK_REACTION_MUL_VALID_FRAME);
					CancelModule::enable_cancel(boma);
				};
			};
			if fighter_kind == *FIGHTER_KIND_LUCARIO  {
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) == 1.0 {
					CancelModule::enable_cancel(boma);
					let mut damage_add = 0.0;
					let current_dmg = DamageModule::damage(boma, 0);
					if current_dmg < 65.0 {
						damage_add = 65.0 - current_dmg;
						DamageModule::add_damage(boma, damage_add, 0);
					}else if current_dmg < 190.0 {
						damage_add = 190.0 - current_dmg;
						DamageModule::add_damage(boma, damage_add, 0);
					} else if current_dmg >= 190.0 {
						damage_add = -1.0 * current_dmg;
						DamageModule::add_damage(boma, damage_add, 0);
					};
				};
			};
			if fighter_kind == *FIGHTER_KIND_REFLET  {
				if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) && ItemModule::is_have_item(boma, *ITEM_KIND_METALBLADE) == false && WorkModule::is_flag(boma, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE) == false {
					WorkModule::set_int(boma, 1, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
					CancelModule::enable_cancel(boma);
				};
			};
		};
	};
}
#[fighter_frame_callback]
pub fn training_restart(fighter : &mut L2CFighterCommon) {
    unsafe {
		if smash::app::smashball::is_training_mode() == true {
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if RESTART == true {
				if ControlModule::get_stick_y(boma) <= -0.5 && ENTRY_ID == 0 {
					RESET_PLACE = 1;
				} else if ControlModule::get_stick_y(boma) >= 0.5 && ENTRY_ID == 0 {
					RESET_PLACE = 2;
				} else if ControlModule::get_stick_x(boma) >= 0.5 && ENTRY_ID == 0 {
					RESET_PLACE = 3;
				}  else if ControlModule::get_stick_x(boma) <= -0.5 && ENTRY_ID == 0 {
					RESET_PLACE = 4;
				};
				RESTART = false;
				println!("RESET: {}", ENTRY_ID);
			};
			if smash::app::sv_information::is_ready_go() == false {
				RESTART = true;
				RESET_PLACE = 0;
				println!("READY GO OFF: {}", ENTRY_ID);
			};
			if RESET_PLACE == 1 {
				let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma), y: 0.0, z: 0.0 };
				PostureModule::set_pos(boma, &pos);
				PostureModule::init_pos(boma, &pos, true, true);
				println!("Move down: {}", ENTRY_ID);
			};
			if RESET_PLACE == 2 {
				let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma)*-1.0, y: PostureModule::pos_y(boma), z: 0.0 };
				PostureModule::set_pos(boma, &pos);
				PostureModule::init_pos(boma, &pos, true, true);
				PostureModule::reverse_lr(boma);
				PostureModule::update_rot_y_lr(boma);
				println!("Move swap: {}", ENTRY_ID);
			};
			if RESET_PLACE == 4 {
				if ENTRY_ID == 0 {
					let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma)-17.0, y: 0.0, z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
				} else if ENTRY_ID == 1 {
					let pos = smash::phx::Vector3f { x: (PostureModule::pos_x(boma)+25.0)*-1.0, y: 0.0, z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
				};
			};
			if RESET_PLACE == 3 {
				if ENTRY_ID == 0 {
					let pos = smash::phx::Vector3f { x: (PostureModule::pos_x(boma)-17.0)*-1.0, y: 0.0, z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
				} else if ENTRY_ID == 1 {
					let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma)+25.0, y: 0.0, z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
				};
			};
			if ENTRY_ID == 1 {
				if RESET_PLACE != 0 {
					println!("ENTRY_ID 1 is being shown");
					RESET_PLACE = 0;
				};
			};
		};
	};
}	
#[fighter_frame_callback]
pub fn self_dmg(fighter : &mut L2CFighterCommon) {
    unsafe {
		if smash::app::smashball::is_training_mode() == true {
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			let fighter_instance = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
			let mut fighter_num = 0;
			if fighter_instance == 0 {
				fighter_num = 0;
			} else if fighter_instance == 1 {
				fighter_num = 1;
			} else if fighter_instance == 2 {
				fighter_num = 2;
			} else if fighter_instance == 3 {
				fighter_num = 3;
			} else if fighter_instance == 4 {
				fighter_num = 4;
			} else if fighter_instance == 5 {
				fighter_num = 5;
			} else if fighter_instance == 6 {
				fighter_num = 6;
			} else if fighter_instance == 7 {
				fighter_num = 7;
			};
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) && status_kind == *FIGHTER_STATUS_KIND_GUARD {
				SELFDAMAGE[1] = -25.0;
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
			};
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) && status_kind == *FIGHTER_STATUS_KIND_GUARD {
				SELFDAMAGE[1] = 25.0;
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
			};
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) && status_kind == *FIGHTER_STATUS_KIND_GUARD {
				SELFDAMAGE[0] = -25.0;
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
			};
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) && status_kind == *FIGHTER_STATUS_KIND_GUARD {
				SELFDAMAGE[0] = 25.0;
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
			};
			if SELFDAMAGE[fighter_num] == -25.0 && DamageModule::damage(boma, 0) == 0.0 {
				SELFDAMAGE[fighter_num] = 0.0;
			};
			if SELFDAMAGE[fighter_num] != 0.0 {
				DamageModule::add_damage(boma, SELFDAMAGE[fighter_num], 0);
				SELFDAMAGE[fighter_num] = 0.0;
			};
		};
	};
}	

pub fn install() {
    unsafe{
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
		if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
    }
	skyline::install_hook!(attack_replace);
	skyline::install_hook!(get_param_float_replace);
	skyline::install_hook!(get_param_int_replace);
	smashline::install_agent_frame_callbacks!(get_item);
	smashline::install_agent_frame_callbacks!(self_dmg);
	smashline::install_agent_frame_callbacks!(training_restart);
}