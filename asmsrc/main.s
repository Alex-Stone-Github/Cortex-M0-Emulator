.syntax unified
.cpu cortex-m0
.thumb
.text

.global _start
.global my_four

_start:                      // pretty much: nop
	b _next
_next:                       // my_three: *u8 = static_alloc(1);
	movs r0, #44
	ldr r3, =my_three
_repeat:                     // while (1) *my_three += 2;
	adds r0, 2
	strb r0, [r3]
	cmp r0, #50
	bne _repeat
	bl centry                // Call C Code
	bl theend                // call more c code(for spec region)
	b .                      // Halt
my_three:
	.byte 3
my_four:
	.byte 4

.data
.ascii "Data Section(gotta love it)"

