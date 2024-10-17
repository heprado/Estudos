#include <stdio.h>


#define ESPACO 32
#define TAB 9
#define BS 8


int main () {

    //Caracter sendo lido.
    int caracter;
    
    //Caracter encontrado a esquerda do caracter sendo lido.
    int caracter_esquerda = 0 ;

    // Enquanto o character coletado do STDIN(ENTRADA PADRÃO) não for EOF(-1, FIM DO ARQUIVO)
    while ((caracter = getchar()) != EOF ) {
        

        //Se for um TAB já transformamos em ESPACO porque só queremos um espaço entre as palavras.
        if (caracter == TAB) {
            caracter = ESPACO;
        }


        //Se o caracter a esquerda e o caracter a direita ambos estiverem como ESPACO, significa que recebemos diversos espaços e tabs seguidos.
        if (
            (
                caracter_esquerda == ESPACO
            ) 
                &&
            (
                caracter == ESPACO
            )
            ) {
            
            //Não coloca caracteres no STDOUT(SAIDA PADRÃO) até encontrar algo que não seja ESPACO
            continue;
            
        }
                
        //Coloca o caracter no STDOUT.
        putchar(caracter);

        //Salvando valor de caracter dentro da variavel caracter_esquerda após termos colocado o caracter no STDOUT(SAIDA PADRÃO)
        caracter_esquerda = caracter;

    }
    
    return 0;
    
}
