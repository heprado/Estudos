#include <stdio.h>


#define BLANK 32
#define TAB 9
#define BS 8


int main () {

    //Caracter sendo lido.
    int c;
    
    //Caracter encontrado a esquerda do caracter sendo lido.
    int left_c = 0 ;

    // Enquanto o character coletado do STDIN(ENTRADA PADRÃO) não for EOF(-1, FIM DO ARQUIVO)
    while ((c = getchar()) != EOF ) {
        

        //Se for um TAB já transformamos em BLANK porque só queremos um espaço entre as palavras.
        if (c == TAB) {
            c = BLANK;
        }


        //Se o caracter a esquerda e o caracter a direita ambos estiverem como BLANK, significa que recebemos diversos espaços e tabs seguidos.
        if (
            (
                left_c == BLANK
            ) 
                &&
            (
                c == BLANK
            )
            ) {
            
            //Não coloca caracteres no STDOUT(SAIDA PADRÃO) até encontrar algo que não seja BLANK
            continue;
            
        }
                
        //Coloca o caracter no STDOUT.
        putchar(c);

        //Salvando valor de c dentro da variavel left_c após termos colocado o caracter no STDOUT(SAIDA PADRÃO)
        left_c = c;

    }
    
    return 0;
    
}
