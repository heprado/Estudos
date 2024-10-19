#include <stdio.h>


#include <locale.h>


#define ESPACO 32
#define TAB 9
#define NOVA_LINHA 10
#define SEPARADOR_ENCONTRADO 0
#define LETRA_ENCONTRADA 1

int main () {

    setlocale(LC_ALL,"");
    
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
            palavra = LETRA_ENCONTRADA;   
        };
        
        if (palavra == LETRA_ENCONTRADA) {
            
            putchar(caracter);
        } else if (palavra == SEPARADOR_ENCONTRADO)  {
            putchar(NOVA_LINHA);
        }
            
    

    }
    
 

    return 0;
}