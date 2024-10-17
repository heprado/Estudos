#include <stdio.h>

int main() {

    long nc;
 
    printf("Escreva um texto e aperte \e[1;32m CTRL + C \e[0m contarmos quantos caracteres ele possui.(Podendo ser nao visiveis) : \n");
    while((getchar()) != EOF) {
        ++nc;
    }
    printf("%.0f\n",nc);
    return 0;
}