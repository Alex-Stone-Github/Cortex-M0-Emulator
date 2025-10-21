# Cortex-M0-Emulator

Implementation of the armv6m architecture for cortex m0.

# How to use

This project is not in a working stage yet. Not all instructions work properly
yet, but if you wish to play around the current source file is
`./asmsrc/main.s`. If you still wish to run the development version, use the
following shell commands sequentially:

```sh
make
cargo r
```

This program now also links with c code, and you can edit the file
`./asmsrc/c.c` and modify the function. It should be able to call other
functions as well if you setup the stack pointer.

```c
extern volatile char my_four;

char* garbage = "HIC";

void centry() {
	my_four = 5;
	for (int i = 0; i < 5; i ++) {
		my_four += 2;
	}
	return;
}
```


# Compatability

This emulator can only run on lsb data access host machines, making it
architecture specific.

# TODO

- Memory address implementation
- Interrupts
- Test all instructions
