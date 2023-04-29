pub struct Context {
    pub game_assembly_module : isize,
}

impl Context {
    pub const fn new() -> Self {
        Self {
            game_assembly_module: 0,
        }
    }
}
