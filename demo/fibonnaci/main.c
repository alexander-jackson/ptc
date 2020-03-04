#include <stdio.h>

#include "fibonacci.h"

int main(int argc, char *argv[]) {
	int cases[3] = { 3, 7, 13 };

	for (int i = 0; i < 3; ++i) {
		printf("fib(%d) = %d\n", cases[i], fibonacci(cases[i]));
	}

	return 0;
}
