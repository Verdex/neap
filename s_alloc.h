
#ifndef __s_alloc_h__
#define __s_alloc_h__

#include<stdlib.h>

enum AllocType { AT_Standard };

struct Alloc {
    enum AllocType type;
    void* data;
    void* (*alloc)(struct Alloc*, size_t);
    void (*free)(struct Alloc*, void*); 
};

#define s_alloc(a, size) (a)->alloc((a), (size))
#define s_free(a, ptr) (a)->free((a), (ptr))

struct Alloc* standard_allocator();
void consume_standard_allocator(struct Alloc*); 

#endif
