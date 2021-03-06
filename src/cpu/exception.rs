#[derive(Debug)]
pub enum Exception {
    IllegalInstruction,
}

impl Exception {
    fn exception_code(&self) -> u64 {
        match self {
            Exception::IllegalInstruction => 2,
        }
    }
}