use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::*;
use acmd::*;
use smash::phx::Hash40;
use smash::hash40;
use smash::params::fighter_param::*;
use std::os::raw::c_int;

static mut IS_RECORD : bool = false;
static mut IS_PLAY : bool = false;
static mut RESTART : bool = false;
static mut STATUS : Vec<i32> = vec![];
static mut AERIAL_KIND : Vec<i32> = vec![];
static mut IS_CHANGING : Vec<i32> = vec![];
static mut ATTACK : Vec<i32> = vec![];
static mut SPECIAL : Vec<i32> = vec![];
static mut GUARD : Vec<i32> = vec![];
static mut JUMP : Vec<i32> = vec![];
static mut FASTFALL : Vec<i32> = vec![];
static mut DASHRECORDS : Vec<i32> = vec![];
static mut PAISYFLOAT : Vec<i32> = vec![];
static mut PICKEL_HI3 : Vec<i32> = vec![];
static mut PICKEL_AIRHI : Vec<i32> = vec![];
static mut PICKEL_SH : Vec<i32> = vec![];
static mut PHANTOM : Vec<i32> = vec![];
static mut ZDROP : Vec<i32> = vec![];
static mut ZGET : Vec<i32> = vec![];
static mut ZAIR : Vec<i32> = vec![];
static mut X : Vec<f32> = vec![];
static mut Y : Vec<f32> = vec![];
static mut STICK_DIR : Vec<f32> = vec![];
static mut LR : Vec<f32> = vec![];
static mut COMMAND_FLAG0 : Vec<i32> = vec![];
static mut COMMAND_FLAG1 : Vec<i32> = vec![];
static mut COMMAND_FLAG2 : Vec<i32> = vec![];
static mut COMMAND_FLAG3 : Vec<i32> = vec![];
static mut MAX : i32 = 0;
static mut STEP : i32 = 0;
static mut IS_BURY : bool = false;

static BITMASK_ARR: [u32; 18] = [
  1u32 << 0,
  1u32 << 1,
  1u32 << 2,
  1u32 << 3,
  1u32 << 5,
  1u32 << 10,
  1u32 << 12,
  1u32 << 13,
  1u32 << 14,
  1u32 << 4,
  1u32 << 6,
  1u32 << 7,
  1u32 << 15,
  1u32 << 8,
  1u32 << 9,
  1u32 << 11,
  1u32 << 16,
  1u32 << 17
];

#[skyline::hook(replace = ControlModule::get_attack_air_kind)]
pub unsafe fn handle_get_attack_air_kind(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> i32 {
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if STEP >= MAX {
		STEP = 0;
		IS_PLAY = false;
	};
	if IS_PLAY == true && STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true && AERIAL_KIND[STEP as usize] != 0{
		println!("new");
		return AERIAL_KIND[STEP as usize];
	} else {
		println!("og");
		original!()(boma)
	}
}
#[skyline::hook(replace = ControlModule::get_stick_dir)]
pub unsafe fn handle_stick_dir(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if STEP >= MAX {
		STEP = 0;
		IS_PLAY = false;
	};
	if IS_PLAY == true && STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
		println!("new");
		return STICK_DIR[STEP as usize];
	} else {
		println!("og");
		original!()(boma)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_command_flag_cat)]
