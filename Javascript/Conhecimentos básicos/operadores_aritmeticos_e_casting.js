
/*

OPERAÇÕES ARITMETICAS


Tudo que for executado dentro do parenteses vai ser executado primeiro,
após isso leva-se em consideração a precedencia caso elas sejam de mesmo
valor são executadas da direita pra esquerda, caso a precedencia seja diferente
as maiores vão ser executadas primeiro

expression = (2 * 4 / 2) ** (3 + 1 - 2)

resultado 16

Explicação: primeiro de tudo executamos tudo dentro do primeiro,
pararenteses então (2 * 4 / 2), primeiro vemos a precedencia dos
dois operadores aritmeticos, no caso * é 2 e / é 2 também,
então fazemos as operações da direita pra esquerda, 2 * 4 = 8,
8 / 2 = 4, após isso vamos exponenciar o valor 4 pelo valor do
paretenses (3 + 1 - 2), seguindo a mesma logica + precedencia 3 e
- precedencia 3 também , 3 + 1 = 4 , 4 - 2 = 2,
finalmente temos os dois valores de ambos os paresentes e podemos 
exponenciar 4 ** 4 = 16




+ (adição) (prededencia 3)
- (subtração) (prededencia 3)
* (multiplicação) (prededencia 2)
/ (divisão) (prededencia 2)
% (módulo) (prededencia 2)
** (exponenciação) (prededencia 1)
() (eleva a precedencia) 


*/


//CONTADORES


let a = 1

console.log(a++)
/*
Output: 1 

Isso acontece pois quando colocamos o ++ depois do valor,
esse valor só vai ser atribuido depois de tudo acontecer no código
o console.log()

*/

console.log(++a)
//Output: 3
//Aqui o valor agora é 3 pois o valor vai ser primeiro adicionado a variavel A,
// e depois o resto do código vai continuar.


console.log(a+=a);
//Output: 6

//Aqui o valor agora é 6 pois somamos A(3) com A(3) e printamos.


//Util de utilizar
for (let i = 0; i < a; i++) {

    a-=2

    console.log(a)

}




//CASTING


//Number
let numero = 5

//Casting de string pra numero.
let numero2 = Number('5')

//Casting de numero pra string.
let numeroparastring = String(1234)


//Calculando Raiz Quadrada

let raiz = 144 ** 0.5

console.log(raiz)

//Dividindo por Zero

//Na maioria das linguaguens de programação, dividir por 0 te retorna
//uma exceção no código como ZeroDivision etc, já no javascript é diferente:


let zerodivision = 100 / 0

console.log(zerodivision)
//Output: Infinity

