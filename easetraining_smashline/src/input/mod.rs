use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use smash::hash40;
use smash::params::fighter_param::*;
use std::os::raw::c_int;

static mut STATUS_DUR : [i32; 8] = [0; 8];

static mut IS_RECORD : bool = false;
static mut IS_PLAY : bool = false;
static mut STATUS : Vec<i32> = vec![];
static mut AERIAL_KIND : Vec<i32> = vec![];
static mut FASTFALL : Vec<i32> = vec![];
static mut DASHRECORDS : Vec<i32> = vec![];
static mut X : Vec<f32> = vec![];
static mut Y : Vec<f32> = vec![];
static mut SUB_X : Vec<f32> = vec![];
static mut SUB_Y : Vec<f32> = vec![];
static mut FLICK_X : Vec<i32> = vec![];
static mut FLICK_Y : Vec<i32> = vec![];
static mut FLICK_X_DIR : Vec<i32> = vec![];
static mut FLICK_Y_DIR : Vec<i32> = vec![];
static mut STICK_DIR : Vec<f32> = vec![];
static mut BELMONT_ANGLE : Vec<i32> = vec![];
static mut LR : Vec<f32> = vec![];
static mut COMMAND_FLAG0 : Vec<i32> = vec![];
static mut COMMAND_FLAG1 : Vec<i32> = vec![];
static mut COMMAND_FLAG2 : Vec<i32> = vec![];
static mut COMMAND_FLAG3 : Vec<i32> = vec![];
static mut STATUS_DURATION : Vec<i32> = vec![];
static mut EXPECTED_SPEED_X : Vec<f32> = vec![];
static mut EXPECTED_SPEED_Y : Vec<f32> = vec![];
static mut EXPECTED_X_POS : Vec<f32> = vec![];
static mut EXPECTED_Y_POS : Vec<f32> = vec![];
static mut MAX : i32 = 0;
static mut STEP : i32 = 0;

//Buttons
static mut ATTACK : Vec<i32> = vec![];
static mut SPECIAL : Vec<i32> = vec![];
static mut GUARD : Vec<i32> = vec![];
static mut JUMP : Vec<i32> = vec![];
static mut FLICK_JUMP : Vec<i32> = vec![];
static mut JUMP_MINI : Vec<i32> = vec![];
static mut CSTICK_ON : Vec<i32> = vec![];
static mut CATCH : Vec<i32> = vec![];


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
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_stick_y)]
pub unsafe fn handle_get_y(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> f32 {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if IS_PLAY == true && STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
			return Y[STEP as usize];
		} else {
			original!()(boma)
		}
	} else {
		original!()(boma)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::set_main_stick_x)]
pub unsafe fn handle_get_main_stick_x(
    boma: &mut smash::app::BattleObjectModuleAccessor,
	float: f32
) -> () {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if IS_PLAY == true && STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
			original!()(boma, X[STEP as usize])
		} else {
			original!()(boma, float)
		}
	} else {
		original!()(boma, float)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::set_main_stick_y)]
pub unsafe fn handle_get_main_stick_y(
    boma: &mut smash::app::BattleObjectModuleAccessor,
	float: f32
) -> () {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if IS_PLAY == true && STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
			println!("Running!");
			original!()(boma, Y[STEP as usize])
		} else {
			original!()(boma, float)
		}
	} else {
		original!()(boma, float)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_stick_x)]
pub unsafe fn handle_get_x(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> f32 {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if IS_PLAY == true && STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
			return X[STEP as usize];
		} else {
			original!()(boma)
		}
	} else {
		original!()(boma)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_sub_stick_y)]
pub unsafe fn handle_get_sub_y(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> f32 {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if IS_PLAY == true && STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
			return SUB_Y[STEP as usize];
		} else {
			original!()(boma)
		}
	} else {
		original!()(boma)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_flick_x)]
pub unsafe fn handle_get_flick_x(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> i32 {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if IS_PLAY == true && STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
			return FLICK_X[STEP as usize];
		} else {
			original!()(boma)
		}
	} else {
		original!()(boma)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_flick_x_dir)]
pub unsafe fn handle_get_flick_x_dir(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> i32 {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if IS_PLAY == true && STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
			return FLICK_X_DIR[STEP as usize];
		} else {
			original!()(boma)
		}
	} else {
		original!()(boma)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_flick_y)]
pub unsafe fn handle_get_flick_y(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> i32 {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if IS_PLAY == true && STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
			return FLICK_Y[STEP as usize];
		} else {
			original!()(boma)
		}
	} else {
		original!()(boma)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_flick_y_dir)]
