#!/bin/env python3

from pwn import *

startSecret1 = 297
startSecret2 = startSecret1 + 10
startMask = startSecret1 + 35

def main():
	elf = ELF('./ELF_FAST_03')
	section = elf.section('.text')

	secret = section[startSecret1:startSecret1 + 8] + section[startSecret2:startSecret2 + 8]
	mask = 0x0503020302.to_bytes(5, 'little')

	password = bytearray(16)
	password[5:] = secret[5:]
	for i in range(5):
		password[i] = secret[i] + mask[i]

	print(secret)
	print(mask)
	print(password)
	print(str(password.decode()))

if __name__ == '__main__':
	main()
