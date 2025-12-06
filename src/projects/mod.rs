pub mod cargo;
pub mod detect;
pub mod flang;
pub mod fpm;
pub mod uv;

// Re-export shared structs and functions
pub use self::lib::*;

// Re-export main project management functions
pub use self::detect::{detect_project_info, get_or_create_project};

mod lib;
