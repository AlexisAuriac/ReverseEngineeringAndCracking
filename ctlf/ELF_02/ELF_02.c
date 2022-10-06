#include <stdio.h>
#include <string.h>

int main (int argc, char **argv) {
    if (argc != 2) {
    	puts("You should give one argument.");
    	return 1;
    }

    char *input = argv[1];
    int input_size = strlen(input);
    int i = 0;

    char c = 'p';
    char *pass = "password424242";

    if (input_size != 14) {
		printf("No, %s is not correct.\n", input);
        return 1;
        // 0xffffffff
    }

    do {
    	char c2 = input[i];

        if (c2 == '\0') {
        	break;
        }

        c--;
        if (c != c2) {
			printf("No, %s is not correct.\n", input);
		    return 1;
		    // 0xffffffff
        }

        i++;
        c = pass[i];
    } while (c != 0);

    printf ("Yes, %s is correct!\n", input);
    return 1;
}
