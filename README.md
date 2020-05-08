if you `cargo build --release` this, even though lto is enabled this produces surprisingly lacking assembly for `read_jump_rel_operand`:

```
[0x00002980]> pd 100 @ sym.read_jump_rel_operand
            ;-- read_jump_rel_operand:
            0x00002a60      50             push rax
            0x00002a61      4080fe04       cmp sil, 4
        ,=< 0x00002a65      0f83cf000000   jae 0x2b3a
        |   0x00002a6b      400fb6c6       movzx eax, sil
        |   0x00002a6f      0fb6440720     movzx eax, byte [rdi + rax + 0x20] ; [0x20:1]=64 ; "@"
        |   0x00002a74      31c9           xor ecx, ecx
        |   0x00002a76      488d15a3ef01.  lea rdx, qword [0x00021a20] ; section..rodata
        |   0x00002a7d      48633482       movsxd rsi, dword [rdx + rax*4]
        |   0x00002a81      4801d6         add rsi, rdx                ; '('
        |   0x00002a84      31c0           xor eax, eax
        |   0x00002a86      31d2           xor edx, edx
        |   0x00002a88      ffe6           jmp rsi
        |   0x00002a8a      8a4717         mov al, byte [rdi + 0x17]   ; [0x17:1]=0
        |   0x00002a8d      31c9           xor ecx, ecx
       ,==< 0x00002a8f      e986000000     jmp 0x2b1a
       ||   0x00002a94      8a4717         mov al, byte [rdi + 0x17]   ; [0x17:1]=0
       ||   0x00002a97      448a471a       mov r8b, byte [rdi + 0x1a]  ; [0x1a:1]=0
       ||   0x00002a9b      31c9           xor ecx, ecx
      ,===< 0x00002a9d      eb7b           jmp 0x2b1a
      |||   0x00002a9f      8a4717         mov al, byte [rdi + 0x17]   ; [0x17:1]=0
      |||   0x00002aa2      448a471a       mov r8b, byte [rdi + 0x1a]  ; [0x1a:1]=0
      |||   0x00002aa6      448a4f1d       mov r9b, byte [rdi + 0x1d]  ; [0x1d:1]=0
      |||   0x00002aaa      31c9           xor ecx, ecx
     ,====< 0x00002aac      eb6c           jmp 0x2b1a
     ||||   0x00002aae      4c8b07         mov r8, qword [rdi]
    ,=====< 0x00002ab1      eb04           jmp 0x2ab7
    |||||   0x00002ab3      4c8b4708       mov r8, qword [rdi + 8]     ; [0x8:8]=0
    `-----> 0x00002ab7      4589c1         mov r9d, r8d
     ||||   0x00002aba      41c1e908       shr r9d, 8
     ||||   0x00002abe      4489c1         mov ecx, r8d
     ||||   0x00002ac1      81e10000ffff   and ecx, 0xffff0000
     ||||   0x00002ac7      31d2           xor edx, edx
    ,=====< 0x00002ac9      eb51           jmp 0x2b1c
    |||||   0x00002acb      b001           mov al, 1
    |||||   0x00002acd      31c9           xor ecx, ecx
   ,======< 0x00002acf      eb49           jmp 0x2b1a
   ||||||   0x00002ad1      8a07           mov al, byte [rdi]
   ||||||   0x00002ad3      b201           mov dl, 1
  ,=======< 0x00002ad5      eb45           jmp 0x2b1c
  |||||||   0x00002ad7      8a4715         mov al, byte [rdi + 0x15]   ; [0x15:1]=0
  |||||||   0x00002ada      31c9           xor ecx, ecx
  ========< 0x00002adc      eb3c           jmp 0x2b1a
  |||||||   0x00002ade      8a471b         mov al, byte [rdi + 0x1b]   ; [0x1b:1]=0
  |||||||   0x00002ae1      31c9           xor ecx, ecx
  ========< 0x00002ae3      eb35           jmp 0x2b1a
  |||||||   0x00002ae5      8a07           mov al, byte [rdi]
  |||||||   0x00002ae7      31c9           xor ecx, ecx
  ========< 0x00002ae9      eb2f           jmp 0x2b1a
  |||||||   0x00002aeb      b006           mov al, 6
  |||||||   0x00002aed      31c9           xor ecx, ecx
  ========< 0x00002aef      eb29           jmp 0x2b1a
  |||||||   0x00002af1      b007           mov al, 7
  |||||||   0x00002af3      31c9           xor ecx, ecx
  ========< 0x00002af5      eb23           jmp 0x2b1a
  |||||||   0x00002af7      8a4719         mov al, byte [rdi + 0x19]   ; [0x19:1]=41
  |||||||   0x00002afa      31c9           xor ecx, ecx
  ========< 0x00002afc      eb1c           jmp 0x2b1a
  |||||||   0x00002afe      8a4717         mov al, byte [rdi + 0x17]   ; [0x17:1]=0
  ========< 0x00002b01      eb03           jmp 0x2b06
  |||||||   0x00002b03      8a4719         mov al, byte [rdi + 0x19]   ; [0x19:1]=41
  --------> 0x00002b06      4c8b4708       mov r8, qword [rdi + 8]     ; [0x8:8]=0
  |||||||   0x00002b0a      4589c1         mov r9d, r8d
  |||||||   0x00002b0d      41c1e908       shr r9d, 8
  |||||||   0x00002b11      4489c1         mov ecx, r8d
  |||||||   0x00002b14      81e10000ffff   and ecx, 0xffff0000
  -`-```--> 0x00002b1a      31d2           xor edx, edx
  `-`-----> 0x00002b1c      410fb6f1       movzx esi, r9b
        |   0x00002b20      c1e608         shl esi, 8
        |   0x00002b23      410fb6f8       movzx edi, r8b
        |   0x00002b27      09cf           or edi, ecx
        |   0x00002b29      09f7           or edi, esi
        |   0x00002b2b      4863cf         movsxd rcx, edi
        |   0x00002b2e      480fbec0       movsx rax, al
        |   0x00002b32      84d2           test dl, dl
        |   0x00002b34      480f44c1       cmove rax, rcx
        |   0x00002b38      59             pop rcx
        |   0x00002b39      c3             ret
        `-> 0x00002b3a      488d3db61102.  lea rdi, qword [0x00023cf7] ; "assertion failed: i < 4failed to read executable information"
            0x00002b41      488d15387022.  lea rdx, qword [0x00229b80]
            0x00002b48      be17000000     mov esi, 0x17
            0x00002b4d      e80e1f0000     call sym.core::panicking::panic::h3a82ab1d0243e74d
            0x00002b52      0f0b           ud2
```

