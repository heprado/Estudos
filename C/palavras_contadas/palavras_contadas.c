#include <stdio.h>

#define ESPACO 32
#define TAB 9
#define NOVA_LINHA 10
#define SEPARADOR_ENCONTRADO 0 
#define PALAVRA_ENCONTRADA 1

int main () {

    int caracter;

    int wc; 
    
    int palavra = SEPARADOR_ENCONTRADO;

    printf("Escreva palavras e aperte  \e[1;32mCTRL + C \e[0m para contarmos quantas palavras tem:\n");

    while ((caracter = getchar()) != EOF) {

        if (caracter == TAB ||
            caracter == ESPACO ||
            caracter == NOVA_LINHA) {
                palavra = SEPARADOR_ENCONTRADO;
            }
        
        else if ( palavra == SEPARADOR_ENCONTRADO) {
            palavra = PALAVRA_ENCONTRADA;
            wc++;
        }
    }
    
    printf("%d\n",wc);
    

    return 0;
}