if you `cargo build --release` this, even though lto is enabled this produces surprisingly lacking assembly for `control_flow`:

```
	.section	.text.control_flow,"ax",@progbits
	.globl	control_flow
	.p2align	4, 0x90
	.type	control_flow,@function
control_flow:
.Lfunc_begin0:
	.cfi_startproc
	subq	$16, %rsp
	.cfi_def_cfa_offset 24
	movq	%rdi, %rax
	movb	8(%rsi), %cl
	movl	$4, %r9d
	xorl	%edi, %edi
	addb	$-1, %cl
	cmpb	$6, %cl
	ja	.LBB0_12
	movq	(%rsi), %r8
	movb	9(%rsi), %dl
	movzbl	%cl, %ecx
	leaq	.LJTI0_0(%rip), %rsi
	movslq	(%rsi,%rcx,4), %rcx
	addq	%rsi, %rcx
	jmpq	*%rcx
.LBB0_2:
	movb	$1, %dil
	jmp	.LBB0_16
.LBB0_3:
	movl	$4, %r9d
	movzbl	%dl, %ecx
	leaq	.LJTI0_4(%rip), %rdx
	movslq	(%rdx,%rcx,4), %rcx
	addq	%rdx, %rcx
	jmpq	*%rcx
.LBB0_5:
	movl	$4, %r9d
	movzbl	%dl, %ecx
	leaq	.LJTI0_3(%rip), %rdx
	movslq	(%rdx,%rcx,4), %rcx
	addq	%rdx, %rcx
	jmpq	*%rcx
.LBB0_4:
	movsbq	%r8b, %r8
	jmp	.LBB0_14
.LBB0_7:
	xorl	%esi, %esi
	movzbl	%dl, %ecx
	leaq	.LJTI0_2(%rip), %rdx
	movslq	(%rdx,%rcx,4), %rcx
	addq	%rdx, %rcx
	jmpq	*%rcx
.LBB0_8:
	xorl	%esi, %esi
	movzbl	%dl, %ecx
	leaq	.LJTI0_1(%rip), %rdx
	movslq	(%rdx,%rcx,4), %rcx
	addq	%rdx, %rcx
	jmpq	*%rcx
.LBB0_9:
	movq	%r8, %rsi
.LBB0_10:
	shlq	$32, %rsi
	movl	$32, %ecx
.LBB0_11:
	sarq	%cl, %rsi
	movl	9(%rsp), %ecx
	movl	12(%rsp), %edx
	movl	%edx, 3(%rsp)
	movl	%ecx, (%rsp)
	xorl	%edi, %edi
	movq	%rsi, %r8
	xorl	%r9d, %r9d
	jmp	.LBB0_16
.LBB0_12:
	jmp	.LBB0_16
.LBB0_13:
	movzbl	%r8b, %r8d
.LBB0_14:
	xorl	%r9d, %r9d
.LBB0_15:
	movl	9(%rsp), %ecx
	movl	12(%rsp), %edx
	movl	%edx, 3(%rsp)
	movl	%ecx, (%rsp)
	movb	$1, %dil
.LBB0_16:
	movq	%r9, (%rax)
	movq	%r8, 8(%rax)
	movb	%dil, 16(%rax)
	movl	(%rsp), %ecx
	movl	3(%rsp), %edx
	movl	%ecx, 17(%rax)
	movl	%edx, 20(%rax)
	addq	$16, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB0_17:
	.cfi_def_cfa_offset 24
	shlq	$56, %r8
	movl	$56, %ecx
	movq	%r8, %rsi
	jmp	.LBB0_11
.Lfunc_end0:
	.size	control_flow, .Lfunc_end0-control_flow
	.cfi_endproc
	.section	.rodata.control_flow,"a",@progbits
	.p2align	2
.LJTI0_0:
	.long	.LBB0_2-.LJTI0_0
	.long	.LBB0_16-.LJTI0_0
	.long	.LBB0_3-.LJTI0_0
	.long	.LBB0_16-.LJTI0_0
	.long	.LBB0_5-.LJTI0_0
	.long	.LBB0_7-.LJTI0_0
	.long	.LBB0_8-.LJTI0_0
.LJTI0_1:
	.long	.LBB0_10-.LJTI0_1
	.long	.LBB0_10-.LJTI0_1
	.long	.LBB0_17-.LJTI0_1
	.long	.LBB0_10-.LJTI0_1
	.long	.LBB0_9-.LJTI0_1
	.long	.LBB0_9-.LJTI0_1
	.long	.LBB0_10-.LJTI0_1
	.long	.LBB0_10-.LJTI0_1
.LJTI0_2:
	.long	.LBB0_10-.LJTI0_2
	.long	.LBB0_10-.LJTI0_2
	.long	.LBB0_17-.LJTI0_2
	.long	.LBB0_10-.LJTI0_2
	.long	.LBB0_9-.LJTI0_2
	.long	.LBB0_9-.LJTI0_2
	.long	.LBB0_10-.LJTI0_2
	.long	.LBB0_10-.LJTI0_2
.LJTI0_3:
	.long	.LBB0_15-.LJTI0_3
	.long	.LBB0_15-.LJTI0_3
	.long	.LBB0_4-.LJTI0_3
	.long	.LBB0_13-.LJTI0_3
	.long	.LBB0_15-.LJTI0_3
	.long	.LBB0_15-.LJTI0_3
	.long	.LBB0_14-.LJTI0_3
	.long	.LBB0_14-.LJTI0_3
.LJTI0_4:
	.long	.LBB0_15-.LJTI0_4
	.long	.LBB0_15-.LJTI0_4
	.long	.LBB0_4-.LJTI0_4
	.long	.LBB0_13-.LJTI0_4
	.long	.LBB0_15-.LJTI0_4
	.long	.LBB0_15-.LJTI0_4
	.long	.LBB0_14-.LJTI0_4
	.long	.LBB0_14-.LJTI0_4
```

what????

`LJTI0_1` and `LJTI0_2` are identical. the blocks leading to them:
```
.LBB0_7:
	xorl	%esi, %esi
	movzbl	%dl, %ecx
	leaq	.LJTI0_2(%rip), %rdx
	movslq	(%rdx,%rcx,4), %rcx
	addq	%rdx, %rcx
	jmpq	*%rcx
.LBB0_8:
	xorl	%esi, %esi
	movzbl	%dl, %ecx
	leaq	.LJTI0_1(%rip), %rdx
	movslq	(%rdx,%rcx,4), %rcx
	addq	%rdx, %rcx
	jmpq	*%rcx
```

are also identical. these seem to line up with the `JO` and `JNO` cases in the match at `src/semantic.rs:175`

the same holds for `JMP` and `CALL` through `LJTI0_3` and `LJTI0_4`, and the blocks leading to them.

in both cases the arms are duplicate and rustc could/should fold them together to reduce space usage at no overhead. why not???
