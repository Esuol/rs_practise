#include "stdio.h"

// #include "add.h"

// 我们可以直接通过 extern 定义外部函数。
extern unsigned add(unsigned a, unsigned b);

int main()
{
    unsigned result = add(1, 2);
    printf("%d", result);
    return 0;
}