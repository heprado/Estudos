//Constantes são valores que vamos usar no nosso código,
//mas diferente de variaveis são valores que não podem mudar.


const numero; // Não vai funcionar pois constantes precisam ser
              // inicializadas.

const numero = 5; // Vai atribuir o valor int 5 dentro da constante.


numero = 4; // Não vai funcionar pois não podemos atribuir valores 
            //a constantes

console.log(typeof numero) ; // Vai printar o tipo do valor
                            // no nosso caso é um number(int).
                            // number cabe até 2^52 - 1 valores dentro
                            // se você precisar de um numero maior
                            // usa BigInt

let bigint = BigInt("NUMERO MUITO GRANDE");


