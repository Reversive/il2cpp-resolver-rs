mod context;
mod defines;
mod unity_api;

pub fn initialize(wait_for_module : bool) -> bool {
    use windows::Win32::System::LibraryLoader::GetModuleHandleA;
    use context::Context;
    use windows::core::PCSTR;
    use std::time::Duration;
    use std::thread;
    use defines::IL2CPP_MAIN_MODULE;
    let ctx = &mut Context::new();
    loop {
        
        if let Ok(module) = unsafe { GetModuleHandleA(PCSTR::from_raw(IL2CPP_MAIN_MODULE.as_ptr())) } {
            ctx.game_assembly_module = module.0;
            break;
        }

        if !wait_for_module { break; }
        thread::sleep(Duration::from_secs(1));
    }

    if ctx.game_assembly_module == 0 { return false; }

    if !unity_api::initialize() { return false; }

    true
}

