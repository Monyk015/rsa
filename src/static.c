#include<stdio.h>
#include <stdint.h>

typedef struct {
    char* n;
    char* e;
    char* d;
} Key;

Key generate_key();
char* encode(char* m, char* e, char* n);
char* decode(char* c, char* d, char* n);


int main()
{
    Key key = generate_key();
    char* message = "1e228dabc";
    char* cipher = encode(message, key.e, key.n);
    printf("\n The result is [%s]\n", decode(cipher, key.d, key.n));
}