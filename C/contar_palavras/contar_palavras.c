#include <stdio.h>

#define SPACE 32
#define TAB 9
#define NL 10
#define OUTSIDE_WORD 0 
#define INSIDE_WORD 1

int main () {

    int c;
    int wc; 
    
    int word = OUTSIDE_WORD;

    printf("Escreva palavras e aperte  \e[1;32mCTRL + C \e[0m para contarmos quantas palavras tem:\n");

    while ((c = getchar()) != EOF) {

        if (c == TAB ||
            c == SPACE ||
            c == NL) {
                word = OUTSIDE_WORD;
            }
        
        else if ( word == OUTSIDE_WORD) {
            word = INSIDE_WORD;
            wc++;
        }
    }
    
    printf("%d\n",wc);
    

    return 0;
}