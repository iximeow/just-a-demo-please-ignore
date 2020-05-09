#![no_std]

mod analyses;
mod control_flow;
mod semantic;

#[no_mangle]
pub fn control_flow(inst: &crate::semantic::test_isa::Instruction) -> crate::control_flow::Effect<u64> {
    use crate::semantic::ControlFlowAnalysis;
    let mut instr_control_flow = ControlFlowAnalysis::new();
    crate::semantic::evaluate(inst, &mut instr_control_flow);
    instr_control_flow.effect
}
