use crate::cpu::{Cpu, CpuMemory};
pub use addressing_mode::AddressingMode;
pub use opcode::Opcode;

mod addressing_mode;
mod opcode;
mod operations;

#[derive(Debug, Clone)]
pub enum Operation {
    // Load
    LoadAccumulator(operations::LoadAccumulator),
    LoadIndexX(operations::LoadIndexX),
    LoadIndexY(operations::LoadIndexY),
    // Store
    StoreAccumulator(operations::StoreAccumulator),
    StoreIndexX(operations::StoreIndexX),
    StoreIndexY(operations::StoreIndexY),
    // Control
    Jump(operations::Jump),
    JumpSubroutine(operations::JumpSubroutine),
    NoOperation(operations::NoOperation),
    // Branch
    BranchCarryClear(operations::BranchCarryClear),
    BranchCarrySet(operations::BranchCarrySet),
    BranchZeroClear(operations::BranchZeroClear),
    BranchZeroSet(operations::BranchZeroSet),
    BranchNegativeClear(operations::BranchNegativeClear),
    BranchNegativeSet(operations::BranchNegativeSet),
    BranchOverflowClear(operations::BranchOverflowClear),
    BranchOverflowSet(operations::BranchOverflowSet),
    // Set flag
    SetCarry(operations::SetCarry),
    SetDecimal(operations::SetDecimal),
    SetInterrupt(operations::SetInterrupt),
    // Clear flag
    ClearCarry(operations::ClearCarry),
    ClearDecimal(operations::ClearDecimal),
    ClearInterrupt(operations::ClearInterrupt),
}

