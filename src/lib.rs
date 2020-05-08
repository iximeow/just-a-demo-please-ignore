#![no_std]

mod analyses;
mod control_flow;
mod semantic;

#[no_mangle]
pub fn control_flow(inst: &crate::semantic::test_isa::Instruction) -> crate::control_flow::Effect<u64> {
    use crate::control_flow::Determinant;
    inst.control_flow(Option::<&()>::None)
}
