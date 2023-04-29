use crate::il2cpp::context::Context;

pub fn initialize(ctx : &mut Context) -> bool {
    use super::memory;
    use crate::il2cpp::defines::{IL2CPP_INIT_EXPORT, IL2CPP_EXPORT_METHODS};

    if memory::resolve_export(ctx.game_assembly_module, IL2CPP_INIT_EXPORT).is_null() { 
        return false; 
    }
    for (i, method) in IL2CPP_EXPORT_METHODS.iter().enumerate() {
        ctx.methods[i] = memory::resolve_export(ctx.game_assembly_module, method);
        if ctx.methods[i].is_null(){ return false; }
    }
    true
}