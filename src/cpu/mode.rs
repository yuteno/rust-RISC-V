use crate::exception::Exception;
use super::Mode;

impl Mode {
    pub fn require(&self, require: Mode) ->  Result<(), Exception>{
        match require {
            Mode::User => {
                if self == &Mode::Machine || self == &Mode::Supervisor || self == &Mode::User {
                    return Ok(());
                }
                Err(Exception::IllegalInstruction)
            }
            Mode::Supervisor => {
                if self == &Mode::Machine || self == &Mode::Supervisor {
                    return Ok(());
                }
                Err(Exception::IllegalInstruction)
            }
            Mode::Machine => {
                if self == &Mode::Machine {
                    return Ok(())
                }
                Err(Exception::IllegalInstruction)
            }
            _ => Err(Exception::IllegalInstruction)
        }

    }
}