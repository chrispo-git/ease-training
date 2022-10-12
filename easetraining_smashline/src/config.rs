use skyline_config::*;

pub static mut IS_PARAMETER : bool = true;


pub fn get_config() -> StorageHolder<SdCardStorage> {
    StorageHolder::new(SdCardStorage::new("ultimate/ease_training"))
}

pub fn is_param(config: &StorageHolder<SdCardStorage>) -> bool {
    config.get_flag("parameter_changes")
}
pub fn config_check() -> () {
	unsafe {
		let config = get_config();
		IS_PARAMETER = is_param(&config);
	}
}
