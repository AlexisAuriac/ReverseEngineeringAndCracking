# KILL_LA_KOOL

Open http://reverse.blackfoot.io:1331/ with google chrome.\
Go to *Sources*.\
Open ```index.wasm```.

## What it does

When a password is entered it will be check by the verify password function.

It does some operations on ```!0#{W}R7_s30u__?_nyd```.

It compares the result with the password we gave it.

## Solution

Put a breakpoint on the line strcmp is called (```0x00ef```)

Run with the debugger until that line

The address to the modified password (the flag) will be in $var1, in Local.

Go to Module.memories.$a.memory

Click on the little icon next to it

It will open the memory inspector

Go to the address that was stored in ```$var1```, the flag will be there.
