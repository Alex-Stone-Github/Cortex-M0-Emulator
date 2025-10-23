	.arch armv6-m
	.fpu softvfp
	.eabi_attribute 20, 1
	.eabi_attribute 21, 1
	.eabi_attribute 23, 3
	.eabi_attribute 24, 1
	.eabi_attribute 25, 1
	.eabi_attribute 26, 1
	.eabi_attribute 30, 4
	.eabi_attribute 34, 0
	.eabi_attribute 18, 4
	.file	"c.c"
	.text
	.align	1
	.global	square
	.syntax unified
	.code	16
	.thumb_func
	.type	square, %function
square:
	@ args = 0, pretend = 0, frame = 0
	@ frame_needed = 0, uses_anonymous_args = 0
	@ link register save eliminated.
	ldr	r3, .L2
	muls	r0, r0
	@ sp needed
	ldrb	r2, [r3]
	adds	r2, r2, #1
	strb	r2, [r3]
	ldrb	r2, [r3]
	subs	r2, r2, #1
	strb	r2, [r3]
	bx	lr
.L3:
	.align	2
.L2:
	.word	my_four
	.size	square, .-square
	.align	1
	.global	centry
	.syntax unified
	.code	16
	.thumb_func
	.type	centry, %function
centry:
	@ args = 0, pretend = 0, frame = 0
	@ frame_needed = 0, uses_anonymous_args = 0
	movs	r3, #5
	push	{r4, r5, r6, lr}
	ldr	r4, .L7
	ldr	r5, .L7+4
	strb	r3, [r4]
	movs	r3, #0
	ldr	r1, [r5]
.L5:
	cmp	r1, r3
	bgt	.L6
	@ sp needed
	movs	r0, #4
	bl	square
	ldr	r3, [r5]
	adds	r3, r3, r0
	movs	r0, #4
	str	r3, [r5]
	bl	square
	ldrb	r3, [r4]
	adds	r3, r3, r0
	strb	r3, [r4]
	pop	{r4, r5, r6, pc}
.L6:
	ldrb	r2, [r4]
	adds	r3, r3, #1
	adds	r2, r2, #2
	strb	r2, [r4]
	b	.L5
.L8:
	.align	2
.L7:
	.word	my_four
	.word	.LANCHOR0
	.size	centry, .-centry
	.align	1
	.global	printserial
	.syntax unified
	.code	16
	.thumb_func
	.type	printserial, %function
printserial:
	@ args = 0, pretend = 0, frame = 0
	@ frame_needed = 0, uses_anonymous_args = 0
	push	{r4, lr}
	movs	r4, #150
	adds	r1, r0, r1
	lsls	r4, r4, #1
.L10:
	ldrb	r3, [r0]
	cmp	r3, #0
	beq	.L9
	subs	r2, r1, r0
	cmp	r2, #0
	bgt	.L12
.L9:
	@ sp needed
	pop	{r4, pc}
.L12:
	strb	r3, [r4]
	adds	r0, r0, #1
	b	.L10
	.size	printserial, .-printserial
	.section	.rodata.str1.1,"aMS",%progbits,1
.LC3:
	.ascii	"Alex was here!!\000"
	.text
	.align	1
	.global	theend
	.syntax unified
	.code	16
	.thumb_func
	.type	theend, %function
theend:
	@ args = 0, pretend = 0, frame = 0
	@ frame_needed = 0, uses_anonymous_args = 0
	movs	r1, #250
	push	{r4, lr}
	ldr	r0, .L15
	lsls	r1, r1, #1
	bl	printserial
	@ sp needed
	movs	r1, #4
	ldr	r0, .L15
	bl	printserial
	movs	r1, #250
	ldr	r0, .L15
	lsls	r1, r1, #1
	bl	printserial
	pop	{r4, pc}
.L16:
	.align	2
.L15:
	.word	.LC3
	.size	theend, .-theend
	.global	garbage
	.section	.rodata.str1.1
.LC5:
	.ascii	"HIC\000"
	.global	number
	.data
	.align	2
	.set	.LANCHOR0,. + 0
	.type	number, %object
	.size	number, 4
number:
	.word	5
	.type	garbage, %object
	.size	garbage, 4
garbage:
	.word	.LC5
	.ident	"GCC: (Fedora 15.1.0-1.fc42) 15.1.0"
