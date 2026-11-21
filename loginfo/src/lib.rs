pub use loginfo_derive::LogInfo;
pub use loginfo_derive::create_user;
pub use loginfo_derive::log_info;

pub trait LogInfo {
    fn log_name(&self);
}