pub unsafe fn command_flag_cat_hook(boma: &mut smash::app::BattleObjectModuleAccessor, int: c_int) -> i32 {
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if STEP >= MAX {
		STEP = 0;
		IS_PLAY = false;
	};
	if IS_PLAY == true && STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
		let mut returner = 0;
		if int == 0 {
			returner = COMMAND_FLAG0[STEP as usize];
		} else if int == 1 {
			returner = COMMAND_FLAG1[STEP as usize];
		} else if int == 2 {
			returner = COMMAND_FLAG2[STEP as usize];
		} else if int == 3 {
			returner = COMMAND_FLAG3[STEP as usize];
		};
		return returner;
	} else {
		original!()(boma, int)
	}
}
unsafe fn ControlModule_set_button_on(boma: *mut smash::app::BattleObjectModuleAccessor, button: i32) {
  let mask = BITMASK_ARR[button as usize];
  let control_module = *(boma as *const u64).add(0x48 / 0x8);
  let internal_object = *(control_module as *const u64).add(0x110 / 0x8) as *mut u32;
  *internal_object.add(0x7C / 0x4) |= mask;
}
unsafe fn ControlModule_set_button_off(boma: *mut smash::app::BattleObjectModuleAccessor, button: i32) {
  let mask = BITMASK_ARR[button as usize];
  let control_module = *(boma as *const u64).add(0x48 / 0x8);
  let internal_object = *(control_module as *const u64).add(0x110 / 0x8) as *mut u32;
  *internal_object.add(0x7C / 0x4) &= !mask;
}

