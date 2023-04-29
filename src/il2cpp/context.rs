use winapi::shared::minwindef::{FARPROC, HMODULE};
pub struct Context {
    pub game_assembly_module : HMODULE,
    pub methods : [FARPROC; 17],
}

impl Context {
    pub const fn new() -> Self {
        Self {
            game_assembly_module: 0 as HMODULE,
            methods: [0 as FARPROC; 17]
        }
    }
}