pub unsafe fn handle_get_flick_y_dir(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> i32 {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
			//println!("new");
			return FLICK_Y_DIR[STEP as usize];
		} else {
			original!()(boma)
		}
	}else {
		original!()(boma)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_sub_stick_x)]
pub unsafe fn handle_get_sub_x(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> f32 {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
			return SUB_X[STEP as usize];
		} else {
			original!()(boma)
		}
	} else {
		original!()(boma)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_sub_stick_prev_x)]
pub unsafe fn handle_get_sub_x_prev(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> f32 {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true && STEP > 0{
			return SUB_X[(STEP-1) as usize];
		} else {
			original!()(boma)
		}
	} else {
		original!()(boma)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_sub_stick_prev_y)]
pub unsafe fn handle_get_sub_y_prev(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> f32 {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true && STEP > 0{
			return SUB_Y[(STEP-1) as usize];
		} else {
			original!()(boma)
		}
	} else {
		original!()(boma)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_attack_air_kind)]
pub unsafe fn handle_get_attack_air_kind(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> i32 {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true && AERIAL_KIND[STEP as usize] != 0{
			return AERIAL_KIND[STEP as usize];
		} else {
			original!()(boma)
		}
	} else {
		original!()(boma)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::set_attack_air_kind)]
pub unsafe fn handle_set_attack_air_kind(
    boma: &mut smash::app::BattleObjectModuleAccessor, 
	int: i32) {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true && AERIAL_KIND[STEP as usize] != 0{
			original!()(boma, AERIAL_KIND[STEP as usize])
		} else {
			original!()(boma, int)
		}
	} else {
		original!()(boma, int)
	}
}	
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::check_button_off)]
pub unsafe fn check_button_off_hook(boma: &mut smash::app::BattleObjectModuleAccessor, int: i32) -> bool {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
			if int == *CONTROL_PAD_BUTTON_ATTACK {
				if ATTACK[STEP as usize] == 0 {
					return true;
				} else {
					return false;
				}
			}else if int == *CONTROL_PAD_BUTTON_CATCH {
				if CATCH[STEP as usize] == 0 {
					return true;
				} else {
					return false;
				}
			}else if int == *CONTROL_PAD_BUTTON_GUARD {
				if GUARD[STEP as usize] == 0 {
					return true;
				} else {
					return false;
				}
			} else if int == *CONTROL_PAD_BUTTON_SPECIAL {
				if SPECIAL[STEP as usize] == 0 {
					if STEP > 1 {
						if status_kind == *FIGHTER_STATUS_KIND_JUMP && (SPECIAL[(STEP-1) as usize] == 1 || SPECIAL[(STEP-2) as usize] == 1) {
							return false;
						} else {
							return true;
						};
					} else {
						return true;
					};
				} else {
					return false;
				}
			} else if int == *CONTROL_PAD_BUTTON_JUMP {
				if JUMP[STEP as usize] == 0 || (status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT && ATTACK[STEP as usize] == 1) {
					return true;
				} else {
					return false;
				}
			} else if int == *CONTROL_PAD_BUTTON_JUMP_MINI {
				if JUMP_MINI[STEP as usize] == 0 {
					return true;
				} else {
					return false;
				}
			} else if int == *CONTROL_PAD_BUTTON_FLICK_JUMP {
				if FLICK_JUMP[STEP as usize] == 0 {
					return true;
				} else {
					return false;
				}
			}else if int == *CONTROL_PAD_BUTTON_CSTICK_ON {
				if CSTICK_ON[STEP as usize] == 0 {
					return true;
				} else {
					return false;
				}
			}else {
				original!()(boma, int)
			}
		} else {
			original!()(boma, int)
		}
	} else {
		original!()(boma, int)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::check_button_on)]
