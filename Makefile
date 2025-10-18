ARM_AS ?= arm-none-eabi-as
ARM_LD ?= arm-none-eabi-ld
ARM_OJBCPY ?= arm-none-eabi-objcopy

BUILDDIR = build

.PHONY: all clean

all: | $(BUILDDIR)
	$(ARM_AS) -mcpu=cortex-m0 -mthumb asmsrc/main.s -o $(BUILDDIR)/main.o
	# My programs start @ 0x0
	$(ARM_LD) -Ttext=0x0 $(BUILDDIR)/main.o -o $(BUILDDIR)/main.elf
	# Move out the text section to a binary file
	$(ARM_OJBCPY) -O binary $(BUILDDIR)/main.elf $(BUILDDIR)/main.bin


$(BUILDDIR):
	mkdir -p $(BUILDDIR)

clean:
	rm $(BUILDDIR) -rf
