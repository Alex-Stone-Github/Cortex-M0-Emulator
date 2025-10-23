# Cortex-M0-Emulator

Implementation of the armv6m architecture for cortex m0.


## Usage

To run this project, use the following commands:

```sh
make
cargo r
```

Checkout [[Configuration]]

## Configuration

Configuration is done in the `config.lua` file. The format is mostly self
explanitory. You define a table of memory regions that have specific
functiosn(eg file load, ram, or a custom lua script).

## Compatability

This emulator can only run on lsb data access host machines, making it
architecture specific.

## TODO List
- Interrupts
- Low Power Modes
- Internal Registers

## References

- [ARMv6-M Reference Manual](https://users.ece.utexas.edu/~valvano/mspm0/Arm_Architecture_v6m_Reference_Manual.pdf)
