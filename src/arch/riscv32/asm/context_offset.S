# Context offset from task context structure

# General purpose registers offset
.set OFFSET_GP, 0
# Task address space hi and lo addr
.set OFFSET_ADDR_SPACE_HI, 128
.set OFFSET_ADDR_SPACE_LO, 132
# Program counter offset
.set OFFSET_PC, 136
# Stack pointer offset (will be set to OFFSET_ADDR_SPACE_HI at first if sp == 0)
.set OFFSET_SP, 140
# Return address
.set OFFSET_RA, 144
.set OFFSET_MSTATUS, 148
# Optionnal flags, will be use later maybe
.set OFFSET_FLAGS, 152
.set OFFSET_INSTRUCTION_REG, 155
