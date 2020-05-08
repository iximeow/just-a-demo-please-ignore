use yaxpeax_x86::long_mode::{Arch as amd64};
use yaxpeax_x86::long_mode::Operand;
use yaxpeax_arch::Arch;

#[no_mangle]
pub fn read_jump_rel_operand(inst: &<amd64 as Arch>::Instruction, idx: u8) -> u64 {
    match inst.operand(idx) {
        Operand::ImmediateI8(imm) => {
            imm as i64 as u64
        }
        Operand::ImmediateI32(imm) => {
            imm as i64 as u64
        }
        _ => {
            unsafe {
                std::hint::unreachable_unchecked();
            }
        }
    }
}
