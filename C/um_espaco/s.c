#include <stdio.h>


#define BLANK 32
#define TAB 9



int main () {

    int c;
    
    int espaco_encontrado = 0;

    int tab_encontrado = 0;

    // Enquanto o character coletado do STDIN não for EOF(-1)
    while ((c = getchar()) != EOF ) {
        
        //ESPAÇO sabe aquele do seu teclado.
        if (c == BLANK) {

            //Aumenta o contador
            espaco_encontrado = espaco_encontrado + 1;

            //Se for maior que um significa que a pessoa escreveu dois espaços seguidos, a gente muda ele pra NULL.
            if (espaco_encontrado > 1) {
                
                c = NULL;
            }

        //Não encontramos nenhum espaço repetido então reiniciamos o contador.
        } else {

            espaco_encontrado = 0;
        }

        //TAB sabe aquele do seu teclado.

        if (c == TAB) {

            //Aumenta contador.
            tab_encontrado = tab_encontrado + 1;

            //Como TAB são três espaços, transformamos ele em UM UNICO ESPAÇO.
            c = BLANK;

            //Se for maior que um significa que a pessoa escreveu dois tabs seguidos, vamos mudar o proximo para NULL.
            if (tab_encontrado > 1) {
                
                c = NULL;

            }

        //Não encontramos nenhum TAB repetido então reiniciamos o contador.
        } else {

            tab_encontrado = 0;
        }
        
        //Coloca o caracter no STDOUT.
        putchar(c);

    }
    
    return 0;
    
}
