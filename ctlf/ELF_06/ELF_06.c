#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

const char *alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !\"#$%&'()*+'-./:;<=>?@[\\]^_`{|}~";
const char *hashed_pwd = "K+N.R.X+R1U-M,X%Q+U";
const int pwd_len = 19;

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

int main(int argc, char **argv) {
	char *pwd = malloc(sizeof(char) * (pwd_len + 1));

	memset(pwd, '0', pwd_len);
	pwd[pwd_len] = '\0';

	for (int i = 0; i < pwd_len; i++) {
		for (int j = 0; alpha[j]; j++) {
			pwd[i] = alpha[j];
			char *hashed = hash(pwd);

			if (strncmp(hashed, hashed_pwd, i + 1) == 0) {
				break;
			}
		}
	}

	printf("%s\n", pwd);
}