pub unsafe fn check_button_on_hook(boma: &mut smash::app::BattleObjectModuleAccessor, int: i32) -> bool {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if STEP >= MAX {
			STEP = 0;
			IS_PLAY = false;
		};
		if STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
			if int == *CONTROL_PAD_BUTTON_ATTACK {
				if ATTACK[STEP as usize] == 1 {
					return true;
				} else {
					return false;
				}
			}else if int == *CONTROL_PAD_BUTTON_CATCH {
				if CATCH[STEP as usize] == 1 {
					return true;
				} else {
					return false;
				}
			}else if int == *CONTROL_PAD_BUTTON_GUARD {
				if GUARD[STEP as usize] == 1 {
					return true;
				} else {
					return false;
				}
			} else if int == *CONTROL_PAD_BUTTON_SPECIAL {
				if SPECIAL[STEP as usize] == 1 {
					return true;
				} else {
					if STEP > 1 {
						if status_kind == *FIGHTER_STATUS_KIND_JUMP && (SPECIAL[(STEP-1) as usize] == 1 || SPECIAL[(STEP-2) as usize] == 1) {
							return true;
						} else {
							return false;
						};
					} else {
						return false;
					};
				}
			} else if int == *CONTROL_PAD_BUTTON_JUMP || int == *CONTROL_PAD_BUTTON_FLICK_JUMP{
				if JUMP[STEP as usize] == 0 || (status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT && ATTACK[STEP as usize] == 1){
					return false;
				} else {
					return true;
				}
			} else if int == *CONTROL_PAD_BUTTON_JUMP_MINI {
				if JUMP_MINI[STEP as usize] == 1 {
					return true;
				} else {
					return false;
				}
			} else if int == *CONTROL_PAD_BUTTON_CSTICK_ON {
				if CSTICK_ON[STEP as usize] == 1 {
					return true;
				} else {
					return false;
				}
			}else {
				original!()(boma, int)
			}
		} else {
			original!()(boma, int)
		}
	} else {
		original!()(boma, int)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_command_flag_cat)]
