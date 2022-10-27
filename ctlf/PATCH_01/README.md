# PATCH_01

## What we know

The program needs to be patched.

We need a md5 of the file
-> The difference must be really small

## Brute force solution (bit flipping)

The original file is 13,756 bytes long.

Flip 1 bit in the file and see if it is the solution. Repeat for every bit in the file (110,048 attempts).

Took 7m33s to complete.

By flipping bit 4 of byte 4284 we get:
Received message: send_help_pls!

md5: ```1db328070f31c4c97c9569122ae2c729```
