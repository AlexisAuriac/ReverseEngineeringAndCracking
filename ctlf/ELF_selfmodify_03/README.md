# ELF_selfmodify_03

## What it does

Modifies itself like ELF_selfmodify_02.

Uses the SIGTRAP handler method to prevent debugging (see ELF_antiDBG_02).
More specifically it uses that method to create infinite loops.

hash pwd with sha256

compares with: ```891b3a6e61d3c71afc3190f9a5718f282749b53e3e064299c636587ce60722ce```

## Solution

Skip infinite loops and other anti-debug methods by changing the RIP when needed.

Used an online hash db: https://md5decrypt.net/en/Sha256/

flag: LONGTUNEL
