mod context;
mod defines;
mod unity_api;
mod memory;
mod cache;

pub fn initialize(wait_for_module : bool) -> bool {
    use context::Context;
    use std::time::Duration;
    use std::thread;
    use defines::IL2CPP_MAIN_MODULE;

    let ctx = &mut Context::new();
    loop {
        
        ctx.game_assembly_module = memory::get_module(IL2CPP_MAIN_MODULE);
        if !wait_for_module || !ctx.game_assembly_module.is_null() { break; }
        thread::sleep(Duration::from_secs(1));
    }

    if ctx.game_assembly_module.is_null() { return false; }

    if !unity_api::initialize(ctx) { return false; }

    true
}

