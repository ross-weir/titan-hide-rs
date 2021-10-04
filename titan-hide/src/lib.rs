#[cfg(not(target_os = "windows"))]
compile_error!("Only windows is supported by TitanHide");

use titan_hide_bindings::*;

use std::fs::OpenOptions;
use std::io::Write;
use std::os::windows::prelude::*;

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

#[derive(Debug, Clone)]
pub struct TitanHide(String, u32);

impl Default for TitanHide {
    fn default() -> Self {
        TitanHide {
            0: String::from("\\\\.\\TitanHide"),
            1: 0xffffffff, // all settings enabled
        }
    }
}

impl TitanHide {
    pub fn new(device_path: String, opts: u32) -> Self {
        TitanHide {
            0: device_path,
            1: opts,
        }
    }

    pub fn hide(&self, pid: u32) -> std::io::Result<()> {
        self.call(pid, HIDE_COMMAND_HidePid)
    }

    pub fn unhide(&self, pid: u32) -> std::io::Result<()> {
        self.call(pid, HIDE_COMMAND_UnhidePid)
    }

    pub fn unhide_all(&self) -> std::io::Result<()> {
        self.call(0, HIDE_COMMAND_UnhideAll)
    }

    fn call(&self, pid: u32, cmd: HIDE_COMMAND) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .share_mode(0)
            .open(self.0.clone())?;
        let hide_info = HIDE_INFO {
            Command: cmd,
            Type: self.1,
            Pid: pid,
        };
        let bytes = unsafe { any_as_u8_slice(&hide_info) };

        file.write_all(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::any_as_u8_slice;
    use titan_hide_bindings::{HIDE_COMMAND_HidePid, HIDE_INFO};

    #[test]
    fn any_as_u8_slice_returns_expected_bytes() {
        let hide_info = HIDE_INFO {
            Command: HIDE_COMMAND_HidePid,
            Type: 0xffffffff, // currently just use all
            Pid: 70,
        };
        let actual = unsafe { any_as_u8_slice(&hide_info) };
        let expected: &[u8] = &[0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0x46, 0, 0, 0];

        assert_eq!(expected, actual);
    }
}
