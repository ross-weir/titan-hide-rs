use std::fmt::{Display, Formatter, Result};

pub type TitanHideResult<T> = std::result::Result<T, TitanHideError>;

#[derive(Debug, Clone, PartialEq)]
pub enum TitanHideError {
    OpenDevice(u32),
    WriteDevice(u32),
    PidNotFound(u32),
}

impl Display for TitanHideError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            TitanHideError::OpenDevice(win_err) => write!(
                f,
                "Failed to open TitanHide device, windows error 0x{:X}",
                win_err
            ),
            TitanHideError::WriteDevice(win_err) => write!(
                f,
                "Failed to write to TitanHide device, windows error 0x{:X}",
                win_err
            ),
            TitanHideError::PidNotFound(pid) => write!(f, "Pid {} not found", pid),
        }
    }
}
