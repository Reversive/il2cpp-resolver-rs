use crate::il2cpp::context::Context;
use windows::core::PCSTR;

// TODO: solve obfuscated/stripped exports
fn resolve_export(module : isize, name : PCSTR) -> isize {
    use windows::Win32::System::LibraryLoader::GetProcAddress;
    use windows::Win32::Foundation::HMODULE;
    let resolved_address = unsafe { GetProcAddress(HMODULE(module), name)};
    match resolved_address {
        Some(address) => address as isize,
        None => 0
    }
} 

pub fn initialize(ctx : &mut Context) -> bool {
    use crate::il2cpp::defines::IL2CPP_INIT_EXPORT;
    if resolve_export(ctx.game_assembly_module, IL2CPP_INIT_EXPORT) == 0 { return false; }
    
    true
}