when `read_jump_rel_operand` is added to `yaxpeax_x86/ffi`, this same function builds with much more reasonable output:
```
[0x00005430]> pdf @ sym.read_jump_rel_operand
/ (fcn) sym.read_jump_rel_operand 24
|   sym.read_jump_rel_operand ();
|           0x00012d70      400fb6c6       movzx eax, sil
|           0x00012d74      807c072006     cmp byte [rdi + rax + 0x20], 6 ; [0x6:1]=1
|           0x00012d79      488b07         mov rax, qword [rdi]
|           0x00012d7c      4863c8         movsxd rcx, eax
|           0x00012d7f      480fbec0       movsx rax, al
|           0x00012d83      480f45c1       cmovne rax, rcx
\           0x00012d87      c3             ret
```

what????

the large switch above has 26 cases. there are not 26 cases in the function! or are there? `inst.operand(idx)`is a call to `Instruction::operand` in `yaxpeax_x86`. on the version used here, `0.0.11`, that's written like:
```rust
pub fn operand(&self, i: u8) -> Operand {
    assert!(i < 4);
    Operand::from_spec(self, self.operands[i as usize])
}
```

this is where the panic at the end of the first function comes from. the 26-case match, though, comes from `Operand::from_spec`:
```rust
#[inline]
fn from_spec(inst: &Instruction, spec: OperandSpec) -> Operand {
    match spec {
        OperandSpec::Nothing => {
            Operand::Nothing
        }
        // the register in modrm_rrr
        OperandSpec::RegRRR => {
            Operand::Register(inst.modrm_rrr)
        }
        // the register in modrm_mmm (eg modrm mod bits were 11)
        OperandSpec::RegMMM => {
            Operand::Register(inst.modrm_mmm)
        }
        OperandSpec::RegVex => {
            Operand::Register(inst.vex_reg)
        }
        OperandSpec::AL => {
            Operand::Register(RegSpec::al())
        }
        OperandSpec::CL => {
            Operand::Register(RegSpec::cl())
        }
        OperandSpec::ImmI8 => Operand::ImmediateI8(inst.imm as i8),
        OperandSpec::ImmU8 => Operand::ImmediateU8(inst.imm as u8),
        OperandSpec::ImmI16 => Operand::ImmediateI16(inst.imm as i16),
        OperandSpec::ImmU16 => Operand::ImmediateU16(inst.imm as u16),
        OperandSpec::ImmI32 => Operand::ImmediateI32(inst.imm as i32),
        OperandSpec::ImmU32 => Operand::ImmediateU32(inst.imm as u32),
        OperandSpec::ImmI64 => Operand::ImmediateI64(inst.imm as i64),
        OperandSpec::ImmU64 => Operand::ImmediateU64(inst.imm as u64),
        OperandSpec::DispU32 => Operand::DisplacementU32(inst.disp as u32),
        OperandSpec::DispU64 => Operand::DisplacementU64(inst.disp as u64),
        OperandSpec::Deref => {
            Operand::RegDeref(inst.modrm_mmm)
        }
        OperandSpec::Deref_rsi => {
            Operand::RegDeref(RegSpec::rsi())
        }
        OperandSpec::Deref_rdi => {
            Operand::RegDeref(RegSpec::rdi())
        }
        OperandSpec::RegDisp => {
            Operand::RegDisp(inst.modrm_mmm, inst.disp as i32)
        }
        OperandSpec::RegScale => {
            Operand::RegScale(inst.sib_index, inst.scale)
        }
        OperandSpec::RegIndexBase => {
            Operand::RegIndexBase(inst.modrm_mmm, inst.sib_index)
        }
        OperandSpec::RegIndexBaseDisp => {
            Operand::RegIndexBaseDisp(inst.modrm_mmm, inst.sib_index, inst.disp as i32)
        }
        OperandSpec::RegScaleDisp => {
            Operand::RegScaleDisp(inst.sib_index, inst.scale, inst.disp as i32)
        }
        OperandSpec::RegIndexBaseScale => {
            Operand::RegIndexBaseScale(inst.modrm_mmm, inst.sib_index, inst.scale)
        }
        OperandSpec::RegIndexBaseScaleDisp => {
            Operand::RegIndexBaseScaleDisp(inst.modrm_mmm, inst.sib_index, inst.scale, inst.disp as i32)
        }
    }
}
```
some arms line up here:
* `0x00002aae      4c8b07         mov r8, qword [rdi] ` is likely reading `inst.disp` for displacement variants
* `0x00002aae      4c8b07         mov r8, qword [rdi] ` and `0x00002ad1      8a07           mov al, byte [rdi]` line up with multiple ways by which `imm` is read
* `0x00002af1      b007           mov al, 7 ` is probably preparing the register number (7) for `OperandSpec::Deref_rdi`
and so on

so even though only two operand arms are actually read, the entirety of `Operand::from_spec` has been inlined and no further analysis has been done..?

