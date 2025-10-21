.syntax unified
.cpu cortex-m0
.thumb
.section .text

.global _start

_start:
	movs r0, #42  
	movs r0, #43
	movs r1, #44
	movs r2, #45
	movs r2, #46
	movs r2, #47
	movs r2, #100
_incit:
	adds r2, #1
	b _incit
	b .

.end
