#include <stdio.h>
extern int bar(int a, int b);

void foo() {
	printf("%d", bar(1,2));
}
