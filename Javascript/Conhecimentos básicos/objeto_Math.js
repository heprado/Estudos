/*

O objeto Math possui diversos metodos que podem nos ajudar em questões
matematicas, abaixo segue alguns exemplos:

*/

/*
Arredondando em Javascript
=======================================================================================================================
*/


let numeroround = 9.5

//Usando o round tudo depende do numero, se ele for 9.4 o round arredonda pra baixo.
//se ele for 9.5 ele arredonda para cima.
let round = Math.round(numeroround)

console.log(round)
//Output:10

//Aqui estamos arredondando para cima
let roundup = Math.ceil(numeroround)


console.log(roundup)
//Output: 10


//Aqui estamos arredondando para baixo
let rounddown = Math.floor(numeroround)

console.log(rounddown)
//Output: 9 

/*
Descobrindo o maior e o menor valor entre varios numeros
=======================================================================================================================
*/


console.log(Math.max(1,2,3,8,6,4345,345,123,124,78,8673,45,123,-1))
//Output:8673


console.log(Math.min(1,2,3,8,6,4345,345,123,124,78,8673,45,123,-1))
//Output:-1

/*
Gerando numeros aleatorios
=======================================================================================================================
*/

//O metodo random gera um numero aleatorio entre 0 e 1

console.log(Math.random())
//Output: qualquer numero entre 0 e 1.
//Exemplo: 0.4809729062778716

const numeroAleatorio = Math.random()

//Se quisermos gerar um numero entre 0 e 10 por exemplo,
//podemos multiplicar o resultado por 11

console.log(numeroAleatorio,numeroAleatorio * 11)
//Output: qualquer numero entre 0 e 10
//Exemplo: 0.8106624984615991 8.91728748307759




