#include<stdio.h>
#include <stdint.h>

typedef struct {
    char* n;
    char* e;
    char* d;
} Key;

Key* generate_key();
char* encode(char* m, char* e, char* n);
char* decode(char* c, char* d, char* n);


int main()
{
    Key* key = generate_key();
    char* message = "1e228dabc1231234142131252131234213123421321565612123213125421321321abcde";
    printf("Pointer is: %p", message);
    printf("Pointers are: %p,\n %p,\n %p \n", key->n, key->e, key->d);
    char* cipher = encode(message, key->e, key->n);
    printf("N is %s", key->n);
    printf("Cipher is %s", cipher);
    printf("\n The result is [%s]\n", decode(cipher, key->d, key->n));
}