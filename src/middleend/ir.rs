// The ir instructions.
#[derive(Debug)]
pub enum IrInstructions {
    PrintString(String),
}

// The ir program.
#[derive(Debug)]
pub struct IrProgram {
    pub instructions: Vec<IrInstructions>,
}