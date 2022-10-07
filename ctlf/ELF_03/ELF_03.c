#include <stdlib.h>
#include <stdio.h>
#include <string.h>

int check_pw(char *arg1, char *arg2, char *arg3) {
	int i = 0;

	do {
		int c = arg3[i];
		c += arg2[i];

		if (arg1[i] != c) {
			return 0;
		if (arg2[i + 1] == '\0') {
		} else if (arg2[i + 1] == '\0') {
			break;
		}

		i++;
	} while (arg1[i] != '\0');

	return 1;
}

int main (int argc, char **argv) {
	if (argc != 2) {
		puts("You should give one argument.\n");
		return 1;
	}

	char *mask = malloc(sizeof(char) * 16);
	memset(mask, 0, 16);

	long z = 0x0503020302;
	memcpy(mask, &z, 5);

	int input_size = strlen(argv[1]);

	if (input_size == 16) {
		int res = check_pw(argv[1], "MasKerade133742A", mask);

		if (res) {
			printf("Yes, %s is correct!\n", argv[1]);
			return 0;
		}
	}

	printf("No, %s is not correct.\n", argv[1]);
	return 1;
}
