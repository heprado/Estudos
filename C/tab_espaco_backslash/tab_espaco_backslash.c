#include <stdio.h>

#define ESPACO 32
#define TAB 9
#define VOLTAR 8
#define BARRA_ESQUERDA 92

//Não deu certo com VOLTAR como pede no livro, eu precisaria ou da curses(linux) ou da conio(windows) para executar ai eu só decidi não fazer mesmo de desgosto, mas depois eu vou tentar fazer uma versão que funciona pros dois usando uns ifndef sei la uns negocio assim eu preciso entender ainda...

int main () {

    int c;

    while ((c = getchar()) != EOF) {

        switch (c) {
            case ESPACO:
                putchar(BARRA_ESQUERDA);
                putchar('s');
                continue;
            case TAB:
                putchar(BARRA_ESQUERDA);
                putchar('t');
                continue;
            case BARRA_ESQUERDA:
                putchar(BARRA_ESQUERDA);
                putchar(BARRA_ESQUERDA);
                continue;
        }
        putchar(c);
    }
    return 0;
}