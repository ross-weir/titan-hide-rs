fn main() {
    windows::build! {
        Windows::Win32::Storage::FileSystem::{CreateFileA, WriteFile, FILE_ACCESS_FLAGS, FILE_SHARE_MODE, FILE_CREATION_DISPOSITION, FILE_FLAGS_AND_ATTRIBUTES},
        Windows::Win32::Security::SECURITY_ATTRIBUTES,
        Windows::Win32::Foundation::{CloseHandle, HANDLE},
    };
}
