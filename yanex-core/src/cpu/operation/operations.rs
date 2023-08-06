mod clear_flag;
mod control;
mod load;
mod set_flag;

pub use clear_flag::*;
pub use control::*;
pub use load::*;
pub use set_flag::*;

use super::addressing_mode::AddressingMode;
use super::addressing_mode::AddressingModeReadAddress;
use super::addressing_mode::AddressingModeReadData;

macro_rules! read {
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

use read;
