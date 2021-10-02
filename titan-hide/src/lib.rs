#[cfg(not(target_os = "windows"))]
compile_error!("Only windows is supported by TitanHide");

use titan_hide_bindings::*;

use crate::error::{TitanHideError, TitanHideResult};
use std::ffi::c_void;
use std::mem::size_of;
use win_bindings::Windows::Win32::Foundation::{CloseHandle, HANDLE, PSTR};
use win_bindings::Windows::Win32::Security::SECURITY_ATTRIBUTES;
use win_bindings::Windows::Win32::Storage::FileSystem::{
    CreateFileA, WriteFile, FILE_FLAGS_AND_ATTRIBUTES, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
    FILE_SHARE_NONE, OPEN_EXISTING,
};
use windows::*;

pub mod error;

#[derive(Debug, Clone)]
pub struct TitanHide(String);

impl Default for TitanHide {
    fn default() -> Self {
        TitanHide {
            0: String::from("\\\\.\\TitanHide"),
        }
    }
}

impl TitanHide {
    fn new(device_path: String) -> Self {
        TitanHide { 0: device_path }
    }

    pub fn hide(&self, pid: u32) -> TitanHideResult<()> {
        self.call(pid, HIDE_COMMAND_HidePid)
    }

    pub fn unhide(&self, pid: u32) -> TitanHideResult<()> {
        self.call(pid, HIDE_COMMAND_UnhidePid)
    }

    pub fn unhide_all(&self) -> TitanHideResult<()> {
        self.call(0, HIDE_COMMAND_UnhideAll)
    }

    fn call(&self, pid: u32, cmd: HIDE_COMMAND) -> TitanHideResult<()> {
        let device = unsafe {
            match CreateFileA(
                PSTR(self.0.clone().as_mut_ptr()),
                FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                FILE_SHARE_NONE,
                &SECURITY_ATTRIBUTES::default(),
                OPEN_EXISTING,
                FILE_FLAGS_AND_ATTRIBUTES::default(),
                HANDLE::default(),
            )
            .ok()
            {
                Ok(h) => h,
                Err(e) => return Err(TitanHideError::OpenDevice(e.code().0)),
            }
        };
        let hide_info = HIDE_INFO {
            Command: cmd,
            Type: 0xffffffffu32, // currently just use all
            Pid: pid,
        };
        let bytes_written = Box::new(0 as u32);
        let hide_info_ptr = &hide_info as *const _ as *const c_void;
        let did_write = unsafe {
            WriteFile(
                device,
                hide_info_ptr,
                size_of::<HIDE_INFO>() as u32,
                Box::into_raw(bytes_written),
                std::ptr::null_mut(),
            )
        };

        unsafe {
            CloseHandle(device);
        }

        match did_write.ok() {
            Ok(..) => Ok(()),
            Err(e) => Err(TitanHideError::WriteDevice(e.code().0)),
        }
    }
}
//
// #[derive(Debug, Default)]
// pub struct Builder {
//     pids: Vec<u32>,
//     device_path: Option<String>,
// }
//
// impl Builder {
//     pub fn by_pid(mut self, pid: u32) -> Builder {
//         self.pids.push(pid);
//         self
//     }
// }