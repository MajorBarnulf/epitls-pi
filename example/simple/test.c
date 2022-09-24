#include <stdio.h>
#include "../../clib/test.h"

void test_it_works() {
	assert_eq_int(2 + 2, 4);
}

void test_it_fails() {
	assert_eq_int(2 + 2, 5);
}
