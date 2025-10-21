ARM_CC ?= arm-none-eabi-gcc
ARM_AS ?= arm-none-eabi-as
ARM_LD ?= arm-none-eabi-ld
ARM_OJBCPY ?= arm-none-eabi-objcopy

SRC_DIR = asmsrc
BUILD_DIR = build
OUT_NAME = program
LD_SCRIPT = linker.ld

ASM_SRC_FILES = $(SRC_DIR)/main.s
C_SRC_FILES = $(SRC_DIR)/c.c
OBJ_FILES = $(patsubst $(SRC_DIR)/%.s,$(BUILD_DIR)/%.o,$(ASM_SRC_FILES))
OBJ_FILES += $(patsubst $(SRC_DIR)/%.c,$(BUILD_DIR)/%.o,$(C_SRC_FILES))

.PHONY: clean run
run: $(BUILD_DIR)/$(OUT_NAME)
	xxd -b $(BUILD_DIR)/$(OUT_NAME)

$(BUILD_DIR)/$(OUT_NAME): $(OBJ_FILES) $(LD_SCRIPT) | $(BUILD_DIR)
	$(ARM_LD) $(OBJ_FILES) -o $(BUILD_DIR)/$(OUT_NAME).elf -T $(LD_SCRIPT)
	# Move out the text section to a binary file
	$(ARM_OJBCPY) -O binary $(BUILD_DIR)/$(OUT_NAME).elf $@

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.s | $(BUILD_DIR)
	$(ARM_AS) $< -o $@

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | $(BUILD_DIR)
	$(ARM_CC) $< -o $@-asm.s -march=armv6-m -mtune=cortex-m0 -mthumb -nostdlib -Os -S
	$(ARM_CC) $< -o $@ -march=armv6-m -mtune=cortex-m0 -mthumb -nostdlib -Os -c

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

clean:
	rm $(BUILD_DIR) -rf
