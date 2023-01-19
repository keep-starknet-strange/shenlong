	.text
	.file	"root"
	.globl	felt_add                # -- Begin function felt_add
	.p2align	4, 0x90
	.type	felt_add,@function
felt_add:                               # @felt_add
	.cfi_startproc
# %bb.0:                                # %entry
	movq	%rdi, %rax
	addq	%r9, %rsi
	adcq	8(%rsp), %rdx
	adcq	16(%rsp), %rcx
	adcq	24(%rsp), %r8
	movq	%rdx, 8(%rdi)
	movq	%rsi, (%rdi)
	movq	%rcx, 16(%rdi)
	movabsq	$1152921504606846975, %rcx # imm = 0xFFFFFFFFFFFFFFF
	andq	%r8, %rcx
	movq	%rcx, 24(%rdi)
	retq
.Lfunc_end0:
	.size	felt_add, .Lfunc_end0-felt_add
	.cfi_endproc
                                        # -- End function
	.globl	main                    # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbx
	.cfi_def_cfa_offset 16
	subq	$32, %rsp
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -16
	movq	%rdi, %rbx
	subq	$8, %rsp
	.cfi_adjust_cfa_offset 8
	leaq	8(%rsp), %rdi
	movl	$34, %esi
	movl	$3, %r9d
	movl	$0, %edx
	movl	$0, %ecx
	movl	$0, %r8d
	pushq	$0
	.cfi_adjust_cfa_offset 8
	pushq	$0
	.cfi_adjust_cfa_offset 8
	pushq	$0
	.cfi_adjust_cfa_offset 8
	callq	felt_add
	addq	$32, %rsp
	.cfi_adjust_cfa_offset -32
	movq	(%rsp), %rax
	movq	8(%rsp), %rcx
	movq	24(%rsp), %rdx
	movq	16(%rsp), %rsi
	movq	%rsi, 16(%rbx)
	movq	%rdx, 24(%rbx)
	movq	%rax, (%rbx)
	movq	%rcx, 8(%rbx)
	movq	%rbx, %rax
	addq	$32, %rsp
	.cfi_def_cfa_offset 16
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
