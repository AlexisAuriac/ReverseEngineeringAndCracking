# ELF_antiDBG_02

## What the program does

When opened with a debugger it compares the password with '=sCc7_InB_9t5nB_D_C_D'.

This will not work when the program is run without a debugger.

The program uses the SIGTRAP handler technique: https://subscription.packtpub.com/book/networking-and-servers/9781782167105/4/ch04lvl1sec39/elf-anti-debugging-and-packing-techniques

Basically, it defines an handler for the SIGTRAP signal (called the program encounters a breakpoint) and has defined a breakpoint in its code. When run with a debugger, the debugger will overwrite the handler and the handler will not be called at the breakpoint.

The SIGTRAP handler hashes the given password with a custom hashing function, similarly to ELF_06 and ELF_07.

## Solution

Same as ELF_06, implement the hashing function and bruteforce 1 character at the time.

### Debugging

To debug the hash implementation, you can run the program with the debugger until the program-defined breakpoint and then change the RIP to the start of the SIGTRAP handler.
