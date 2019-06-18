
#include<stdint.h>
#include<stdio.h>
#include "s_alloc.h"

int main() {
    struct Alloc* a = standard_allocator();

    int8_t* arr = s_alloc(a, sizeof(int8_t) * 10);
    
    for(int i = 0; i < 10; i++) {
        arr[i] = i;
    }

    for(int i = 0; i < 10; i++) {
        printf("%d\n", arr[i]);
    }

    s_free(a, arr);

    consume_standard_allocator(a);
	
	return 0;
}
