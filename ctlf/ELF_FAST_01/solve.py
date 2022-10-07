#!/bin/env python3

from pwn import *

def getSetionData(elf):
	for i in elf.sections:
		if b'You should give one argument.' in i.data():
			return i.data()

def main():
	elf = ELF('./ELF_FAST_01')
	section = getSetionData(elf)

	pwd = section[34:54]
	print(pwd.decode())

if __name__ == '__main__':
	main()
