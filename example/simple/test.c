#include <assert.h>
#include <stdio.h>

void test_it_works() {
	assert(2 + 2 == 4);
}

void test_it_fails() {
	assert(2 + 2 == 5);
}
