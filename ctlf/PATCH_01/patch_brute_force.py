#!/bin/env python3

import subprocess
import hashlib
import shutil

PROG_TO_PATCH = './patch_1'
TEST_PATCHED = './test_patched'

# Open log file
logs = open('logs.txt', mode='a')

# Get original bin
f = open(PROG_TO_PATCH, mode='rb')
elfBytes = f.read()
f.close()
elfByteArray = bytearray(elfBytes)

# Initialize test patched file
shutil.copy(PROG_TO_PATCH, TEST_PATCHED)

# For every byte in the file
for i in range(len(elfBytes)):
	# For every bit in the byte
	for j in range(8):
		# Modify byte
		elfByteArray[i] ^= 2 ** j
		elfBytes = bytes(elfByteArray)

		# Write to a file
		f = open(TEST_PATCHED, mode='wb')
		f.write(elfBytes)
		f.close()

		# Try to run new program
		try:
			proc = subprocess.run(TEST_PATCHED, stdout=subprocess.PIPE, timeout=1)
		except Exception as e:
			elfByteArray[i] ^= 2 ** j
			continue

		# If it looks ok write to a log file
		if proc.returncode == 0 and proc.stdout != b"Received message: (null)\n":
			m = hashlib.new('md5')
			m.update(elfBytes)
			md5 = m.digest().hex()

			logs.write("""====================
{} {}
{}
{}
""".format(i, j, md5, proc.stdout))
			logs.flush()

		elfByteArray[i] ^= 2 ** j

logs.close()
