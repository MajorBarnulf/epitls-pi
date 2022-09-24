#include <stdio.h>
#include "../../test.h"

int main() {
	int a;
	printf("hello, world!! %d\n", a);
}

void test_it_works() {
	assert_eq_int(2 + 2, 4);
}

void test_it_fails() {
	assert_eq_int(2 + 2, 5);
}