pub unsafe fn command_flag_cat_hook(boma: &mut smash::app::BattleObjectModuleAccessor, int: c_int) -> i32 {
	if IS_PLAY == true && smash::app::smashball::is_training_mode() == true {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if STEP < MAX && ENTRY_ID == 1 && smash::app::smashball::is_training_mode() == true {
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
	} else {
		original!()(boma, int)
	}
}
#[fighter_frame_callback]
pub fn repeater(fighter : &mut L2CFighterCommon) {
    unsafe {
		if smash::app::smashball::is_training_mode() == true {
			let lua_state = fighter.lua_state_agent;
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
			let fighter_kind = smash::app::utility::get_kind(boma);
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_num = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32;
			let frame = MotionModule::frame(boma);
			if smash::app::sv_information::is_ready_go() == false {
				STEP = 0;
				IS_PLAY = false;
			};
			if fighter_num == 1 && IS_PLAY == true  {
				if IS_PLAY == true {
					if STEP >= MAX {
						STEP = 0;
						IS_PLAY = false;
					};
					/*let STATUS[STEP as usize] = STATUS[STEP as usize];
					if STATUS[STEP as usize] == *FIGHTER_STATUS_KIND_TURN_DASH && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) && (X[STEP as usize] * PostureModule::lr(boma)) <= -0.85  {
						if status_kind != *FIGHTER_STATUS_KIND_TURN_DASH || (DASHRECORDS[STEP as usize] >= 2 && MotionModule::frame(boma) > 5.0 && DASHRECORDS[STEP as usize] < 20) {
							StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_TURN_DASH, true);
						};
					};*/
					if [*FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&STATUS[STEP as usize]) && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) || status_kind == *FIGHTER_STATUS_KIND_DASH || frame >= cancel_frame) && (X[STEP as usize] * PostureModule::lr(boma)) <= -0.85{
						if FLICK_X[STEP as usize] >= 2 &&  FLICK_X[STEP as usize] < 20{
							if fighter_kind == *FIGHTER_KIND_DOLLY {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, true);
							}else if fighter_kind == *FIGHTER_KIND_RYU {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, true);
							}else if fighter_kind == *FIGHTER_KIND_KEN {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, true);
							}else if fighter_kind == *FIGHTER_KIND_DEMON {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK, true);
							} /*else {
								PostureModule::reverse_lr(boma);
								PostureModule::update_rot_y_lr(boma);
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
							};*/
						};
					};
					if STATUS[STEP as usize] == *FIGHTER_STATUS_KIND_TURN_DASH && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) {
						StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_TURN_DASH, true);
					};
					if STATUS[STEP as usize] == *FIGHTER_STATUS_KIND_DASH && status_kind == *FIGHTER_STATUS_KIND_TURN_DASH{
						StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_DASH, true);
					};
					/*if PostureModule::lr(boma) != LR[STEP as usize] {
						PostureModule::set_lr(boma, LR[STEP as usize]);
						PostureModule::update_rot_y_lr(boma);
					};*/
					if STATUS[STEP as usize] == *FIGHTER_STATUS_KIND_ATTACK_100 && status_kind == *FIGHTER_STATUS_KIND_ATTACK {
						StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ATTACK_100, true);
					};
					if STEP > 1 {
						if STATUS[STEP as usize] == *FIGHTER_STATUS_KIND_CATCH_ATTACK && ([*FIGHTER_STATUS_KIND_CATCH_DASH_PULL, *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_WAIT].contains(&status_kind) || (frame > cancel_frame && status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)) && ATTACK[STEP as usize] == 1 && ATTACK[(STEP-1) as usize] == 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_ATTACK, true);
						};
					};
					if STATUS[STEP as usize] == *FIGHTER_STATUS_KIND_ATTACK_DASH {
						if status_kind != *FIGHTER_STATUS_KIND_ATTACK_DASH && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN].contains(&status_kind){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
						};
					};
					if STATUS[STEP as usize] == *FIGHTER_STATUS_KIND_CATCH_DASH {
						if status_kind != *FIGHTER_STATUS_KIND_CATCH_DASH && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_DASH, true);
						};
					};
					if [*FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&STATUS[STEP as usize]) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
						StatusModule::change_status_request_from_script(boma, STATUS[STEP as usize], true);
					};
					if STATUS[STEP as usize] == *FIGHTER_STATUS_KIND_ATTACK_AIR && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
					};
					if fighter_kind == *FIGHTER_KIND_BAYONETTA {
						if (STATUS[STEP as usize] == *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F || AERIAL_KIND[STEP as usize] == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F )&& (status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || (status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F && WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO) && ATTACK[STEP as usize]-ATTACK[(STEP-1) as usize] == 1)) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_TRAIL {
						if (STATUS[STEP as usize] == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F || AERIAL_KIND[STEP as usize] == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F) && (status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N ||  status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || (status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F && WorkModule::is_flag(boma, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO) && ATTACK[STEP as usize]-ATTACK[(STEP-1) as usize] == 1)) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || (status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N && WorkModule::is_flag(boma, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_ENABLE_COMBO) && ATTACK[STEP as usize]-ATTACK[(STEP-1) as usize] == 1)) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, true);
						};
					};
					if GroundModule::is_passable_ground(fighter.module_accessor) && STATUS[STEP as usize] == *FIGHTER_STATUS_KIND_PASS {
						if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3) {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_PICKEL {
						let mut pickel_airhi = 0;
						let mut pickel_hi3 = 0;
						if Y[STEP as usize] >= 0.25 || SUB_Y[STEP as usize] >= 0.25{
							pickel_airhi = 1;
							pickel_hi3 = 1;
						};
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
						/*if pickel_sh == 1 {
							WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_JUMP_MINI);
						} else if pickel_sh == 0 {
							WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_JUMP_MINI);
						};*/
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) || MotionModule::frame(boma) >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && SPECIAL[STEP as usize] == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) || MotionModule::frame(boma) >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && SPECIAL[STEP as usize] == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK_BACK && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) || MotionModule::frame(boma) >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && SPECIAL[STEP as usize] == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK_BACK, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT && [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK_BACK].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK) || MotionModule::frame(boma) >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && ATTACK[STEP as usize] == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK) || MotionModule::frame(boma) >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && ATTACK[STEP as usize] == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK) || MotionModule::frame(boma) >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && ATTACK[STEP as usize] == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK, true);
						};
						if  STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || MotionModule::frame(boma) >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && ATTACK[STEP as usize] == 1){
							if status_kind == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_SQUAT {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP, true);
							} else {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL, true);
							};
						};
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || MotionModule::frame(boma) >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && ATTACK[STEP as usize] == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || MotionModule::frame(boma) >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && ATTACK[STEP as usize] == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_START && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_START, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_END && [*FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_LOOP, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_START].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_END, true);
						};
						if status_kind != *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL && STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_AERIAL && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL) || MotionModule::frame(boma) >= FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) && ATTACK[STEP as usize] == 1){
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N2 && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N2, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_SQUAT && [*FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK].contains(&status_kind) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_SQUAT, true);
						};
					};
					if fighter_kind == *FIGHTER_KIND_SIMON || fighter_kind == *FIGHTER_KIND_RICHTER {
						if MotionModule::motion_kind(boma) == hash40("attack_air_f") {
							if BELMONT_ANGLE[STEP as usize] == 1 {
								MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("attack_air_f_lw"), 0.0, 1.0, 0.0, false, false);
							} else if BELMONT_ANGLE[STEP as usize] == 2 {
								MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("attack_air_f_hi"), 0.0, 1.0, 0.0, false, false);
							};
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_b") {
							if BELMONT_ANGLE[STEP as usize] == 1 {
								MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("attack_air_b_lw"), 0.0, 1.0, 0.0, false, false);
							} else if BELMONT_ANGLE[STEP as usize] == 2 {
								MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("attack_air_b_hi"), 0.0, 1.0, 0.0, false, false);
							};
						};
					};
					if fighter_kind == *FIGHTER_KIND_ROCKMAN {
						if STATUS[STEP as usize] == *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK)|| [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN].contains(&status_kind) || STATUS_DURATION[STEP as usize] < 1 ) && ATTACK[STEP as usize] == 1  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK)|| [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN].contains(&status_kind)|| STATUS_DURATION[STEP as usize] < 1 ) && ATTACK[STEP as usize] == 1  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK)|| [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN].contains(&status_kind)|| STATUS_DURATION[STEP as usize] < 1 ) && ATTACK[STEP as usize] == 1  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK)|| [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN].contains(&status_kind)|| STATUS_DURATION[STEP as usize] < 1 ) && ATTACK[STEP as usize] == 1  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE, true);
						};
						if STATUS[STEP as usize] == *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK)|| [*FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN].contains(&status_kind)|| STATUS_DURATION[STEP as usize] < 1 ) && ATTACK[STEP as usize] == 1  {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT, true);
						};
					};
					if FASTFALL[STEP as usize] == 1 {
						WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					} else {
						WorkModule::set_flag(boma, false, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					};
					ControlModule::set_main_stick_x(boma, X[STEP as usize]);
					ControlModule::set_main_stick_y(boma, Y[STEP as usize]);
					STEP += 1;
				};
			};
		};
	}
}
//COPYCAT MACRO
#[fighter_frame_callback]
pub fn recorder(fighter : &mut L2CFighterCommon) {
    unsafe {
		if smash::app::smashball::is_training_mode() == true {
			let lua_state = fighter.lua_state_agent;
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let prev_status = StatusModule::prev_status_kind(boma, 0);
			let fighter_num = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let fighter_kind = smash::app::utility::get_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if fighter_num == 0 {
				if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) && status_kind != *FIGHTER_STATUS_KIND_GUARD && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD)) || status_kind == *FIGHTER_STATUS_KIND_DEAD{
					IS_RECORD = true;
					IS_PLAY = false;
					STATUS.clear();
					FASTFALL.clear();
					DASHRECORDS.clear();
					LR.clear();
					X.clear();
					Y.clear();
					SUB_X.clear();
					SUB_Y.clear();
					STICK_DIR.clear();
					AERIAL_KIND.clear();
					COMMAND_FLAG0.clear();
					COMMAND_FLAG1.clear();
					COMMAND_FLAG2.clear();
					COMMAND_FLAG3.clear();
					FLICK_X.clear();
					FLICK_X_DIR.clear();
					FLICK_Y.clear();
					FLICK_Y_DIR.clear();
					//Buttons
					ATTACK.clear();
					SPECIAL.clear();
					GUARD.clear();
					JUMP.clear();
					CATCH.clear();
					JUMP_MINI.clear();
					CSTICK_ON.clear();
					BELMONT_ANGLE.clear();
					STATUS_DURATION.clear();
					//Expected
					EXPECTED_X_POS.clear();
					EXPECTED_Y_POS.clear();
					EXPECTED_SPEED_X.clear();
					EXPECTED_SPEED_Y.clear();
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
					//Entirely for bugtesting
					EXPECTED_X_POS.push(PostureModule::pos_x(boma)*-1.0);
					EXPECTED_Y_POS.push(PostureModule::pos_y(boma));
					EXPECTED_SPEED_X.push(KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)*-1.0);
					EXPECTED_SPEED_Y.push(KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
					
					if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR ||  status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR && MotionModule::frame(boma) < 10.0{
						if MotionModule::motion_kind(boma) == hash40("attack_air_f") || MotionModule::motion_kind(boma) == hash40("landing_air_f"){
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_F);
							//println!("fair");
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_b") || MotionModule::motion_kind(boma) == hash40("landing_air_b") {
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_B);
							//println!("bair");
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_hi") || MotionModule::motion_kind(boma) == hash40("landing_air_hi") {
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_HI);
							//println!("uair");
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_lw") || MotionModule::motion_kind(boma) == hash40("landing_air_lw") {
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_LW);
							//println!("dair");
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_f_hi"){
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_F);
							//println!("hfair");
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_f_lw"){
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_F);
							//println!("lfair");
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_b_hi"){
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_B);
							//println!("hbair");
						} else if MotionModule::motion_kind(boma) == hash40("attack_air_b_lw"){
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_B);
							//println!("lbair");
						} else {
							AERIAL_KIND.push(*FIGHTER_COMMAND_ATTACK_AIR_KIND_N);
							//println!("nair");
						};
					} else {
						AERIAL_KIND.push(-1);
					};
					if status_kind != prev_status || STEP < 1 || StatusModule::is_changing(boma) {
						STATUS_DUR[ENTRY_ID] = 0;
					} else {
						STATUS_DUR[ENTRY_ID] += 1;
					};
					STATUS_DURATION.push(STATUS_DUR[ENTRY_ID]);
					if MotionModule::motion_kind(boma) == hash40("attack_air_f_lw") || MotionModule::motion_kind(boma) == hash40("attack_air_b_lw"){
							BELMONT_ANGLE.push(1);
					} else if MotionModule::motion_kind(boma) == hash40("attack_air_f_hi") ||  MotionModule::motion_kind(boma) == hash40("attack_air_b_hi"){
							BELMONT_ANGLE.push(2);
					} else {
							BELMONT_ANGLE.push(0);
					};
					if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 {
						COMMAND_FLAG0.push(0);
					} else {
						COMMAND_FLAG0.push(ControlModule::get_command_flag_cat(boma, 0));
					};
					let cat2 = ControlModule::get_command_flag_cat(boma, 1);
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0{
						COMMAND_FLAG1.push(0);
					} else {
						COMMAND_FLAG1.push(ControlModule::get_command_flag_cat(boma, 1));
					};
					COMMAND_FLAG2.push(ControlModule::get_command_flag_cat(boma, 2));
					COMMAND_FLAG3.push(ControlModule::get_command_flag_cat(boma, 3));
					//println!("aerial_kind {}", ControlModule::get_attack_air_kind(boma));
					//println!("0:{}  1:{}  2:{}  3:{}", ControlModule::get_command_flag_cat(boma, 0), ControlModule::get_command_flag_cat(boma, 1), ControlModule::get_command_flag_cat(boma, 2), ControlModule::get_command_flag_cat(boma, 3));
					STATUS.push(status_kind);
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
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
						GUARD.push(1);
					} else {
						GUARD.push(0);
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
						CATCH.push(1);
					} else {
						CATCH.push(0);
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && (!ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || status_kind != *FIGHTER_STATUS_KIND_JUMP_SQUAT) {
						JUMP.push(1);
					} else {
						JUMP.push(0);
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
						JUMP_MINI.push(1);
					} else {
						JUMP_MINI.push(0);
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)  && (!ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || status_kind != *FIGHTER_STATUS_KIND_JUMP_SQUAT) {
						FLICK_JUMP.push(1);
					} else {
						FLICK_JUMP.push(0);
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
						CSTICK_ON.push(1);
					} else {
						CSTICK_ON.push(0);
					};
					if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
						FASTFALL.push(1);
					} else {
						FASTFALL.push(0);
					};
					
					LR.push(PostureModule::lr(boma)*-1.0);
					X.push(ControlModule::get_stick_x(boma)*-1.0);
					Y.push(ControlModule::get_stick_y(boma));
					FLICK_X.push(ControlModule::get_flick_x(boma));
					FLICK_Y.push(ControlModule::get_flick_y(boma));
					FLICK_X_DIR.push(ControlModule::get_flick_x_dir(boma));
					FLICK_Y_DIR.push(ControlModule::get_flick_y_dir(boma));
					SUB_X.push(ControlModule::get_sub_stick_x(boma));
					SUB_Y.push(ControlModule::get_sub_stick_y(boma));
				};
			};
		};
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(recorder);
    smashline::install_agent_frame_callbacks!(repeater);
	skyline::install_hook!(command_flag_cat_hook);
	skyline::install_hooks!(handle_set_attack_air_kind, handle_get_attack_air_kind);
	skyline::install_hooks!(handle_get_x, handle_get_y);
	skyline::install_hook!(check_button_on_hook);
	//skyline::install_hook!(check_button_trigger_hook);
	skyline::install_hook!(check_button_off_hook);
	skyline::install_hook!(handle_get_sub_x);
	skyline::install_hook!(handle_get_sub_y);
	skyline::install_hooks!(handle_get_main_stick_x, handle_get_main_stick_y);
	skyline::install_hooks!(handle_get_flick_x, handle_get_flick_y);
}