impl Operation {
    pub fn execute(&mut self, cpu: &mut Cpu, memory: &mut CpuMemory) -> bool {
        match self {
            // Load
            Operation::LoadAccumulator(operation) => operation.execute(cpu, memory),
            Operation::LoadIndexX(operation) => operation.execute(cpu, memory),
            Operation::LoadIndexY(operation) => operation.execute(cpu, memory),
            // Store
            Operation::StoreAccumulator(operation) => operation.execute(cpu, memory),
            Operation::StoreIndexX(operation) => operation.execute(cpu, memory),
            Operation::StoreIndexY(operation) => operation.execute(cpu, memory),
            // Control
            Operation::Jump(operation) => operation.execute(cpu, memory),
            Operation::JumpSubroutine(operation) => operation.execute(cpu, memory),
            Operation::NoOperation(operation) => operation.execute(cpu, memory),
            // Branch
            Operation::BranchCarryClear(operation) => operation.execute(cpu, memory),
            Operation::BranchCarrySet(operation) => operation.execute(cpu, memory),
            Operation::BranchZeroClear(operation) => operation.execute(cpu, memory),
            Operation::BranchZeroSet(operation) => operation.execute(cpu, memory),
            Operation::BranchNegativeClear(operation) => operation.execute(cpu, memory),
            Operation::BranchNegativeSet(operation) => operation.execute(cpu, memory),
            Operation::BranchOverflowClear(operation) => operation.execute(cpu, memory),
            Operation::BranchOverflowSet(operation) => operation.execute(cpu, memory),
            // Set flag
            Operation::SetCarry(operation) => operation.execute(cpu, memory),
            Operation::SetDecimal(operation) => operation.execute(cpu, memory),
            Operation::SetInterrupt(operation) => operation.execute(cpu, memory),
            // Clear flag
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
            // Load
            LdaImm => Operation::LoadAccumulator(LoadAccumulator::Decoded(Immediate)),
            LdaZp => Operation::LoadAccumulator(LoadAccumulator::Decoded(ZeroPage)),
            LdaZpX => Operation::LoadAccumulator(LoadAccumulator::Decoded(ZeroPageX)),
            LdaAbs => Operation::LoadAccumulator(LoadAccumulator::Decoded(Absolute)),
            LdaAbsX => Operation::LoadAccumulator(LoadAccumulator::Decoded(AbsoluteX)),
            LdaAbsY => Operation::LoadAccumulator(LoadAccumulator::Decoded(AbsoluteY)),
            LdaIndX => Operation::LoadAccumulator(LoadAccumulator::Decoded(IndirectX)),
            LdaIndY => Operation::LoadAccumulator(LoadAccumulator::Decoded(IndirectY)),
            LdxImm => Operation::LoadIndexX(LoadIndexX::Decoded(Immediate)),
            LdxZp => Operation::LoadIndexX(LoadIndexX::Decoded(ZeroPage)),
            LdxZpY => Operation::LoadIndexX(LoadIndexX::Decoded(ZeroPageY)),
            LdxAbs => Operation::LoadIndexX(LoadIndexX::Decoded(Absolute)),
            LdxAbsY => Operation::LoadIndexX(LoadIndexX::Decoded(AbsoluteY)),
            LdyImm => Operation::LoadIndexY(LoadIndexY::Decoded(Immediate)),
            LdyZp => Operation::LoadIndexY(LoadIndexY::Decoded(ZeroPage)),
            LdyZpX => Operation::LoadIndexY(LoadIndexY::Decoded(ZeroPageX)),
            LdyAbs => Operation::LoadIndexY(LoadIndexY::Decoded(Absolute)),
            LdyAbsX => Operation::LoadIndexY(LoadIndexY::Decoded(AbsoluteX)),
            // Store
            StaZp => Operation::StoreAccumulator(StoreAccumulator::Decoded(ZeroPage)),
            StaZpX => Operation::StoreAccumulator(StoreAccumulator::Decoded(ZeroPageX)),
            StaAbs => Operation::StoreAccumulator(StoreAccumulator::Decoded(Absolute)),
            StaAbsX => Operation::StoreAccumulator(StoreAccumulator::Decoded(AbsoluteX)),
            StaAbsY => Operation::StoreAccumulator(StoreAccumulator::Decoded(AbsoluteY)),
            StaIndX => Operation::StoreAccumulator(StoreAccumulator::Decoded(IndirectX)),
            StaIndY => Operation::StoreAccumulator(StoreAccumulator::Decoded(IndirectY)),
            StxZp => Operation::StoreIndexX(StoreIndexX::Decoded(ZeroPage)),
            StxZpY => Operation::StoreIndexX(StoreIndexX::Decoded(ZeroPageY)),
            StxAbs => Operation::StoreIndexX(StoreIndexX::Decoded(Absolute)),
            StyZp => Operation::StoreIndexY(StoreIndexY::Decoded(ZeroPage)),
            StyZpX => Operation::StoreIndexY(StoreIndexY::Decoded(ZeroPageX)),
            StyAbs => Operation::StoreIndexY(StoreIndexY::Decoded(Absolute)),
            // Control
            JmpAbs => Operation::Jump(Jump::Decoded(Absolute)),
            JmpInd => Operation::Jump(Jump::Decoded(Indirect)),
            JsrAbs => Operation::JumpSubroutine(JumpSubroutine::Decoded(Absolute)),
            NopImp => Operation::NoOperation(NoOperation::Decoded(Implied)),
            // Branch
            BccRel => Operation::BranchCarryClear(BranchCarryClear::Decoded(Relative)),
            BcsRel => Operation::BranchCarrySet(BranchCarrySet::Decoded(Relative)),

            BneRel => Operation::BranchZeroClear(BranchZeroClear::Decoded(Relative)),
            BeqRel => Operation::BranchZeroSet(BranchZeroSet::Decoded(Relative)),

            BplRel => Operation::BranchNegativeClear(BranchNegativeClear::Decoded(Relative)),
            BmiRel => Operation::BranchNegativeSet(BranchNegativeSet::Decoded(Relative)),

            BvcRel => Operation::BranchOverflowClear(BranchOverflowClear::Decoded(Relative)),
            BvsRel => Operation::BranchOverflowSet(BranchOverflowSet::Decoded(Relative)),
            // Set flag
            SecImp => Operation::SetCarry(SetCarry::Decoded(Implied)),
            SedImp => Operation::SetDecimal(SetDecimal::Decoded(Implied)),
            SeiImp => Operation::SetInterrupt(SetInterrupt::Decoded(Implied)),
            // Clear flag
            ClcImp => Operation::ClearCarry(ClearCarry::Decoded(Implied)),
            CldImp => Operation::ClearDecimal(ClearDecimal::Decoded(Implied)),
            CliImp => Operation::ClearInterrupt(ClearInterrupt::Decoded(Implied)),
            _ => todo!("Not supported opcode {opcode:?}"),
        }
    }
}
