

/*

Diferente da maioria das linguaguens de programação, no javascript não temos
int,float,double,long,short etc, todos esses tipos de numeros são resumidos dentro 
do objeto Number.

Todos os Numbers do javascript são guardados como double floats de 64bits.

*/



/*
Casting de Numeros para Strings.
=======================================================================================================================
*/


let num1 = 2.5 //Podemos guardar numeros fracionados
let num2 = 1000  //Tambem podemos guardar numeros inteiros



//O objeto Number no javascript possui o metodo toString, mas podemos também chamar o objeto String() para transformar em String.
//A unica diferença entre os dois é que o toString retorna um TypeError caso o valor seja null ou undefined, já o String() não retorna nada
console.log(typeof num2.toString(), typeof String(num1))
//Output: string string.


/*
Numeros quebrados em javascript
=======================================================================================================================
*/

//Quando usamos numeros fracionados em javascript ele tenta o maximo possivel de manter a precisão do resultado utilizando os padrões
//do IEEE 754-2008


let numeroquebrado1 = 0.7

let numeroquebrado2 = 0.1

let contaquebrada = numeroquebrado1 + numeroquebrado2

console.log(contaquebrada);
//Output: 0.7999999999999999

//Para arrumarmos as casas decimais só utilizarmos o toFixed novamente.

console.log(contaquebrada.toFixed(1))
//Output: 0.7



/*
Transformando numero em binario ou hexadecimal
=======================================================================================================================
*/

//Podemos passar o radix dentro do toString para definirmos qual numero ele vai usar como base para converter os valores.

let binhexnumber = 192

console.log(binhexnumber.toString(2))
//Output: 11000000

console.log(binhexnumber.toString(16))
//Output: c0

/*
Removendo numeros após a virgula.
=======================================================================================================================
*/

//Podemos usar o toFixed(), passando dentro do metodo como parametro quantos numeros queremos da casa decimal.

let bignumber = 10.3939393930293029

console.log(bignumber.toFixed(2))
//Output: 10.39

console.log(bignumber.toFixed(4))
//Output: 10.3939

//PS: Utilize o toFixed só quando for precisar mostrar esse valor, enquanto estiver fazendo contas deixe como está para
//termos precisão maiores nos resultados.


/*
Checando se um numero é inteiro
=======================================================================================================================
*/

//Utilizamos o metodo isInteger do objeto Number para checar se um numero é inteiro ou não
//se for retorna true se não false

let isintnumber = 10

console.log(Number.isInteger(isintnumber))
//Output: true


/*
Checando se o resultado de uma conta resulta em um numero.
=======================================================================================================================
*/

let truenumber = 10

let falsenumber = "oi eu não sou um numero"

let contaNaN = truenumber * falsenumber

console.log(Number.isNaN(contaNaN))
//Output: true




