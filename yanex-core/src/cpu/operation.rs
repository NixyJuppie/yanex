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
    // Transfer
    TransferAccumulatorToX(operations::TransferAccumulatorToX),
    TransferAccumulatorToY(operations::TransferAccumulatorToY),
    TransferXToAccumulator(operations::TransferXToAccumulator),
    TransferYToAccumulator(operations::TransferYToAccumulator),
    TransferStackPointerToX(operations::TransferStackPointerToX),
    TransferXToStackPointer(operations::TransferXToStackPointer),
    // Stack
    PushAccumulator(operations::PushAccumulator),
    PullAccumulator(operations::PullAccumulator),
    PushStatus(operations::PushStatus),
    PullStatus(operations::PullStatus),
    // Logic
    BitTest(operations::BitTest),
    BitAnd(operations::BitAnd),
    BitOr(operations::BitOr),
    BitXor(operations::BitXor),
    // Shift
    ShiftLeft(operations::ShiftLeft),
    ShiftRight(operations::ShiftRight),
    RotateLeft(operations::RotateLeft),
    RotateRight(operations::RotateRight),
    // Increment
    IncrementIndexX(operations::IncrementIndexX),
    DecrementIndexX(operations::DecrementIndexX),
    IncrementIndexY(operations::IncrementIndexY),
    DecrementIndexY(operations::DecrementIndexY),
    IncrementMemory(operations::IncrementMemory),
    DecrementMemory(operations::DecrementMemory),
    // Arithmetic
    CompareAccumulator(operations::CompareAccumulator),
    CompareIndexX(operations::CompareIndexX),
    CompareIndexY(operations::CompareIndexY),
    AddCarry(operations::AddCarry),
    SubtractCarry(operations::SubtractCarry),
    // Control
    Jump(operations::Jump),
    JumpSubroutine(operations::JumpSubroutine),
    ReturnSubroutine(operations::ReturnSubroutine),
    ReturnInterrupt(operations::ReturnInterrupt),
    #[allow(clippy::enum_variant_names)]
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
    // Flags
    SetCarry(operations::SetCarry),
    SetDecimal(operations::SetDecimal),
    SetInterrupt(operations::SetInterrupt),
    ClearCarry(operations::ClearCarry),
    ClearDecimal(operations::ClearDecimal),
    ClearInterrupt(operations::ClearInterrupt),
    ClearOverflow(operations::ClearOverflow),
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
            // Transfer
            Operation::TransferAccumulatorToX(operation) => operation.execute(cpu, memory),
            Operation::TransferAccumulatorToY(operation) => operation.execute(cpu, memory),
            Operation::TransferXToAccumulator(operation) => operation.execute(cpu, memory),
            Operation::TransferYToAccumulator(operation) => operation.execute(cpu, memory),
            Operation::TransferStackPointerToX(operation) => operation.execute(cpu, memory),
            Operation::TransferXToStackPointer(operation) => operation.execute(cpu, memory),
            // Stack
            Operation::PushAccumulator(operation) => operation.execute(cpu, memory),
            Operation::PullAccumulator(operation) => operation.execute(cpu, memory),
            Operation::PushStatus(operation) => operation.execute(cpu, memory),
            Operation::PullStatus(operation) => operation.execute(cpu, memory),
            // Logic
            Operation::BitTest(operation) => operation.execute(cpu, memory),
            Operation::BitAnd(operation) => operation.execute(cpu, memory),
            Operation::BitOr(operation) => operation.execute(cpu, memory),
            Operation::BitXor(operation) => operation.execute(cpu, memory),
            // Shift
            Operation::ShiftLeft(operation) => operation.execute(cpu, memory),
            Operation::ShiftRight(operation) => operation.execute(cpu, memory),
            Operation::RotateLeft(operation) => operation.execute(cpu, memory),
            Operation::RotateRight(operation) => operation.execute(cpu, memory),
            // Increment
            Operation::IncrementIndexX(operation) => operation.execute(cpu, memory),
            Operation::DecrementIndexX(operation) => operation.execute(cpu, memory),
            Operation::IncrementIndexY(operation) => operation.execute(cpu, memory),
            Operation::DecrementIndexY(operation) => operation.execute(cpu, memory),
            Operation::IncrementMemory(operation) => operation.execute(cpu, memory),
            Operation::DecrementMemory(operation) => operation.execute(cpu, memory),
            // Arithmetic
            Operation::CompareAccumulator(operation) => operation.execute(cpu, memory),
            Operation::CompareIndexX(operation) => operation.execute(cpu, memory),
            Operation::CompareIndexY(operation) => operation.execute(cpu, memory),
            Operation::AddCarry(operation) => operation.execute(cpu, memory),
            Operation::SubtractCarry(operation) => operation.execute(cpu, memory),
            // Control
            Operation::Jump(operation) => operation.execute(cpu, memory),
            Operation::JumpSubroutine(operation) => operation.execute(cpu, memory),
            Operation::ReturnSubroutine(operation) => operation.execute(cpu, memory),
            Operation::ReturnInterrupt(operation) => operation.execute(cpu, memory),
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
            // Flags
            Operation::SetCarry(operation) => operation.execute(cpu, memory),
            Operation::SetDecimal(operation) => operation.execute(cpu, memory),
            Operation::SetInterrupt(operation) => operation.execute(cpu, memory),
            Operation::ClearCarry(operation) => operation.execute(cpu, memory),
            Operation::ClearDecimal(operation) => operation.execute(cpu, memory),
            Operation::ClearInterrupt(operation) => operation.execute(cpu, memory),
            Operation::ClearOverflow(operation) => operation.execute(cpu, memory),
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
            // Transfer
            TaxImp => Operation::TransferAccumulatorToX(TransferAccumulatorToX::Decoded(Implied)),
            TayImp => Operation::TransferAccumulatorToY(TransferAccumulatorToY::Decoded(Implied)),
            TxaImp => Operation::TransferXToAccumulator(TransferXToAccumulator::Decoded(Implied)),
            TyaImp => Operation::TransferYToAccumulator(TransferYToAccumulator::Decoded(Implied)),
            TsxImp => Operation::TransferStackPointerToX(TransferStackPointerToX::Decoded(Implied)),
            TxsImp => Operation::TransferXToStackPointer(TransferXToStackPointer::Decoded(Implied)),
            // Stack
            PhaImp => Operation::PushAccumulator(PushAccumulator::Decoded(Implied)),
            PlaImp => Operation::PullAccumulator(PullAccumulator::Decoded(Implied)),
            PhpImp => Operation::PushStatus(PushStatus::Decoded(Implied)),
            PlpImp => Operation::PullStatus(PullStatus::Decoded(Implied)),
            // Logic
            BitZp => Operation::BitTest(BitTest::Decoded(ZeroPage)),
            BitAbs => Operation::BitTest(BitTest::Decoded(Absolute)),
            AndImm => Operation::BitAnd(BitAnd::Decoded(Immediate)),
            AndZp => Operation::BitAnd(BitAnd::Decoded(ZeroPage)),
            AndZpX => Operation::BitAnd(BitAnd::Decoded(ZeroPageX)),
            AndAbs => Operation::BitAnd(BitAnd::Decoded(Absolute)),
            AndAbsX => Operation::BitAnd(BitAnd::Decoded(AbsoluteX)),
            AndAbsY => Operation::BitAnd(BitAnd::Decoded(AbsoluteY)),
            AndIndX => Operation::BitAnd(BitAnd::Decoded(IndirectX)),
            AndIndY => Operation::BitAnd(BitAnd::Decoded(IndirectY)),
            OraImm => Operation::BitOr(BitOr::Decoded(Immediate)),
            OraZp => Operation::BitOr(BitOr::Decoded(ZeroPage)),
            OraZpX => Operation::BitOr(BitOr::Decoded(ZeroPageX)),
            OraAbs => Operation::BitOr(BitOr::Decoded(Absolute)),
            OraAbsX => Operation::BitOr(BitOr::Decoded(AbsoluteX)),
            OraAbsY => Operation::BitOr(BitOr::Decoded(AbsoluteY)),
            OraIndX => Operation::BitOr(BitOr::Decoded(IndirectX)),
            OraIndY => Operation::BitOr(BitOr::Decoded(IndirectY)),
            EorImm => Operation::BitXor(BitXor::Decoded(Immediate)),
            EorZp => Operation::BitXor(BitXor::Decoded(ZeroPage)),
            EorZpX => Operation::BitXor(BitXor::Decoded(ZeroPageX)),
            EorAbs => Operation::BitXor(BitXor::Decoded(Absolute)),
            EorAbsX => Operation::BitXor(BitXor::Decoded(AbsoluteX)),
            EorAbsY => Operation::BitXor(BitXor::Decoded(AbsoluteY)),
            EorIndX => Operation::BitXor(BitXor::Decoded(IndirectX)),
            EorIndY => Operation::BitXor(BitXor::Decoded(IndirectY)),
            // Shift
            AslAcc => Operation::ShiftLeft(ShiftLeft::Decoded(Accumulator)),
            AslAbs => Operation::ShiftLeft(ShiftLeft::Decoded(Absolute)),
            AslAbsX => Operation::ShiftLeft(ShiftLeft::Decoded(AbsoluteX)),
            AslZp => Operation::ShiftLeft(ShiftLeft::Decoded(ZeroPage)),
            AslZpX => Operation::ShiftLeft(ShiftLeft::Decoded(ZeroPageX)),
            LsrAcc => Operation::ShiftRight(ShiftRight::Decoded(Accumulator)),
            LsrAbs => Operation::ShiftRight(ShiftRight::Decoded(Absolute)),
            LsrAbsX => Operation::ShiftRight(ShiftRight::Decoded(AbsoluteX)),
            LsrZp => Operation::ShiftRight(ShiftRight::Decoded(ZeroPage)),
            LsrZpX => Operation::ShiftRight(ShiftRight::Decoded(ZeroPageX)),
            RolAcc => Operation::RotateLeft(RotateLeft::Decoded(Accumulator)),
            RolAbs => Operation::RotateLeft(RotateLeft::Decoded(Absolute)),
            RolAbsX => Operation::RotateLeft(RotateLeft::Decoded(AbsoluteX)),
            RolZp => Operation::RotateLeft(RotateLeft::Decoded(ZeroPage)),
            RolZpX => Operation::RotateLeft(RotateLeft::Decoded(ZeroPageX)),
            RorAcc => Operation::RotateRight(RotateRight::Decoded(Accumulator)),
            RorAbs => Operation::RotateRight(RotateRight::Decoded(Absolute)),
            RorAbsX => Operation::RotateRight(RotateRight::Decoded(AbsoluteX)),
            RorZp => Operation::RotateRight(RotateRight::Decoded(ZeroPage)),
            RorZpX => Operation::RotateRight(RotateRight::Decoded(ZeroPageX)),
            // Increment
            InxImp => Operation::IncrementIndexX(IncrementIndexX::Decoded(Implied)),
            DexImp => Operation::DecrementIndexX(DecrementIndexX::Decoded(Implied)),
            InyImp => Operation::IncrementIndexY(IncrementIndexY::Decoded(Implied)),
            DeyImp => Operation::DecrementIndexY(DecrementIndexY::Decoded(Implied)),
            IncAbs => Operation::IncrementMemory(IncrementMemory::Decoded(Absolute)),
            IncAbsX => Operation::IncrementMemory(IncrementMemory::Decoded(AbsoluteX)),
            IncZp => Operation::IncrementMemory(IncrementMemory::Decoded(ZeroPage)),
            IncZpX => Operation::IncrementMemory(IncrementMemory::Decoded(ZeroPageX)),
            DecAbs => Operation::DecrementMemory(DecrementMemory::Decoded(Absolute)),
            DecAbsX => Operation::DecrementMemory(DecrementMemory::Decoded(AbsoluteX)),
            DecZp => Operation::DecrementMemory(DecrementMemory::Decoded(ZeroPage)),
            DecZpX => Operation::DecrementMemory(DecrementMemory::Decoded(ZeroPageX)),
            // Arithmetic
            CmpImm => Operation::CompareAccumulator(CompareAccumulator::Decoded(Immediate)),
            CmpZp => Operation::CompareAccumulator(CompareAccumulator::Decoded(ZeroPage)),
            CmpZpX => Operation::CompareAccumulator(CompareAccumulator::Decoded(ZeroPageX)),
            CmpAbs => Operation::CompareAccumulator(CompareAccumulator::Decoded(Absolute)),
            CmpAbsX => Operation::CompareAccumulator(CompareAccumulator::Decoded(AbsoluteX)),
            CmpAbsY => Operation::CompareAccumulator(CompareAccumulator::Decoded(AbsoluteY)),
            CmpIndX => Operation::CompareAccumulator(CompareAccumulator::Decoded(IndirectX)),
            CmpIndY => Operation::CompareAccumulator(CompareAccumulator::Decoded(IndirectY)),
            CpxImm => Operation::CompareIndexX(CompareIndexX::Decoded(Immediate)),
            CpxZp => Operation::CompareIndexX(CompareIndexX::Decoded(ZeroPage)),
            CpxAbs => Operation::CompareIndexX(CompareIndexX::Decoded(Absolute)),
            CpyImm => Operation::CompareIndexY(CompareIndexY::Decoded(Immediate)),
            CpyZp => Operation::CompareIndexY(CompareIndexY::Decoded(ZeroPage)),
            CpyAbs => Operation::CompareIndexY(CompareIndexY::Decoded(Absolute)),
            AdcImm => Operation::AddCarry(AddCarry::Decoded(Immediate)),
            AdcZp => Operation::AddCarry(AddCarry::Decoded(ZeroPage)),
            AdcZpX => Operation::AddCarry(AddCarry::Decoded(ZeroPageX)),
            AdcAbs => Operation::AddCarry(AddCarry::Decoded(Absolute)),
            AdcAbsX => Operation::AddCarry(AddCarry::Decoded(AbsoluteX)),
            AdcAbsY => Operation::AddCarry(AddCarry::Decoded(AbsoluteY)),
            AdcIndX => Operation::AddCarry(AddCarry::Decoded(IndirectX)),
            AdcIndY => Operation::AddCarry(AddCarry::Decoded(IndirectY)),
            SbcImm => Operation::SubtractCarry(SubtractCarry::Decoded(Immediate)),
            SbcZp => Operation::SubtractCarry(SubtractCarry::Decoded(ZeroPage)),
            SbcZpX => Operation::SubtractCarry(SubtractCarry::Decoded(ZeroPageX)),
            SbcAbs => Operation::SubtractCarry(SubtractCarry::Decoded(Absolute)),
            SbcAbsX => Operation::SubtractCarry(SubtractCarry::Decoded(AbsoluteX)),
            SbcAbsY => Operation::SubtractCarry(SubtractCarry::Decoded(AbsoluteY)),
            SbcIndX => Operation::SubtractCarry(SubtractCarry::Decoded(IndirectX)),
            SbcIndY => Operation::SubtractCarry(SubtractCarry::Decoded(IndirectY)),
            // Control
            JmpAbs => Operation::Jump(Jump::Decoded(Absolute)),
            JmpInd => Operation::Jump(Jump::Decoded(Indirect)),
            JsrAbs => Operation::JumpSubroutine(JumpSubroutine::Decoded(Absolute)),
            RtsImp => Operation::ReturnSubroutine(ReturnSubroutine::Decoded(Implied)),
            RtiImp => Operation::ReturnInterrupt(ReturnInterrupt::Decoded(Implied)),
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
            // Flags
            SecImp => Operation::SetCarry(SetCarry::Decoded(Implied)),
            SedImp => Operation::SetDecimal(SetDecimal::Decoded(Implied)),
            SeiImp => Operation::SetInterrupt(SetInterrupt::Decoded(Implied)),
            ClcImp => Operation::ClearCarry(ClearCarry::Decoded(Implied)),
            CldImp => Operation::ClearDecimal(ClearDecimal::Decoded(Implied)),
            CliImp => Operation::ClearInterrupt(ClearInterrupt::Decoded(Implied)),
            ClvImp => Operation::ClearOverflow(ClearOverflow::Decoded(Implied)),
            _ => todo!("Not supported opcode {opcode:?}"),
        }
    }
}
