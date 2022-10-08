# ELF_antiDBG_01

opened with cutter

changed rax to 0 after ptrace call

found lllloworldabcdef in code, it goes through a "compute" function to get the password.

after compute() call, looked around in heap until found flag
