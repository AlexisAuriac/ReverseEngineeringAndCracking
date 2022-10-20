#include<stdlib.h>
#include<stdio.h>
#include<string.h>

char *custom_hash(char *input) {
	char *s = strdup(input);
	int len = strlen(s);

	for (int i = 0; i < len; i++) {
		if (i % 2 != 0) {
			s[i] += 42;
		} else {
			s[i] -= 84;
		}
	}

	for (int i = 0; i < len; i++) {
		if (s[i] == 0x7f) {
			s[i] = 'U';
		}

		if (s[i] <= 0x20) {
			s[i] += 0x54;
		}
	}

	return s;
}

const char *alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !\"#$%&'()*+'-./:;<=>?@[\\]^_`{|}~";
const char *hashed_pwd = "=sCc7_InB_9t5nB_D_C_D";
const int pwd_len = 21;

int main(int argc, char **argv) {
	char *pwd = malloc(sizeof(char) * (pwd_len + 1));

	memset(pwd, '0', pwd_len);
	pwd[pwd_len] = '\0';

	for (int i = 0; i < pwd_len; i++) {
		for (int j = 0; alpha[j]; j++) {
			pwd[i] = alpha[j];
			char *hashed = custom_hash(pwd);

			if (strncmp(hashed, hashed_pwd, i + 1) == 0) {
				break;
			}
		}
	}

	printf("%s\n", pwd);
}
