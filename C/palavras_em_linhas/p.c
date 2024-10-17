#include <stdio.h>

#define ESPACO 32
#define TAB 9
#define NOVA_LINHA 10
#define SEPARADOR_ENCONTRADO 0
#define PALAVRA_ENCONTRADA 1

int main () {

    int caracter;
    
    int palavra = SEPARADOR_ENCONTRADO;
    
    // TODO
    printf("Escreva palavras e aperte \e[1;32m ENTER \e[0m, vamos mostrar cada palavra em uma linha:\n");
    printf("Aperte \e[1;32m CTRL + C \e[0m para sair.\n");
    printf("Não tá pronto...\n");

    while ((caracter = getchar()) != EOF) {
        
        
        if (caracter == TAB ||
            caracter == ESPACO ||
            caracter == NOVA_LINHA) {
             palavra = SEPARADOR_ENCONTRADO;
            }
        else if (palavra == SEPARADOR_ENCONTRADO) {
         palavra = PALAVRA_ENCONTRADA;
            
        };
        
        if (palavra == PALAVRA_ENCONTRADA) {
            putchar(caracter);
            putchar(NOVA_LINHA);
        };

        
    }
    
    

    return 0;
}