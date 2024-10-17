#include <stdio.h>

int main() {
    /*O caracter recebido é definido como int pois precisamos de espaço suficiente para guardarmos EOF, se colocassemos como char não teriamos espaço na variavel.*/
    int c;

    /*Recebe o caracter digitado e checa se ele é -1(EOF)*/
    /*EOF é uma costante da biblioteca stdio que define o fim de um arquivo que no nosso caso é quando pressionamos enter.*/

    while ((c = getchar()) != EOF) {
        /*Imprime os caracteres recebidos*/
        putchar(c);
    }

    return 0;
}
