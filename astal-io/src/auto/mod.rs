// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-astal
// from ../gir-files
// DO NOT EDIT

mod application;
pub use self::application::Application;

mod daemon;
pub use self::daemon::Daemon;

mod process;
pub use self::process::Process;

mod time;
pub use self::time::Time;

mod variable;
pub use self::variable::Variable;

mod variable_base;
pub use self::variable_base::VariableBase;

mod enums;
pub use self::enums::AppError;

pub(crate) mod functions;

pub(crate) mod traits {
    pub use super::application::ApplicationExt;
    pub use super::daemon::DaemonExt;
    pub use super::process::ProcessExt;
    pub use super::time::TimeExt;
    pub use super::variable::VariableExt;
    pub use super::variable_base::VariableBaseExt;
}
pub(crate) mod builders {
    pub use super::process::ProcessBuilder;
    pub use super::variable::VariableBuilder;
}
