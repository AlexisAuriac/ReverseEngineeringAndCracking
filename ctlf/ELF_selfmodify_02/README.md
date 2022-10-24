# ELF_selfmodify_02

## What it does

changes the protection of a part of the program so that it can be written onto (mprotect), this part contains 2 functions with a lot of invalid instructions

xors this section with another section, patching all errors

checks pwd is lowercase only

it uses ptrace to check if the program is being debugged (see ELF_antiDBG_01)

uses custom hashing function to unhash !&{!!t$&&p#&t $$ww!v'pr# {ur{qs$!&!# #rztq$u{&{q$#vw{##p'$qt!vqp
It becomes: cd9cc6fdd2ad6bff55c4e20ab970931fcdcaba0863f79d93fa459aa2ef36c432

hash pwd with sha256

compares pwd with: cd9cc6fdd2ad6bff55c4e20ab970931fcdcaba0863f79d93fa459aa2ef36c432

## Solution

https://md5decrypt.net/en/Sha256/
flag: bananapower
