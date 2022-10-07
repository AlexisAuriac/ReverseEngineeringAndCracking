#include <stdlib.h>
#include <stdio.h>
#include <string.h>

int main (int argc, char **argv) {
	if (argc != 2) {
		puts("You should give one argument.");
		return 1;
	}

	char *input = argv[1];
	char c = input[0];
	int i = 1;
	int j = 0;
	int char_sum = 0;

	if (c != '\0') {
		do {
			char_sum += c;
			j = i;
			i++;
			c = input[i - 1];
		} while (c != '\0');
	}

	if (j != 16 || char_sum != 1664) {
		printf("No, %s is not correct.\n", argv[1]);
		return 1;
	}

	printf("Yes, %s is correct!\n", argv[1]);
	return 0;
}
