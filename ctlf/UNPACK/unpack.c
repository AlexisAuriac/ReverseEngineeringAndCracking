int main(int argc, char **argv) {
	if (argc == -1) {

	} else if (argc != 2) {
		char *prog = argv[0];

		if (prog == NULL) {
			prog = "unpack";
		}

		printf("USAGE: %s key\n", prog);
		return 1;
	} else {
		main(-1, ?);
	}
}
