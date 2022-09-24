#include <stdlib.h>
#include <stdio.h>

void assert_eq_int(int left, int right) {
	if (left != right) {
		printf("assertion failed\n'%i' != '%i'\n", left, right);
		exit(1);
	}
}