pub fn test(fighter : &mut L2CFighterCommon) {
    unsafe {
		let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		ControlModule_set_button_off(boma, *CONTROL_PAD_BUTTON_GUARD);
	}
}
pub fn repeater(fighter : &mut L2CFighterCommon) {
    unsafe {
		if smash::app::smashball::is_training_mode() == true {
			let lua_state = fighter.lua_state_agent;
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
			let fighter_kind = smash::app::utility::get_kind(boma);
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_num = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if smash::app::sv_information::is_ready_go() == false {
				STEP = 0;
				IS_PLAY = false;
			};
			if fighter_num == 1   {
				if IS_PLAY == true {
					if STEP >= MAX {
						STEP = 0;
						IS_PLAY = false;
					};
					let aerial_status = AERIAL_KIND[STEP as usize];
					let attacking = ATTACK[STEP as usize];
					let specialing = SPECIAL[STEP as usize];
					let jumping = JUMP[STEP as usize];
					let fastfalling = FASTFALL[STEP as usize];
					let next_status = STATUS[STEP as usize];
					let dash = DASHRECORDS[STEP as usize];
					if next_status == *FIGHTER_STATUS_KIND_TURN_DASH && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) && (X[STEP as usize] * PostureModule::lr(boma)) <= -0.85  {
						if status_kind != *FIGHTER_STATUS_KIND_TURN_DASH || (dash >= 2 && MotionModule::frame(boma) > 5.0 && dash < 20) {
							StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_TURN_DASH, true);
						};
					};
					ControlModule::reset_main_stick(boma);
					ControlModule::set_main_stick_x(boma, X[STEP as usize]);
					ControlModule::set_main_stick_y(boma, Y[STEP as usize]);
					if AERIAL_KIND[STEP as usize] != -1 {
						ControlModule::set_attack_air_kind(boma, AERIAL_KIND[STEP as usize]);
						FighterControlModuleImpl::update_attack_air_kind(boma, true);
					};
					if ATTACK[STEP as usize] == 1 {
						ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK);
					};
					if SPECIAL[STEP as usize] == 1 {
						ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL);
					};
					if JUMP[STEP as usize] == 1 {
						ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_JUMP);
					};
					if fastfalling == 1 {
						WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					} else {
						WorkModule::set_flag(boma, false, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					};
					STEP += 1;
				};
			};
		};
	}
}
//COPYCAT MACRO
pub fn recorder(fighter : &mut L2CFighterCommon) {
    unsafe {
		if smash::app::smashball::is_training_mode() == true {
			let lua_state = fighter.lua_state_agent;
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_num = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let fighter_kind = smash::app::utility::get_kind(boma);
			if fighter_num == 0 {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) && status_kind != *FIGHTER_STATUS_KIND_GUARD && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD) {
					IS_RECORD = true;
					IS_PLAY = false;
					STATUS.clear();
					ATTACK.clear();
					SPECIAL.clear();
					GUARD.clear();
					JUMP.clear();
					IS_CHANGING.clear();
					AERIAL_KIND.clear();
					FASTFALL.clear();
					DASHRECORDS.clear();
					PAISYFLOAT.clear();
					PICKEL_AIRHI.clear();
					PICKEL_HI3.clear();
					ZAIR.clear();
					PICKEL_SH.clear();
					ZDROP.clear();
					ZGET.clear();
					LR.clear();
					X.clear();
					Y.clear();
					STICK_DIR.clear();
					COMMAND_FLAG0.clear();
					COMMAND_FLAG1.clear();
					COMMAND_FLAG2.clear();
					COMMAND_FLAG3.clear();
					MAX = 0;
					if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
					};
				};
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) && status_kind != *FIGHTER_STATUS_KIND_GUARD && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD) {
					IS_RECORD = false;
					if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
					};
				};
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) && MAX != 0 && status_kind != *FIGHTER_STATUS_KIND_GUARD && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD) {
						IS_PLAY = true;
						IS_RECORD = false;
						STEP = 0;
						if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
						};
				};
				if IS_RECORD == true {
					MAX += 1;
					if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 {
						COMMAND_FLAG0.push(0);
					} else {
						COMMAND_FLAG0.push(ControlModule::get_command_flag_cat(boma, 0));
					};
					STICK_DIR.push(ControlModule::get_stick_dir(boma));
					if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR ||  status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR{
						if MotionModule::motion_kind(boma) == hash40("attack_air_f"){
							AERIAL_KIND.push(2);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_b"){
							AERIAL_KIND.push(3);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_hi"){
							AERIAL_KIND.push(4);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_lw"){
							AERIAL_KIND.push(5);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_f_hi"){
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_F);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_f_lw"){
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_F);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_b_hi"){
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_B);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_b_lw"){
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_B);
						} else {
							AERIAL_KIND.push(1);
						};
					} else {
						AERIAL_KIND.push(-1);
					};
					let cat2 = ControlModule::get_command_flag_cat(boma, 1);
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 && (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0{
						COMMAND_FLAG1.push(0);
					} else {
						COMMAND_FLAG1.push(ControlModule::get_command_flag_cat(boma, 1));
					};
					COMMAND_FLAG2.push(ControlModule::get_command_flag_cat(boma, 2));
					COMMAND_FLAG3.push(ControlModule::get_command_flag_cat(boma, 3));
					println!("0:{}  1:{}  2:{}  3:{}", ControlModule::get_command_flag_cat(boma, 0), ControlModule::get_command_flag_cat(boma, 1), ControlModule::get_command_flag_cat(boma, 2), ControlModule::get_command_flag_cat(boma, 3));
					STATUS.push(status_kind);
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
						GUARD.push(1);
					} else {
						GUARD.push(0);
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
						ATTACK.push(1);
					} else {
						ATTACK.push(0);
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						SPECIAL.push(1);
					} else {
						SPECIAL.push(0);
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP){
						JUMP.push(1);
					} else {
						JUMP.push(0);
					};
					if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
						FASTFALL.push(1);
					} else {
						FASTFALL.push(0);
					};
					X.push(ControlModule::get_stick_x(boma)*-1.0);
					Y.push(ControlModule::get_stick_y(boma));
					DASHRECORDS.push(ControlModule::get_flick_x(boma));
					//println!("X: {}, Y: {}, Status: {}, Aerial Kind: {}, Is Jump: {}, Is Attack: {}, Is Shield: {}, Dash Flick: {}", ControlModule::get_stick_x(boma)*-1.0, ControlModule::get_stick_y(boma), status_kind, AERIAL_KIND[(MAX-1) as usize], JUMP[(MAX-1) as usize], ATTACK[(MAX-1) as usize], GUARD[(MAX-1) as usize], DASHRECORDS[(MAX-1) as usize]);
				};
			};
		};
    }
}

pub fn install() {
    acmd::add_custom_hooks!(recorder);
    acmd::add_custom_hooks!(repeater);
	skyline::install_hook!(command_flag_cat_hook);
	skyline::install_hook!(handle_get_attack_air_kind);
}