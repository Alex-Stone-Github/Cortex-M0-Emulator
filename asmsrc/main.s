.syntax unified
.cpu cortex-m0
.thumb

.global _start

_start:
	movs r0, #42    @Load r2 into cpu.r[0]
	b _start
