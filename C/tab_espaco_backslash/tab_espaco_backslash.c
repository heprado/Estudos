#include <stdio.h>

#define SPACE 32
#define TAB 9
#define BACKSPACE 8
#define BACKSLASH 92

//Não deu certo com backspace como pede no livro, eu precisaria ou da curses(linux) ou da conio(windows) para executar.
int main () {

    int c;

    while ((c = getchar()) != EOF) {

        switch (c) {
            case SPACE:
                putchar(BACKSLASH);
                putchar('s');
                continue;
            case TAB:
                putchar(BACKSLASH);
                putchar('t');
                continue;
            case BACKSLASH:
                putchar(BACKSLASH);
                putchar(BACKSLASH);
                continue;
        }
        putchar(c);
    }
    return 0;
}