# ELF_06

## What it does

How the binary works:
- 1: Take a password as argument
- 2: hash password
- 3: compare to hardcoded hashed password

In the code we found the hashed password: "K+N.R.X+R1U-M,X%Q+U" (19 bytes)

hash function:
- Each byte of the password is hashed individually
- the way a byte is hashed depends in part on the size of the password
- there are a lot of collisions

## Solution

Reimplement hash function.

Brute force each byte of the password **one at the time**.

See solve.c for an implementation of the solution

One possible solution is "castatgaandjtrgppad".
