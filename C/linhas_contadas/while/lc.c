#include <stdio.h>

#define EOL 10

int main () {

    int c;
    double lc = 0;

    while( (c = getchar()) != EOF) {

        if (c == EOL)
            ++lc;
        
    }

    if (lc == 1) {
        printf("O texto possui %.f linha",lc);
    } else {
        printf("O texto possui %.f linhas",lc); 
    }

    

    return 0;
}