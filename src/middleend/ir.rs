// The ir instructions.
#[derive(Debug)]
pub enum IrInstructions {
    PrintString(String),
    PrintlnString(String),

    PrintVariable(String),
    PrintlnVariable(String),

    AssignVariable {
        name: String,
        value: String,
    },

    DeclareVariable {
        name: String,
        value: String,
        mutable: bool,
    },
}

// The ir program.
#[derive(Debug)]
pub struct IrProgram {
    pub instructions: Vec<IrInstructions>,
}
