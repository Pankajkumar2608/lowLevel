#include <stdio.h>

int main() {
    // struct - use to create a custom data type that can be used to store multiple values
    struct hello
    {
        int id;
        char name;
    };
    
    struct hello h;
    h.id = 1;
    h.name = 'a';
    printf("%d %c\n", h.id, h.name);
    
    printf("Hello world\n");
}