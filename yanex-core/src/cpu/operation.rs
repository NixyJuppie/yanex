use crate::cpu::{Cpu, CpuMemory};
pub use addressing_mode::AddressingMode;
pub use opcode::Opcode;

mod addressing_mode;
mod opcode;
mod operations;

#[derive(Debug, Clone)]
pub enum Operation {
    Jump(operations::Jump),
    SetCarry(operations::SetCarry),
    SetDecimal(operations::SetDecimal),
    SetInterrupt(operations::SetInterrupt),
    ClearCarry(operations::ClearCarry),
    ClearDecimal(operations::ClearDecimal),
    ClearInterrupt(operations::ClearInterrupt),
}

impl Operation {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> bool {
        match self {
            Operation::Jump(operation) => operation.execute(cpu, memory),
            Operation::SetCarry(operation) => operation.execute(cpu, memory),
            Operation::SetDecimal(operation) => operation.execute(cpu, memory),
            Operation::SetInterrupt(operation) => operation.execute(cpu, memory),
            Operation::ClearCarry(operation) => operation.execute(cpu, memory),
            Operation::ClearDecimal(operation) => operation.execute(cpu, memory),
            Operation::ClearInterrupt(operation) => operation.execute(cpu, memory),
        }
        .is_some()
    }
}

impl From<Opcode> for Operation {
    fn from(opcode: Opcode) -> Self {
        use addressing_mode::AddressingMode::*;
        use opcode::Opcode::*;
        use operations::*;

        match opcode {
            JmpAbs => Operation::Jump(Jump::Decoded(Absolute)),
            JmpInd => Operation::Jump(Jump::Decoded(Indirect)),
            SecImp => Operation::SetCarry(SetCarry::Decoded(Implied)),
            SedImp => Operation::SetDecimal(SetDecimal::Decoded(Implied)),
            SeiImp => Operation::SetInterrupt(SetInterrupt::Decoded(Implied)),
            ClcImp => Operation::ClearCarry(ClearCarry::Decoded(Implied)),
            CldImp => Operation::ClearDecimal(ClearDecimal::Decoded(Implied)),
            CliImp => Operation::ClearInterrupt(ClearInterrupt::Decoded(Implied)),
            _ => todo!("Not supported opcode {opcode:?}"),
        }
    }
}
