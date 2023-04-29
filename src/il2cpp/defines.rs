use windows::core::PCSTR;

const RAW_IL2CPP_MAIN_MODULE_DLL : &'static [u8] = b"GameAssembly.dll\0";
pub const IL2CPP_MAIN_MODULE : PCSTR = PCSTR::from_raw(RAW_IL2CPP_MAIN_MODULE_DLL.as_ptr());