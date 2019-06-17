
#include<stdlib.h>
#include<assert.h>
#include "s_alloc.h"

static void standard_free(struct Alloc* allocator, void* ptr) {
    assert( allocator->type == AT_Standard );

    free(ptr);
}

static void* standard_alloc(struct Alloc* allocator, size_t size) {
    assert( allocator->type == AT_Standard );

    return malloc(size);
}

struct Alloc* standard_allocator() {
    struct Alloc* ret = malloc(sizeof(struct Alloc));
    ret->type = AT_Standard;
    ret->data = NULL;
    ret->free = standard_free;
    ret->alloc = standard_alloc;
    return ret;
}

void consume_standard_allocator(struct Alloc* allocator) {
    assert( allocator->type == AT_Standard );
     
    free(allocator);
}
