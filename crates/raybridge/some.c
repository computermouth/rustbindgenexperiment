#include "some.h"

int add(int a, int b){ return a + b; }

teststruct teststruct_add(teststruct a, teststruct b) { return (teststruct){a.a + b.a}; }