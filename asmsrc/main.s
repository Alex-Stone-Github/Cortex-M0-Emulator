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
_incit:
	adds r0, #1
	b _incit
	b .

.end
