# ELF_antiDBG_01

opened with cutter

changed rax to 0 after ptrace call OR replace ptrace call with ```nop``` instruction.

found "lllloworldabcdef" in code, it goes through a compute() function to get the password.

Skip compute(), go to the strncmp and look at what the password is compared against.\
flag: "qgqgtrtmq_f]h_ja"
