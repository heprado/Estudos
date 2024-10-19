#include <stdio.h>

#define BLANK 32
#define TAB 9
#define LF 10
 

int main() {

    int c;
    long blanks = 0 ;
    long tabs = 0;
    long lfs = 0;
 
    
    for (c=0;(c = getchar()) != EOF;) {

        switch (c) {
            case BLANK:
                ++blanks;
                break;
            case TAB:
                ++tabs;
                break;
            case LF:
                ++lfs;
                break;
        }

    }
    
    
    printf("O texto possui %ld linhas\n",lfs);
    printf("O texto possui %ld espacos\n",blanks);
    printf("O texto possui %ld tabs\n",tabs);

    return 0;
}