use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use acmd::*;
use smash::phx::Hash40;
use smash::hash40;
use smash::params::fighter_param::*;

static mut IS_RECORD : bool = false;
static mut IS_PLAY : bool = false;
static mut RESTART : bool = false;
static mut STATUS : Vec<i32> = vec![];
static mut AERIAL_KIND : Vec<i32> = vec![];
static mut THROW_KIND : Vec<i32> = vec![];
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
static mut LR : Vec<f32> = vec![];
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
			if fighter_num == 0  && IS_BURY == true {
				if status_kind == *FIGHTER_STATUS_KIND_DAMAGE {
					StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_BURY, true);
					IS_BURY = false;
				} else {
					StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_DAMAGE, true);
				};
			};
			if fighter_num == 1   {
				if IS_PLAY == true {
					if STEP >= MAX {
						STEP = 0;
						IS_PLAY = false;
					};
					let next_status = STATUS[STEP as usize];
					let mut next_status_2 = 0;
					if fighter_kind == *FIGHTER_KIND_POPO {
						if STEP < 7 && STEP < MAX -1 {
							STEP = 7;
						};
					}
					if STEP < MAX -1 {
						next_status_2 = STATUS[(STEP+1) as usize];
					} else {
						next_status_2 = STATUS[STEP as usize];
					};
					let throw_status = THROW_KIND[STEP as usize];
					let aerial_status = AERIAL_KIND[STEP as usize];
					let changing_status = IS_CHANGING[STEP as usize];
					let attacking = ATTACK[STEP as usize];
					let specialing = SPECIAL[STEP as usize];
					let jumping = JUMP[STEP as usize];
					let shielding = GUARD[STEP as usize];
					let fastfalling = FASTFALL[STEP as usize];
					let mut dash = DASHRECORDS[STEP as usize];
					let paisyfloat = PAISYFLOAT[STEP as usize];
					let pickel_hi3 = PICKEL_HI3[STEP as usize];
					let pickel_airhi = PICKEL_AIRHI[STEP as usize];
					let pickel_sh = PICKEL_SH[STEP as usize];
					let zelda_phantom = PHANTOM[STEP as usize];
					let zair = ZAIR[STEP as usize];
					let zdrop = ZDROP[STEP as usize];
					let zget = ZGET[STEP as usize];
					ControlModule::reset_main_stick(boma);
					ControlModule::set_main_stick_x(boma, X[STEP as usize]);
					ControlModule::set_main_stick_y(boma, Y[STEP as usize]);
					if shielding == 0 {
						WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
						WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
					};
					//println!("Current frame: {}, Cancel frame: {}", MotionModule::frame(boma) as i32, FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false));
					if true == true {
						//Well this is fun.
						//Mobility
						if zdrop == 1 {
							ItemModule::throw_item(boma, 0.0, 0.0, 1.0, 0, true, 0.0); // This emulates the zdrop functionality by throwing the item with a power of 0
							println!("zdrop!!!!");
						};
						if next_status == *FIGHTER_STATUS_KIND_DASH && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) && (X[STEP as usize] * PostureModule::lr(boma)) >= 0.85{
							if dash >= 2 && dash < 20{
								StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_DASH, true);
							};
						};
						if [*FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, *FIGHTER_RYU_STATUS_KIND_DASH_BACK].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) && (X[STEP as usize] * PostureModule::lr(boma)) <= -0.85{
							if dash >= 2 && dash < 20{
								if fighter_kind == *FIGHTER_KIND_DOLLY {
									StatusModule::change_status_force(boma, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, true);
								};
								if fighter_kind == *FIGHTER_KIND_RYU {
									StatusModule::change_status_force(boma, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, true);
								};
							};
						};
						if next_status == *FIGHTER_STATUS_KIND_TURN_DASH && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) && (X[STEP as usize] * PostureModule::lr(boma)) <= -0.85  {
							if status_kind != *FIGHTER_STATUS_KIND_TURN_DASH || (dash >= 2 && MotionModule::frame(boma) > 5.0 && dash < 20) {
								StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_TURN_DASH, true);
							};
						};
						if next_status == *FIGHTER_STATUS_KIND_WALK && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALK, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_SQUAT && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_SQUAT_RV && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_RV) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT_RV, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_SQUAT_B && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_B) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT_B, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_SQUAT_F && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT_F) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT_F, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_RUN && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_RUN) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_RUN, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_RUN && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_RUN, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_FALL && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_FALL_SPECIAL && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_JUMP_SQUAT && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && status_kind != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_JUMP_AERIAL && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && status_kind != *FIGHTER_STATUS_KIND_JUMP_AERIAL  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
						};
						if [*FIGHTER_KIND_KIRBY, *FIGHTER_KIND_PURIN, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_PIT, *FIGHTER_KIND_PITB, *FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_BUDDY, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_PLIZARDON].contains(&fighter_kind) {
							if next_status == 13 && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && jumping == 1 {
								StatusModule::change_status_request_from_script(boma, 13, true);
							};
						};
						if next_status == *FIGHTER_STATUS_KIND_TURN && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN) && (X[STEP as usize] * PostureModule::lr(boma)) > -0.85{
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
						};
						//Normals
						if attacking == 1 {
							if next_status == *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, true);
								PostureModule::set_lr(boma, LR[STEP as usize]);
								PostureModule::update_rot_y_lr(boma);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
								PostureModule::set_lr(boma, LR[STEP as usize]);
								PostureModule::update_rot_y_lr(boma);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK && status_kind == *FIGHTER_STATUS_KIND_ATTACK && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && attacking == 1 && StopModule::is_stop(boma) == false{
								if MotionModule::motion_kind(boma) == hash40("attack_12") {
									WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
									MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_13"), 0.0, 1.0, false, 0.0, false, false);
								};
								if MotionModule::motion_kind(boma) == hash40("attack_11") {
									WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
									MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_12"), 0.0, 1.0, false, 0.0, false, false);
								};
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_DASH {
								if status_kind != *FIGHTER_STATUS_KIND_ATTACK_DASH && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
									StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
								};
							};
							if next_status_2 == *FIGHTER_STATUS_KIND_ATTACK_AIR && fighter_kind != *FIGHTER_KIND_PIKMIN {
								if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
									if jumping == 1 && [*FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind) == false && STEP > 0 {
										if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_JUMP || JUMP[(STEP-1) as usize] == 0 {
											KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL);
											println!("dj should occur");
										};
									};
									StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
									println!("aerial occurs");
								};
								if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
									KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
									StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
									println!("sh aerial occurs");
								};
							};
							if next_status_2 == *FIGHTER_STATUS_KIND_ATTACK_AIR && fighter_kind == *FIGHTER_KIND_PIKMIN {
								if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || (WorkModule::is_flag(boma, *FIGHTER_PIKMIN_INSTANCE_ATTACK_AIR_WORK_FLAG_FALL_SPECIAL) == false && [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind)) {
									if jumping == 1 && [*FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind) == false && STEP > 0 {
										if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_JUMP || JUMP[(STEP-1) as usize] == 0 {
											KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL);
											println!("dj should occur");
										};
									};
									StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
									println!("aerial occurs");
								};
								if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
									KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
									StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
									println!("sh aerial occurs oli");
								};
								if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
									WorkModule::on_flag(boma, *FIGHTER_PIKMIN_INSTANCE_ATTACK_AIR_WORK_FLAG_FALL_SPECIAL);
								};
							};
							if fighter_kind == *FIGHTER_KIND_ROBOT && next_status == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK && [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP].contains(&status_kind) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
							};
							if  MotionModule::motion_kind(boma) == hash40("attack_air_n") {
								if aerial_status == 1{
									MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_f"), 0.0, 1.0, false, 0.0, false, false);
									MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									if fighter_kind == *FIGHTER_KIND_EFLAME {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_f"), 2.0, 1.0, false, 0.0, false, false);
										MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									};
									if fighter_kind == *FIGHTER_KIND_ELIGHT {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_f"), 5.8, 2.85, false, 0.0, false, false);
										MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									};
									if fighter_kind == *FIGHTER_KIND_ROSETTA {
										ArticleModule::change_motion(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO, smash::phx::Hash40::new("attack_air_f"), false, 0.0);
									};
								} else if aerial_status == 2{
									MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_b"), 0.0, 1.0, false, 0.0, false, false);
									MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									if fighter_kind == *FIGHTER_KIND_EFLAME {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_b"), 4.0, 1.0, false, 0.0, false, false);
										MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									};
									if fighter_kind == *FIGHTER_KIND_ELIGHT {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_b"), 0.5, 1.0, false, 0.0, false, false);
										MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									};
									if fighter_kind == *FIGHTER_KIND_ROSETTA {
										ArticleModule::change_motion(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO, smash::phx::Hash40::new("attack_air_b"), false, 0.0);
									};
								} else if aerial_status == 3{
									MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_hi"), 0.0, 1.0, false, 0.0, false, false);
									MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									if fighter_kind == *FIGHTER_KIND_EFLAME {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_hi"), 2.0, 1.0, false, 0.0, false, false);
										MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									};
									if fighter_kind == *FIGHTER_KIND_ELIGHT {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_hi"), 2.0, 1.0, false, 0.0, false, false);
										MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									};
									if fighter_kind == *FIGHTER_KIND_ROSETTA {
										ArticleModule::change_motion(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO, smash::phx::Hash40::new("attack_air_hi"), false, 0.0);
									};
								} else if aerial_status == 4{
									MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_lw"), 0.0, 1.0, false, 0.0, false, false);
									MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									if fighter_kind == *FIGHTER_KIND_EFLAME {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_lw"), 4.0, 1.0, false, 0.0, false, false);
										MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									};
									if fighter_kind == *FIGHTER_KIND_ELIGHT {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_lw"), 4.0, 1.0, false, 0.0, false, false);
										MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									};
									if fighter_kind == *FIGHTER_KIND_ROSETTA {
										ArticleModule::change_motion(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO, smash::phx::Hash40::new("attack_air_lw"), false, 0.0);
									};
								} else if [*FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER].contains(&fighter_kind) {
									if aerial_status == 5 {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_f_hi"), 0.0, 1.0, false, 0.0, false, false);
										MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									} else if aerial_status == 6 {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_f_lw"), 0.0, 1.0, false, 0.0, false, false);
										MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									};
									if aerial_status == 7 {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_b_hi"), 0.0, 1.0, false, 0.0, false, false);
										MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									} else if aerial_status == 8 {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_b_lw"), 0.0, 1.0, false, 0.0, false, false);
										MotionModule::set_frame(boma, MotionModule::frame(boma)-1.0, false);
									};
								};
							};
							if zair == 1 && WorkModule::get_param_int(boma, hash40("air_lasso_type"), 0) == 1 {
								println!("zair use");
								if jumping == 1 && status_kind !=  *FIGHTER_STATUS_KIND_JUMP && [*FIGHTER_KIND_NESS, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_MEWTWO].contains(&fighter_kind) == false {
									KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL);
								};
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_AIR_LASSO, true);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_100 && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100) {
								//Checks if the char has a rapid jab
								if WorkModule::get_param_int(boma, hash40("attack100_type"), 0) == 1 {
									StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_100, true);
									println!("Rapid jabbable");
								};
							};
							if next_status == *FIGHTER_STATUS_KIND_ITEM_THROW && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE)) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, true);
								if (X[STEP as usize] * PostureModule::lr(boma)) <= -0.5 {
									if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("item_light_throw_b"), 0.0, 1.0, false, 0.0, false, false);
									} else {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("item_light_throw_air_b"), 0.0, 1.0, false, 0.0, false, false);
									};
								} else if Y[STEP as usize] >= 0.5 {
									if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("item_light_throw_hi"), 0.0, 1.0, false, 0.0, false, false);
									} else {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("item_light_throw_air_hi"), 0.0, 1.0, false, 0.0, false, false);
									};
								} else if Y[STEP as usize] <= -0.5 {
									if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("item_light_throw_lw"), 0.0, 1.0, false, 0.0, false, false);
									} else {
										MotionModule::change_motion(boma, smash::phx::Hash40::new("item_light_throw_air_lw"), 0.0, 1.0, false, 0.0, false, false);
									};
								};
							};
							if next_status == *FIGHTER_STATUS_KIND_ITEM_THROW_DASH && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH)) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW_DASH, true);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_S3 && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
								PostureModule::set_lr(boma, LR[STEP as usize]);
								PostureModule::update_rot_y_lr(boma);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_HI3 && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
								PostureModule::set_lr(boma, LR[STEP as usize]);
								PostureModule::update_rot_y_lr(boma);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_LW3 && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
								PostureModule::set_lr(boma, LR[STEP as usize]);
								PostureModule::update_rot_y_lr(boma);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_S4 && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4, true);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && StopModule::is_hit(boma) == false {
									if status_kind != *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD {
										StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, true);
									};
									ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_S4_START && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
								PostureModule::set_lr(boma, LR[STEP as usize]);
								PostureModule::update_rot_y_lr(boma);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_HI4 && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4, true);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && StopModule::is_hit(boma) == false {
									if status_kind != *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD {
										StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, true);
									};
									ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_HI4_START && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
								PostureModule::set_lr(boma, LR[STEP as usize]);
								PostureModule::update_rot_y_lr(boma);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_LW4 && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_LW4_START && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
								PostureModule::set_lr(boma, LR[STEP as usize]);
								PostureModule::update_rot_y_lr(boma);
							};
							if next_status == *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && StopModule::is_hit(boma) == false {
									if status_kind != *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD {
										StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, true);
									};
									ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK);
							};
						};
						//Specials
						if specialing == 1 {
							if next_status == *FIGHTER_STATUS_KIND_SPECIAL_N && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
							};
							if next_status == *FIGHTER_STATUS_KIND_SPECIAL_S && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
							};
							if next_status == *FIGHTER_STATUS_KIND_SPECIAL_HI && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
							};
							if next_status == *FIGHTER_STATUS_KIND_SPECIAL_LW && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
								if [*FIGHTER_KIND_PEACH, *FIGHTER_KIND_DAISY].contains(&fighter_kind) == false || StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
									StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
								};
							};
						};
						//Defensive
						if next_status == *FIGHTER_STATUS_KIND_PASS && [*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_F].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_TREAD_JUMP && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TREAD_JUMP, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_THROW && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_THROW, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_THROW_KIRBY && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_THROW_KIRBY, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_CATCH_ATTACK && [*FIGHTER_STATUS_KIND_CATCH_DASH_PULL, *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCH_ATTACK].contains(&status_kind) && attacking == 1 && ATTACK[(STEP-1) as usize] == 0 {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_ATTACK, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_CATCH && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_CATCH_DASH && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_DASH, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_CATCH_TURN && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_TURN, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_PASSIVE && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
						};
						if shielding == 1 {
							if next_status == *FIGHTER_STATUS_KIND_ESCAPE && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE, true);
							};
							if next_status == *FIGHTER_STATUS_KIND_ESCAPE_AIR && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
							};
							if next_status == *FIGHTER_STATUS_KIND_ESCAPE_B && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_B, true);
							};
							if next_status == *FIGHTER_STATUS_KIND_ESCAPE_F && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_F, true);
							};
						};
					};
					//Shield Hold
					if shielding == 1 {
						ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_GUARD);
					} else if shielding == 0 {
						ControlModule_set_button_off(boma, *CONTROL_PAD_BUTTON_GUARD);
					};
					if specialing == 1 {
						ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL);
					};
					//Full hop forcer
					if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT && attacking == 0 {
						if jumping == 1 {
							ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_JUMP);
						} else {
							ControlModule_set_button_off(boma, *CONTROL_PAD_BUTTON_JUMP);
						};
					};
					if attacking == 1 {
						ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK);
					} else {
						ControlModule_set_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK);
					};
					//Fastfall
					if fastfalling == 1 {
						WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					} else {
						WorkModule::set_flag(boma, false, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					};
					//Throw Kind Setter
					if MotionModule::motion_kind(boma) == hash40("throw_f") {
							if throw_status == 1{
								MotionModule::change_motion(boma, smash::phx::Hash40::new("throw_b"), 0.0, 1.0, false, 0.0, false, false);
							} else if throw_status == 2{
								MotionModule::change_motion(boma, smash::phx::Hash40::new("throw_hi"), 0.0, 1.0, false, 0.0, false, false);
								if fighter_kind == *FIGHTER_KIND_LUIGI {
									ArticleModule::change_motion(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU,smash::phx::Hash40::new("throw_hi"),false,0.0);
								};
							} else if throw_status == 3{
								MotionModule::change_motion(boma, smash::phx::Hash40::new("throw_lw"), 0.0, 1.0, false, 0.0, false, false);
								if fighter_kind == *FIGHTER_KIND_LUIGI {
									ArticleModule::change_motion(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU,smash::phx::Hash40::new("throw_lw"),false,0.0);
								};
								if fighter_kind == *FIGHTER_KIND_BUDDY && MotionModule::frame(boma) > 33.0{
									WorkModule::on_flag(boma, *FIGHTER_BUDDY_STATUS_THROW_LW_FLAG_BURY);
								};
							};
					};
					//Handles landing aerial landing lag for not nair
					if MotionModule::motion_kind(boma) == hash40("landing_air_n") {
							if aerial_status == 1{
								println!("Fair landing");
								AttackModule::clear_all(boma);
								MotionModule::change_motion(boma, smash::phx::Hash40::new("landing_air_f"), 1.0, 1.0, false, 0.0, false, false);
								let fair_landing_mult = 1.0/(WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_f"), 0) / FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
								MotionModule::set_rate(boma, fair_landing_mult as f32);
							} else if aerial_status == 2{
								println!("Bair landing");
								AttackModule::clear_all(boma);
								MotionModule::change_motion(boma, smash::phx::Hash40::new("landing_air_b"), 1.0, 1.0, false, 0.0, false, false);
								let bair_landing_mult = 1.0/(WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_b"), 0) / FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
								MotionModule::set_rate(boma, bair_landing_mult as f32);
							} else if  aerial_status == 3{
								println!("Uair landing");
								AttackModule::clear_all(boma);
								MotionModule::change_motion(boma, smash::phx::Hash40::new("landing_air_f"), 1.0, 1.0, false, 0.0, false, false);
								let uair_landing_mult = 1.0/(WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_hi"), 0) / FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
								MotionModule::set_rate(boma, uair_landing_mult as f32);
							} else if  aerial_status == 4{
								println!("Dair landing");
								AttackModule::clear_all(boma);
								MotionModule::change_motion(boma, smash::phx::Hash40::new("landing_air_lw"), 1.0, 1.0, false, 0.0, false, false);
								let dair_landing_mult = 1.0/(WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_lw"), 0) / FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
								MotionModule::set_rate(boma, dair_landing_mult as f32);
							};
					};
					//Character Specific Fixes
					if [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_KROOL].contains(&fighter_kind){
						if WorkModule::is_flag(boma, *FIGHTER_BUDDY_STATUS_THROW_LW_FLAG_BURY) || WorkModule::is_flag(boma, *FIGHTER_KROOL_STATUS_THROW_LW_FLAG_BURY) {
							IS_BURY = true;
							WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CAPTURE_CUT);
						};
					};
					if fighter_kind == *FIGHTER_KIND_MARIO {
						if next_status == *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE && [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, true);
						};
						if next_status == *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT  {
							if [*FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_STATUS_KIND_ESCAPE].contains(&status_kind) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, true);
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_ROBOT {
						if next_status == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ITEM_THROW && [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ITEM_THROW, true);
						};
						if next_status == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ITEM_SHOOT_AIR && [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ITEM_SHOOT_AIR, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_WARIO {
						if next_status == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, true);
						};
						if next_status == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_APPEAL && [*FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DRIVE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_APPEAL, true);
						};
						if next_status == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START && [*FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DRIVE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_APPEAL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_ROCKMAN {
						if next_status == *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK)|| [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN].contains(&status_kind)) && attacking == 1  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, true);
						};
						if next_status == *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK)|| [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN].contains(&status_kind)) && attacking == 1  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, true);
						};
						if next_status == *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK)|| [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN].contains(&status_kind)) && attacking == 1  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN, true);
						};
						if next_status == *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK)|| [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN].contains(&status_kind)) && attacking == 1  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE, true);
						};
						if next_status == *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK)|| [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN].contains(&status_kind)) && attacking == 1  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_KAMUI {
						if next_status == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK && ([*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S))  {
							let kinetic = KineticModule::get_kinetic_type(boma);
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK, true);
							KineticModule::change_kinetic(boma, kinetic);
						};
						if next_status == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END && [*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END, true);
						};
						if next_status == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_JUMP && [*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_JUMP, true);
						};
						if next_status == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F && [*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F, true);
						};
						if next_status == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B && [*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_MURABITO {
						if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT, true);
						};
						if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_AIR].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_AIR, true);
						};
						if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT_FAIL].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT_FAIL, true);
						};
						if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WAIT].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WAIT, true);
						};
						if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP_SQUAT].contains(&next_status) && [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_B, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_F, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_DASH_B, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_DASH_F, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_BRAKE_B, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WALK_BRAKE_F].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP_SQUAT, true);
						};
						if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_DEFOREST].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_DEFOREST, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_WIIFIT {
						if next_status == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_BREATH && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_BREATH, true);
						};
						if next_status == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS && status_kind == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_BREATH {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS, true);
						};
						if next_status == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_END && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_END, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_KROOL {
						if next_status == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_THROW && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) && status_kind != FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_THROW && specialing == 1 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_THROW, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_SHULK {
						if next_status == *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && WorkModule::is_flag(boma, *FIGHTER_SHULK_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_ADD_SHIFT_INPUT) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_ROBOT {
						if next_status == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_RYU || fighter_kind == *FIGHTER_KIND_KEN  {
						if next_status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true);
						};
						if next_status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_SPECIAL_HI && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_SPECIAL_S && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_SPECIAL_N && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
						};
						if next_status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND && ((AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N)) && (attacking == 1 || specialing == 1) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND, true);
						};
						if next_status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND && ((AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N)) && (attacking == 1 || specialing == 1)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, true);
						};
						if next_status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND && ((AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S)) && (attacking == 1 || specialing == 1)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, true);
						};
						if next_status == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND && ((AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI)) && (attacking == 1 || specialing == 1)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_DOLLY  {
						if next_status == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) && status_kind != FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B && specialing == 1 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, true);
						};
						if next_status == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_SPECIAL_HI && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_SPECIAL_S && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_SPECIAL_N && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_SPECIAL_LW && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
						};
						if next_status == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL && ((AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL)) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S)) && (attacking == 1 || specialing == 1)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, true);
						};
						if next_status == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2 && ((AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL)) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S)) && (attacking == 1 || specialing == 1)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, true);
						};
						if next_status == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND && ((AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL)) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S)) && (attacking == 1 || specialing == 1)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, true);
						};
						if next_status == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND && ((AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL)) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S)) && (attacking == 1 || specialing == 1)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, true);
						};
						if next_status == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND && ((AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL)) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)) && (attacking == 1 || specialing == 1)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, true);
						};
						if next_status == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND && ((AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::is_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL)) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI)) && (attacking == 1 || specialing == 1)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_DEDEDE {
						if next_status == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE && [*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_HIT, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_TURN, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE, true);
						};
						if next_status == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_PASS && [*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WAIT, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WALK, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_TURN].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_PASS, true);
						};
						if next_status == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT && [*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WAIT, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WALK, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_TURN].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT, true);
						};
						if next_status == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SPIT && [*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WALK, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_JUMP1, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_JUMP2, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT_FALL, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT_JUMP].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SPIT, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_DIDDY {
						if next_status == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_FLIP_FALL, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_JUMP].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK, true);
						};
						if next_status == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK_JUMP && [*FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK_JUMP, true);
						};
						if next_status == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK_JUMP2 && [*FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK_JUMP2, true);
						};
						if next_status == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK_ATTACK && [*FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK_ATTACK, true);
						};
						if next_status == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK_ATTACK2 && [*FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_STICK_ATTACK2, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_SONIC {
						if next_status == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING && status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING, true);
						};
						if next_status == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP && status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_MIIFIGHTER {
						if next_status == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK && [*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) && [hash40("special_lw2_start"), hash40("special_air_lw2_start")].contains(&MotionModule::motion_kind(boma)) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK, true);
						};
						if next_status == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_3 && status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_2  && WorkModule::is_flag(boma, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_TENCHI_KICK_SHIFT){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_3, true);
						};
						if [*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_AIR].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)  {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND, true);
							}else if [*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_AIR].contains(&status_kind) == false {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_AIR, true);
							};
						};
						if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_AIR && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND && MotionModule::frame(boma) < 30.0 {
							MotionModule::set_frame(boma, 31.0, false);
						};
					};
					if fighter_kind == *FIGHTER_KIND_GAMEWATCH {
						if MotionModule::motion_kind(boma) == hash40("attack_air_b") && MotionModule::frame(boma) < 2.0 {
							WorkModule::set_int(boma, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_TURTLE, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
							ArticleModule::change_motion(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, smash::phx::Hash40::new("attack_air_b"), false, 0.0);
						};
						if MotionModule::motion_kind(boma) == hash40("landing_air_b") && MotionModule::frame(boma) < 2.0 {
							WorkModule::set_int(boma, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_TURTLE, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
							ArticleModule::change_motion(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, smash::phx::Hash40::new("landing_air_b"), false, 0.0);
						};
						if MotionModule::motion_kind(boma) == hash40("attack_air_f") && MotionModule::frame(boma) < 2.0 {
							ArticleModule::remove_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
						};
						if MotionModule::motion_kind(boma) == hash40("attack_air_hi") && MotionModule::frame(boma) < 2.0 {
							ArticleModule::remove_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
						};
						if MotionModule::motion_kind(boma) == hash40("attack_air_lw") && MotionModule::frame(boma) < 2.0 {
							ArticleModule::remove_exist(boma, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
						};
					};
					if fighter_kind == *FIGHTER_KIND_ELIGHT {
						if MotionModule::motion_kind(boma) == hash40("attack_air_f") && MotionModule::frame(boma) > 11.0 {
							MotionModule::set_rate(boma, 1.0);
						};
					};
					if fighter_kind == *FIGHTER_KIND_SHEIK {
						if next_status == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_LOOP && [*FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_LOOP, true);
						};
						if next_status == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_SHOOT  {
							if [*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_STATUS_KIND_ESCAPE].contains(&status_kind) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END, true);
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_PALUTENA {
						if dash >= 2 && dash < 20 {
							WorkModule::on_flag(boma, *FIGHTER_PALUTENA_STATUS_SPECIAL_S_FLAG_SMASH);
						};
					};
					if fighter_kind == *FIGHTER_KIND_INKLING {
						if next_status == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END && [*FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK_TURN, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN_TURN].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END, true);
						};
						if next_status == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_JUMP_END && [*FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK_TURN, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN_TURN].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_JUMP_END, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_PICKEL {
						if pickel_airhi == 1 {
							WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI);
						} else if pickel_airhi == 0 {
							WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI);
						};
						if pickel_hi3 == 1 {
							WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_HI3);
						} else if pickel_hi3 == 0 {
							WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_HI3);
						};
						if pickel_sh == 1 {
							WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_JUMP_MINI);
						} else if pickel_sh == 0 {
							WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_JUMP_MINI);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && specialing == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT, true);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && specialing == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK, true);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK_BACK && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && specialing == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK_BACK, true);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT && [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK_BACK].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT, true);
						};
						if status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT {
							if jumping == 1 {
								ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_JUMP);
							} else {
								ControlModule_set_button_off(boma, *CONTROL_PAD_BUTTON_JUMP);
							};
						};
						
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT, true);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK, true);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK, true);
						};
						if  next_status == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							if status_kind == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_SQUAT {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP, true);
							} else {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL, true);
							};
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL, true);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL, true);
						};
						
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_START && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_START, true);
						};
						if status_kind != *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL && next_status == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_AERIAL && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL, true);
						};
						
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_RIDE && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) && status_kind != *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_RIDE {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_RIDE, true);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_JUMP && status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_DRIVE {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_JUMP, true);
						};
						
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_JUMP && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_JUMP, true);
							WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AFTER_USE);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_WAIT && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_WAIT, true);
						};
						
						
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_LW_BOMB && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_LW_BOMB, true);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_LW_WAIT && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_LW_WAIT, true);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_LW_WALK && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_LW_WALK, true);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_LW_PLATE && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) || MotionModule::frame(boma) as i32 >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && attacking == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_LW_PLATE, true);
						};
						
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N2 && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N2, true);
						};
						if next_status == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_SQUAT && [*FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_SQUAT, true);
						};
						if status_kind == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_SQUAT {
							if jumping == 1 {
								ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_JUMP);
							} else {
								ControlModule_set_button_off(boma, *CONTROL_PAD_BUTTON_JUMP);
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_DONKEY {
						if next_status == *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_LOOP && [*FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_LOOP, true);
						};
						if next_status == *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END  {
							if [*FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_STATUS_KIND_ESCAPE].contains(&status_kind) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END, true);
							};
						};
						if next_status == *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START, true);
						};
						if next_status == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_F && [*FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_TURN, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WALK, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_LANDING, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_FALL].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DONKEY_STATUS_KIND_THROW_F_F, true);
						};
						if next_status == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_B && [*FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_TURN, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WALK, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_LANDING, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_FALL].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DONKEY_STATUS_KIND_THROW_F_B, true);
						};
						if next_status == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_HI && [*FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_TURN, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WALK, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_LANDING, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_FALL].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DONKEY_STATUS_KIND_THROW_F_HI, true);
						};
						if next_status == *FIGHTER_DONKEY_STATUS_KIND_THROW_F_LW && [*FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_TURN, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WALK, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_LANDING, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_FALL].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DONKEY_STATUS_KIND_THROW_F_LW, true);
						};
						if next_status == *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT && [*FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_TURN, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WALK].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT, true);
						};
						if next_status == *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT_B && [*FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_TURN, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WALK].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT_B, true);
						};
						if status_kind == *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT_B || status_kind == *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT {
							if jumping == 1 {
								ControlModule_set_button_on(boma, *CONTROL_PAD_BUTTON_JUMP);
							} else {
								ControlModule_set_button_off(boma, *CONTROL_PAD_BUTTON_JUMP);
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_FOX {
						if STEP > 0 {
							if WorkModule::is_flag(boma, *FIGHTER_FOX_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT) && specialing == 1 && SPECIAL[(STEP-1) as usize] == 0 {
								if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
									MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n_loop"), 0.0, 1.0, false, 0.0, false, false);
								} else {
									MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n_loop"), 0.0, 1.0, false, 0.0, false, false);
								};
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_MIIGUNNER {
						if next_status == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_START  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_START, true);
						};
						if next_status == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_HOLD && shielding == 0  {
							if status_kind == *FIGHTER_STATUS_KIND_ESCAPE &&  StatusModule::prev_status_kind(boma, 0) != *FIGHTER_STATUS_KIND_GUARD {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_HOLD, true);
							};
						};
						if next_status == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE  {
							if [*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE, *FIGHTER_STATUS_KIND_ESCAPE].contains(&status_kind) || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE, true);
							};
						};
						if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_GROUND].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) && specialing == 1 {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_GROUND, true);
							} else {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR, true);
							};
						};
						if [*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) && specialing == 1 {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND, true);
							} else {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR, true);
							};
						};
						if STEP > 0 {
							if [hash40("special_n2_loop"), hash40("special_air_n2_loop")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) > 10.0 && specialing == 1 && SPECIAL[(STEP-1) as usize] == 0 {
								if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
									MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n2_loop"), 0.0, 1.0, false, 0.0, false, false);
								} else {
									MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n2_loop"), 0.0, 1.0, false, 0.0, false, false);
								};
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_FALCO {
						if STEP > 0 {
							if [hash40("special_n_loop"), hash40("special_air_n_loop")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) > 23.0 && specialing == 1 && SPECIAL[(STEP-1) as usize] == 0 {
								if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
									MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n_loop"), 0.0, 1.0, false, 0.0, false, false);
								} else {
									MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n_loop"), 0.0, 1.0, false, 0.0, false, false);
								};
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_KIRBY {
						if next_status == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP_SQUAT{
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP_SQUAT, true);
						};
						if next_status == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT && [*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_WALK, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_JUMP1, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_JUMP2, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_WAIT_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_WAIT_JUMP].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT, true);
						};
						if next_status == *FIGHTER_STATUS_KIND_SPECIAL_LW && false == true {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP_SQUAT, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_YOSHI {
						if [*FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_LOOP].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_LOOP, true);
						};
						if [*FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END].contains(&next_status) && [*FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_TURN].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END, true);
						};
						if [*FIGHTER_YOSHI_STATUS_KIND_SPECIAL_AIR_S_JUMP].contains(&next_status) && [*FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_TURN].contains(&status_kind)  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_AIR_S_JUMP, true);
						};
						if [*FIGHTER_YOSHI_STATUS_KIND_SPECIAL_AIR_LW].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)  {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_LW_TO_AIR, true);
							}else if [*FIGHTER_YOSHI_STATUS_KIND_SPECIAL_LW_TO_AIR, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_AIR_LW].contains(&status_kind) == false {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_AIR_LW, true);
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_KOOPA {
						if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI)  {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G, true);
							} else {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, true);
							};
						};
						if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)  {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G, true);
							}else if [*FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A].contains(&status_kind) == false {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A, true);
							};
						};
						if status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G && MotionModule::frame(boma) < 30.0 {
							MotionModule::set_frame(boma, 31.0, false);
						};
					};
					if fighter_kind == *FIGHTER_KIND_LINK {
						if next_status == *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_LINK || fighter_kind == *FIGHTER_KIND_YOUNGLINK {
						if STEP > 0 {
							if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && attacking == 1 && ATTACK[(STEP-1) as usize] == 0 && status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4   {
								MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);
							};
						};
						if MotionModule::motion_kind(boma) == hash40("attack_air_lw") && WorkModule::is_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK) && WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) == false && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
							WorkModule::on_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_BOUND);
							println!("bounce");
						};
					};
					if fighter_kind == *FIGHTER_KIND_BRAVE {
						if STEP > 0 {
							if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && attacking == 1 && ATTACK[(STEP-1) as usize] == 0 && status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3  {
								MotionModule::set_rate(boma, 1.0);
								MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_s3_s2"), 0.0, 1.0, false, 0.0, false, false);
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_BAYONETTA {
						if (next_status == *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F || (next_status == *FIGHTER_STATUS_KIND_ATTACK_AIR && aerial_status == 1)) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F, true);
						};
						if STEP > 0 {
							if WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO) && attacking == 1 && ATTACK[(STEP-1) as usize] == 0 && MotionModule::motion_kind(boma) == hash40("attack_air_f2")  {
								MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_f3"), 0.0, 1.0, false, 0.0, false, false);
							};
							if WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO) && attacking == 1 && ATTACK[(STEP-1) as usize] == 0 && MotionModule::motion_kind(boma) == hash40("attack_air_f")  {
								MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_f2"), 0.0, 1.0, false, 0.0, false, false);
							};
						};
						if specialing == 1 {
							if next_status == *FIGHTER_STATUS_KIND_SPECIAL_S && StatusModule::situation_kind(boma) == SITUATION_KIND_AIR && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S){
								StatusModule::change_status_request_from_script(boma, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, true);
							};
							if next_status == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S){
								StatusModule::change_status_request_from_script(boma, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, true);
							};
							if next_status == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S){
								StatusModule::change_status_request_from_script(boma, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, true);
							};
						};
						if STEP > 0 {
							if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && attacking == 1 && ATTACK[(STEP-1) as usize] == 0 && MotionModule::motion_kind(boma) == hash40("attack_s3_s2")  {
								MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_s3_s3"), 0.0, 1.0, false, 0.0, false, false);
							};
							if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && attacking == 1 && ATTACK[(STEP-1) as usize] == 0 && MotionModule::motion_kind(boma) == hash40("attack_s3_s")  {
								MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_s3_s2"), 0.0, 1.0, false, 0.0, false, false);
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_SAMUS {
						if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) && specialing == 1 {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, true);
							} else {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, true);
							};
						};
						if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) && specialing == 1 {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, true);
							} else {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, true);
							};
						};
						if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) && specialing == 1 {
							if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, true);
							} else {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, true);
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_LITTLEMAC {
						if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_START].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_START, true);
						};
						if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START, true);
						};
						if [*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH].contains(&next_status) && status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH, true);
						};
						if [*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH_TURN].contains(&next_status) && status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH_TURN, true);
						};
						if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) && WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE) >= 5.0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, true);
						};
						if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP].contains(&next_status) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_SZEROSUIT {
						if [hash40("attack_air_b"), hash40("attack_air_f"), hash40("attack_air_hi"), hash40("attack_air_lw"), hash40("landing_air_b"), hash40("landing_air_f"), hash40("landing_air_hi"), hash40("landing_air_lw")].contains(&MotionModule::motion_kind(boma))  {
							ArticleModule::remove_exist(boma, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
						};
					};
					if fighter_kind == *FIGHTER_KIND_PEACH || fighter_kind == *FIGHTER_KIND_DAISY {
						if paisyfloat == 1 {
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_PEACH_UNIQ_FLOAT);
							if aerial_status != -1 {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
							};
						} else {
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
						};
					};
					if fighter_kind == *FIGHTER_KIND_MASTER {
						if [hash40("attack_air_b"), hash40("attack_air_f"), hash40("attack_air_hi"), hash40("attack_air_lw"), hash40("landing_air_b"), hash40("landing_air_f"), hash40("landing_air_hi"), hash40("landing_air_lw")].contains(&MotionModule::motion_kind(boma))  {
							ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
						};
						if [hash40("landing_air_b"), hash40("landing_air_f"), hash40("landing_air_hi"), hash40("landing_air_lw")].contains(&MotionModule::motion_kind(boma))  {
							ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
							ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
							ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
						};
						if next_status == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT_DASH && status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT_DASH, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_KOOPAJR {
						if next_status == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP && status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP, true);
						};
						if next_status == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP && status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_SHIZUE {
						if [hash40("attack_air_b"), hash40("attack_air_f"), hash40("attack_air_hi"), hash40("attack_air_lw"), hash40("landing_air_b"), hash40("landing_air_f"), hash40("landing_air_hi"), hash40("landing_air_lw")].contains(&MotionModule::motion_kind(boma))  {
							ArticleModule::remove_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_POMPON, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
						};
					};
					if fighter_kind == *FIGHTER_KIND_SNAKE {
						if next_status == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_PRODUCE && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_PRODUCE, true);
						};
						if next_status == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_S_AWAY_END && status_kind == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_S_OPERATION {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_S_AWAY_END, true);
						};
						if STEP > 0 {
							if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && attacking == 1 && ATTACK[(STEP-1) as usize] == 0 && status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
								MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_s3_s2"), 0.0, 1.0, false, 0.0, false, false);
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_ZELDA {
						if zelda_phantom == 1 {
							ArticleModule::change_status(boma, *WEAPON_KIND_ZELDA_PHANTOM, *WEAPON_ZELDA_PHANTOM_STATUS_KIND_ATTACK, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
							println!("phantom attack yay!");
						};
					};
					if fighter_kind == *FIGHTER_KIND_CLOUD {
						if next_status == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3 && status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3, true);
						};
						if next_status == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2 && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, true);
						};
						if next_status == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2 && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && WorkModule::is_flag(boma, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2, true);
						};
					};
					if [*FIGHTER_KIND_MARTH, *FIGHTER_KIND_LUCINA].contains(&fighter_kind) {
						if next_status == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2 && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, true);
						} else if next_status == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3 && status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, true);
						} else if next_status == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4 && status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, true);
						};
						if [hash40("special_s1"), hash40("special_air_s1"), hash40("special_s2_lw"), hash40("special_air_s2_lw"), hash40("special_s3_s"), hash40("special_air_s3_s"), hash40("special_s4_s"), hash40("special_air_s4_s")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) < 2.0 {
							if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2 {
								if Y[STEP as usize] >= 0.5 {
									if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s2_hi"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									} else {
										println!("WORKS");
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_air_s2_hi"), 0.0, 1.0, 0.0, false, false);
									};
								};
							};
							if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3 {
								if Y[STEP as usize] >= 0.5 {
									if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s3_hi"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									} else {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_air_s3_hi"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									};
								} else if Y[STEP as usize] <= -0.5 {
									if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s3_lw"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									} else {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_air_s3_lw"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									};
								};
							};
							if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4 {
								if Y[STEP as usize] >= 0.5 {
									if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s4_hi"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									} else {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_air_s4_hi"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									};
								} else if Y[STEP as usize] <= -0.5{
									if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s4_lw"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									} else {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_air_s4_lw"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									};
								};
							};
						};
					};
					if [*FIGHTER_KIND_ROY, *FIGHTER_KIND_CHROM].contains(&fighter_kind) {
						if next_status == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2 && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, true);
						} else if next_status == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3 && status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, true);
						} else if next_status == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4 && status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, true);
						};
						if [hash40("special_s1"), hash40("special_air_s1"), hash40("special_s2_lw"), hash40("special_air_s2_lw"), hash40("special_s3_s"), hash40("special_air_s3_s"), hash40("special_s4_s"), hash40("special_air_s4_s")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) < 2.0 {
							if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2 {
								if Y[STEP as usize] >= 0.5 {
									if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s2_hi"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									} else {
										println!("WORKS");
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_air_s2_hi"), 0.0, 1.0, 0.0, false, false);
									};
								};
							};
							if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3 {
								if Y[STEP as usize] >= 0.5 {
									if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s3_hi"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									} else {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_air_s3_hi"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									};
								} else if Y[STEP as usize] <= -0.5 {
									if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s3_lw"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									} else {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_air_s3_lw"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									};
								};
							};
							if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4 {
								if Y[STEP as usize] >= 0.5 {
									if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s4_hi"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									} else {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_air_s4_hi"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									};
								} else if Y[STEP as usize] <= -0.5{
									if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s4_lw"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									} else {
										MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_air_s4_lw"), 0.0, 1.0, 0.0, false, false);
										println!("WORKS");
									};
								};
							};
						};
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
					THROW_KIND.clear();
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
				if IS_RECORD == true  && fighter_kind != *FIGHTER_KIND_NANA {
					MAX += 1;
					STATUS.push(status_kind);
					if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR ||  status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR{
						if MotionModule::motion_kind(boma) == hash40("attack_air_f") || MotionModule::motion_kind(boma) == hash40("landing_air_f"){
							AERIAL_KIND.push(1);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_b") || MotionModule::motion_kind(boma) == hash40("landing_air_b") {
							AERIAL_KIND.push(2);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_hi") || MotionModule::motion_kind(boma) == hash40("landing_air_hi") {
							AERIAL_KIND.push(3);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_lw") || MotionModule::motion_kind(boma) == hash40("landing_air_lw") {
							AERIAL_KIND.push(4);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_f_hi"){
							AERIAL_KIND.push(5);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_f_lw"){
							AERIAL_KIND.push(6);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_b_hi"){
							AERIAL_KIND.push(7);
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_b_lw"){
							AERIAL_KIND.push(8);
						} else {
							AERIAL_KIND.push(0);
						};
					} else {
						AERIAL_KIND.push(-1);
					};
					if status_kind == *FIGHTER_STATUS_KIND_THROW || status_kind == *FIGHTER_STATUS_KIND_THROW_KIRBY {
						if MotionModule::motion_kind(boma) == hash40("throw_f"){
							THROW_KIND.push(0);
						} else if MotionModule::motion_kind(boma) == hash40("throw_b"){
							THROW_KIND.push(1);
						} else if MotionModule::motion_kind(boma) == hash40("throw_hi"){
							THROW_KIND.push(2);
						} else {
							THROW_KIND.push(3);
						};
					} else {
						THROW_KIND.push(-1);
					};
					if MotionModule::frame(boma) == 1.0 {
						IS_CHANGING.push(1);
					} else {
						IS_CHANGING.push(0);
					};
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
					if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_PEACH_UNIQ_FLOAT {
						PAISYFLOAT.push(1);
					} else {
						PAISYFLOAT.push(0);
					};
					if fighter_kind == *FIGHTER_KIND_ZELDA && ArticleModule::is_exist(boma, *WEAPON_KIND_ZELDA_PHANTOM) {
						if [hash40("attack_kick"), hash40("attack_punch"), hash40("attack_s"), hash40("attack_l"), hash40("attack_max")].contains(&ArticleModule::motion_kind(boma, *WEAPON_KIND_ZELDA_PHANTOM, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)))  {
							PHANTOM.push(1);
							println!("zelda phantom attack");
						} else {
							PHANTOM.push(0);
						};
					} else {
						PHANTOM.push(-1);
					};
					if status_kind == *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_LW_END {
						println!("DOWNB END");
					};
					if fighter_kind == *FIGHTER_KIND_PICKEL {
						if WorkModule::is_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_HI3) {
							PICKEL_HI3.push(1);
						} else {
							PICKEL_HI3.push(0);
						};
						if WorkModule::is_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI) {
							PICKEL_AIRHI.push(1);
						} else {
							PICKEL_AIRHI.push(0);
						};
						if WorkModule::is_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_JUMP_MINI) {
							PICKEL_SH.push(1);
						} else {
							PICKEL_SH.push(0);
						};
					} else {
						PICKEL_HI3.push(-1);
						PICKEL_AIRHI.push(-1);
						PICKEL_SH.push(-1);
					};
					X.push(ControlModule::get_stick_x(boma)*-1.0);
					Y.push(ControlModule::get_stick_y(boma));
					LR.push(PostureModule::lr(boma)*-1.0);
					DASHRECORDS.push(ControlModule::get_flick_x(boma));
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						println!("status_kind: {}", status_kind);
					};
					let cat2 = ControlModule::get_command_flag_cat(boma, 1);
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_AIR_LASSO) != 0 {
						ZAIR.push(1);
						println!("zair");
					} else {
						ZAIR.push(0);
					};
					let cat3 = ControlModule::get_command_flag_cat(boma, 2);
					if (cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_DROP_AIR) != 0 {
						ZDROP.push(1);
						println!("zdrop!?");
					} else {
						ZDROP.push(0);
					};
					let cat3 = ControlModule::get_command_flag_cat(boma, 2);
					if (cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_GET_AIR) != 0 {
						ZGET.push(1);
						println!("zget!?");
					} else {
						ZGET.push(0);
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && status_kind != *FIGHTER_STATUS_KIND_JUMP {
						println!("dj status! : {}. Kinetic type: {}", status_kind, KineticModule::get_kinetic_type(boma));
					};
					//println!("X: {}, Y: {}, Status: {}, Aerial Kind: {}, Is Jump: {}, Is Attack: {}, Is Shield: {}, Dash Flick: {}", ControlModule::get_stick_x(boma)*-1.0, ControlModule::get_stick_y(boma), status_kind, AERIAL_KIND[(MAX-1) as usize], JUMP[(MAX-1) as usize], ATTACK[(MAX-1) as usize], GUARD[(MAX-1) as usize], DASHRECORDS[(MAX-1) as usize]);
				};
			};
		};
    }
}

pub fn install() {
    acmd::add_custom_hooks!(recorder);
    acmd::add_custom_hooks!(repeater);
}