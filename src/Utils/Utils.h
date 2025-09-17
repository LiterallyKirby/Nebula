
#ifndef UTILS_H_
#define UTILS_H_

#include <cassert>
#include <cstdio>

#define myassert(condition)                                  \
    do {                                                     \
        if (!(condition)) {                                  \
            fprintf(stderr, "[ERROR] Assertion failed: %s\n", #condition); \
            assert(condition);                               \
        }                                                    \
    } while (0)

#endif
