mod arithmetic;
mod branch;
mod control;
mod flags;
mod increment;
mod load;
mod logic;
mod stack;
mod store;
mod transfer;

pub use arithmetic::*;
pub use branch::*;
pub use control::*;
pub use flags::*;
pub use increment::*;
pub use load::*;
pub use logic::*;
pub use stack::*;
pub use store::*;
pub use transfer::*;

use super::addressing_mode::AddressingMode;
use super::addressing_mode::AddressingModeReadAddress;
use super::addressing_mode::AddressingModeReadData;
use super::addressing_mode::AddressingModeWriteData;

macro_rules! mem_read {
    ($self:ident, $cpu:ident, $memory:ident, $read:ident, $reading_state:ident) => {{
        match $read.read($cpu, $memory) {
            None => {
                *$self = Self::$reading_state($read);
                None
            }
            Some(data) => Some(data),
        }
    }};
}

macro_rules! mem_write {
    ($self:ident, $cpu:ident, $memory:ident, $write:ident, $writing_state:ident) => {{
        match $write.write($cpu, $memory) {
            None => {
                *$self = Self::$writing_state($write);
                None
            }
            Some(v) => Some(v),
        }
    }};
}

use mem_read;
use mem_write;
