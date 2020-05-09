use core::fmt;

use crate::analyses::{DFG, Value};
use crate::control_flow;
use yaxpeax_arch::{Address, AddressDiff};

#[allow(dead_code)]
pub mod test_isa {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum Opcode {
        NOP,
        RETURN,
        SUB,
        CALL,
        ADD,
        JMP,
        JO,
        JNO,
        JS,
        JNS,
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum Operand {
        RegDeref(u8),
        Register(u8),
        ImmediateI8(i8),
        ImmediateU8(u8),
        ImmediateI32(i32),
        ImmediateU32(u32),
        ImmediateI64(i64),
        ImmediateU64(u64),
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    enum OperandCode {
        RegDeref,
        Register,
        ImmediateI8,
        ImmediateU8,
        ImmediateI32,
        ImmediateU32,
        ImmediateI64,
        ImmediateU64,
    }

    impl Operand {
        fn from_spec(inst: &Instruction, code: OperandCode) -> Operand {
            match code {
                OperandCode::RegDeref => {
                    Operand::RegDeref(inst.reg)
                }
                OperandCode::Register => {
                    Operand::Register(inst.reg)
                }
                OperandCode::ImmediateI8 => {
                    Operand::ImmediateI8(inst.imm as i8)
                }
                OperandCode::ImmediateU8 => {
                    Operand::ImmediateU8(inst.imm as u8)
                }
                OperandCode::ImmediateI32 => {
                    Operand::ImmediateI32(inst.imm as i32)
                }
                OperandCode::ImmediateU32 => {
                    Operand::ImmediateU32(inst.imm as u32)
                }
                OperandCode::ImmediateI64 => {
                    Operand::ImmediateI64(inst.imm as i64)
                }
                OperandCode::ImmediateU64 => {
                    Operand::ImmediateU64(inst.imm)
                }
            }
        }
    }

    pub struct Instruction {
        pub opcode: Opcode,
        operands: [OperandCode; 2],
        reg: u8,
        imm: u64,
    }

    impl Instruction {
        pub fn operand(&self, idx: u8) -> Operand {
            Operand::from_spec(&self, self.operands[idx as usize])
        }
    }
}
use test_isa::{Opcode, Operand, Instruction};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MemoryRegion(u16);

pub const ANY: MemoryRegion = MemoryRegion(0);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Location {
    Register(u8),
    Memory(MemoryRegion),
    // necessary to have a location to write that is provably not an Operand variant.
    // no x86 instruction explicitly writes to RIP in a way that could be ambiguous with other
    // operands, so this allows x86 semantics to specialize nicely for control flow.
    RIP,
}

pub(crate) fn evaluate<V: Value, D: DFG<V, Location=Location>>(instr: &Instruction, dfg: &mut D) {
    #[inline(always)]
    fn effective_address<V: Value, D: DFG<V, Location=Location>>(dfg: &mut D, operand: &Operand) -> V {
        match *operand {
            Operand::RegDeref(reg) => {
                dfg.read_loc(Location::Register(reg))
            }
            _ => {
                V::unknown()
            }
        }
    }

    #[inline(always)]
    fn read_jump_rel_operand<V: Value, D: DFG<V, Location=Location>>(_dfg: &mut D, operand: &Operand) -> V {
        match operand {
            Operand::ImmediateI8(imm) => {
                V::from_const(*imm as i64 as u64)
            }
            Operand::ImmediateI32(imm) => {
                V::from_const(*imm as i64 as u64)
            }
            _ => {
                unsafe {
                    core::hint::unreachable_unchecked();
                }
            }
        }
    }

    #[inline(always)]
    fn read_operand<V: Value, D: DFG<V, Location=Location>>(dfg: &mut D, operand: &Operand) -> V {
        match operand {
            Operand::Register(reg) => {
                dfg.read_loc(Location::Register(*reg))
            },
            Operand::ImmediateI8(imm) => {
                V::from_const(*imm as i64 as u64)
            }
            Operand::ImmediateU8(imm) => {
                V::from_const(*imm as u64)
            }
            Operand::ImmediateI64(imm) => {
                V::from_const(*imm as u64)
            }
            Operand::ImmediateU64(imm) => {
                V::from_const(*imm)
            }
            op => {
                let _ea = effective_address(dfg, op);
                let disambiguated = Location::Memory(ANY);
                dfg.read_loc(disambiguated)
            }
        }
    }

    fn jmp_tail<
        V: Value,
        D: DFG<V, Location=Location>,
    >(dfg: &mut D, instr: &Instruction) {
        let res = {
            V::from_set(&[
                dfg.read_loc(Location::RIP).add(&read_jump_rel_operand(dfg, &instr.operand(0))),
                dfg.read_loc(Location::RIP),
            ])
        };
        dfg.write_loc(Location::RIP, res);
    }

    match instr.opcode {
        Opcode::RETURN => {
            dfg.write_loc(Location::RIP, V::unknown());
        },
        Opcode::CALL => {
            let jump_target = read_operand(dfg, &instr.operand(0));
            dfg.write_loc(Location::RIP, jump_target);
        },
        Opcode::JMP => {
            let jump_target = read_operand(dfg, &instr.operand(0));
            dfg.write_loc(Location::RIP, jump_target);
        },
        Opcode::JO => {
            jmp_tail(
                dfg,
                instr,
            );
        },
        Opcode::JNO => {
            jmp_tail(
                dfg,
                instr,
            );
        },
        _ => {}
    };
}

pub struct ControlFlowAnalysis<A: Address + fmt::Debug> {
    pub effect: control_flow::Effect<A>,
}

impl <A: Address + fmt::Debug> ControlFlowAnalysis<A> {
    pub(crate) fn new() -> Self {
        Self {
            effect: control_flow::Effect::cont(),
        }
    }
}

pub trait ToAddrDiff: yaxpeax_arch::AddressDiffAmount {
    fn translate_offset(from: u64) -> AddressDiff<Self>;
}

impl ToAddrDiff for u64 {
    fn translate_offset(from: u64) -> AddressDiff<Self> {
        AddressDiff::from_const(from)
    }
}

impl <A: Address + ToAddrDiff + fmt::Debug> Value for control_flow::Effect<A> {
    fn unknown() -> Self {
        control_flow::Effect::stop()
    }

