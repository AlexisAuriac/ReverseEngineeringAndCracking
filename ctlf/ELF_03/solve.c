#include <stdlib.h>
#include <stdio.h>
#include <string.h>

char *get_mask() {
	char *mask = malloc(sizeof(char) * 16);
	memset(mask, 0, 16);

	long z = 0x0503020302;
	memcpy(mask, &z, 5);

	return mask;
}

int main (int argc, char **argv) {
	char *secret = "MasKerade133742A";
	char *mask = get_mask();

	for (int i = 0; i < 16; i++) {
		int c = mask[i];
		c += secret[i];

		printf("%c", c);
	}

	puts("");
}
