#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <stdint.h>

char *substr(char *s, int i, int n) {
	char *sub = malloc(sizeof(char) * (n + 1));

	strncpy(sub, s + i, n);
	sub[n] = '\0';

	return sub;
}

int is_alpha_only(char *s) {
	for (int i = 0; s[i]; i++) {
		if (!islower(s[i])) {
			printf("%c\n", s[i]);
			return 1;
		}
	}

	return 0;
}

char *hash(char *input) {
	char *s = strdup(input);
	int input_len = strlen(s);

	for (int i = 0; i < input_len; i++) {
		for (int j = 0; j < input_len; j++) {
			if (j % 2 == 0) {
				if (i % 2 == 0) {
					s[j] += 0x1a;
				} else {
					s[j] -= 0x2a;
				}
			} else {
				if (i % 2 == 0) {
					s[j] -= 3;
				} else {
					s[j] -= 0x2a;
				}
			}

			if (s[j] == '\0') {
				s[j] = '*';
			}
		}

		for (int j = input_len - 1; j > 0; j--) {
			s[j] -= 2;

			if (s[j] == '\0') {
				s[j] = 0x2a;
			}
		}

		for (int j = 0; j < input_len; j++) {
			if (s[j] < 0) {
				s[j] = ~s[j] + 1;
			}

			if (s[j] <= 0x20) {
				s[j] += 0x2f;
			}
		}
	}

	return s;
}

const char *sum1 = "8531d8960e7f2447508d80e80d48fd96730cf89a9987268971d858fc49cba71a";
const int sum_size = 32;

int checksum(const char *sum, char *s, int i, int j) {
	int *sum_i = malloc(sizeof(uint8_t) * 32);

	for (int i = 0; i < 32; i++) {
		sscanf(sum + i * 2, "%2x", &sum_i[i]);
	}

	char *sub = substr(s, 12, 8);

	char *hash = sha256(sub, 8, 0);
}

int main(int argc, char **argv) {
	if (argc != 2) {
		puts("Args...");
		return 1;
	}

	char *input = strdup(argv[1]);
	int input_len = strlen(input);

	if (input_len != 20) {
		printf("No, %s is not correct.\n", input);
		return 1;
	}

	char *sub1 = substr(input, 9, 3);
	char *sub2 = substr(input, 0, 3);

	printf("%s\n", sub1);
	printf("%s\n", sub2);

	if (strcmp(sub1, sub2) != 0) {
		printf("No, %s is not correct.\n", input);
		return 1;
	}

	char *hashed = hash(input);

	if (is_alpha_only(input) != 0) {
		printf("No, %s is not correct.\n", input);
		return 1;
	}

	if (checksum(sum1, input, 12, 8) > 0) {

	}

	printf("%s\n", hashed);
}

// L0&0(0*0,0%0(0*0,0,0