    fn from_const(c: u64) -> Self {
        control_flow::Effect::stop_and(
            control_flow::Target::Relative(A::translate_offset(c))
        )
    }

    fn from_set(effects: &[Self]) -> Self {
        use self::control_flow::Effect;
        use self::control_flow::Target;

        let mut stop_after = true;
        let mut target: Option<Target<A>> = None;

        for effect in effects {
            stop_after &= effect.is_stop();

            let merged_target = match (target, effect.dest.as_ref()) {
                (None, None) => {
                    None
                }
                (None, Some(o)) => {
                    Some(o.clone())
                }
                (Some(o), None) => {
                    Some(o)
                }
                _ => {
                    unsafe {
                        core::hint::unreachable_unchecked();
                    }
                }
            };
            target = merged_target;
        }

        Effect {
            stop_after,
            dest: target,
        }
    }

    fn to_const(&self) -> Option<u64> {
        None
    }

    #[inline(always)]
    fn add(&self, other: &Self) -> Self {
        if (self.stop_after == true && self.dest.is_none()) ||
            (other.stop_after == true && other.dest.is_none()) {

            return Self::unknown();
        }

        match (self.dest.as_ref(), other.dest.as_ref()) {
            (None, Some(control_flow::Target::Relative(rel))) |
            (Some(control_flow::Target::Relative(rel)), None) => {
                control_flow::Effect {
                    stop_after: self.stop_after || other.stop_after,
                    dest: Some(control_flow::Target::Relative(*rel))
                }
            },
            (Some(control_flow::Target::Relative(l)), Some(control_flow::Target::Relative(r))) => {
                control_flow::Effect {
                    stop_after: self.stop_after || other.stop_after,
                    dest: Some(control_flow::Target::Relative(
                        A::zero().wrapping_offset(*l).wrapping_offset(*r).diff(&A::zero()).unwrap_or_else(|| unsafe { core::hint::unreachable_unchecked() }) //.expect("can compute diff")
                    ))
                }
            }
            _ => {
                unsafe {
                    core::hint::unreachable_unchecked();
                    // panic!("bad add: {:?} + {:?}", self, other);
                }
            }
        }
    }
}

impl<Addr: Address + fmt::Debug + ToAddrDiff> DFG<control_flow::Effect<Addr>> for ControlFlowAnalysis<Addr> {
    type Location = Location;

    fn read_loc(&self, loc: Self::Location) -> control_flow::Effect<Addr> {
        if loc == Location::RIP {
            self.effect.clone()
        } else {
            control_flow::Effect::unknown()
        }
    }

    fn write_loc(&mut self, loc: Self::Location, value: control_flow::Effect<Addr>) {
        if loc == Location::RIP {
            self.effect = value;
        } else {
            // do nothing, it's a location we ignore for control flow analysis
        }
    }
}
