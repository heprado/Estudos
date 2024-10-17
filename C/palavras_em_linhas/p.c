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
    printf("Escreva palavras e aperte ENTER, vamos mostrar cada palavra em uma linha:\n");
    printf("Aperte CTRL + C para sair.\n");

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