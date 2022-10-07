#!/bin/env python3

from pwn import *

def getSetionData(elf):
	for i in elf.sections:
		if b'You should give one argument.' in i.data():
			return i.data()

def getPwd():
	elf = ELF('./ELF_FAST_02')
	section = getSetionData(elf)

	pwd = section[79:99]
	return pwd.decode()

def main():
	pwd = getPwd()
	flag = ""

	for (i, c) in enumerate(pwd):
		flag += chr(ord(c) - 1)
	print(flag)

if __name__ == '__main__':
	main()
