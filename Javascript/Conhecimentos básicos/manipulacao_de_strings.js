
//Sempre que você quiser escapar um caracter especial numa string é só colocar uma "\" antes do caracter

let texto = "Eu queria muito escrever um \n sem pular a linha mas não sabia que precisava por um caracter de escape \\n"

console.log(texto)
//Output: Eu queria muito escrever um 
// sem pular a linha mas não sabia que precisava por um caracter de escape \n

/*
Raw strings.
=======================================================================================================================
*/



//Podemos passar para a tag function( ou method) raw do objeto String uma template string com 
//caracteres especias que o javascript não vai interpretar elas por se tratar de uma raw string.

let raw_string = String.raw`Aqui eu posso escrever \n de boa que ele não vai pular a linha`

console.log(raw_string)
//Output: Aqui eu posso escrever \n de boa que ele não vai pular a linha


/*
String Indexes
=======================================================================================================================
*/




//Toda string no javascript é um array de chars, então podemos referenciar qual valor queremos da string através de indices
//isso é legal de se usar quando estamos usando algum tipo de loop como for e while.

let letras = "abcdefg"

console.log(letras[2])



/*
Descobrindo Index
=======================================================================================================================
*/

//Caso queiramos saber em qual index começa uma certa parte da string podemos utilizar o indexOf e o lastIndexOf.



let umTexto = "Aqui temos um texto. Aqui temos a continuação do texto. "


//Começa procurando a string "Aqui" a partir do COMEÇO da string até o fim, o primeiro que ele encontrar vai ser o resultado
console.log(umTexto.indexOf("Aqui"))
//Output: 0

//Mas e se quisermos não o primeiro "Aqui" e sim o segundo, podemos passar como parametro da função a partir de qual index ele deve procurar.
console.log(umTexto.indexOf("Aqui",19))
//Output: 21

//Começa procurando a string "Aqui" a partir do FIM da string, o primeiro que ele encontra vai pra match
console.log(umTexto.lastIndexOf("Aqui"))
//Output: 21

//Se passarmos um index para o lastIndexOf ele funciona como o indexOf.
console.log(umTexto.lastIndexOf("Aqui",3))
//Output:0


/*
Usando Regexp
=======================================================================================================================
*/

/*
    No javascript o objeto string já temos os metodos para
    regex, diferente da maioria das linguagens em que precisamos
    importar um modulo externo para utilizarmos regex.

*/

let regextext = "Eu sou uma string, vou ser usada no regexp"

//Com o match podemos procurar por um valor dentro da string,
//no caso abaixo estamos procurando todas as letras minusculas
//e maiusculas, mas o match sempre para na primeiro match que ele
//der
console.log(regextext.match(/[a-zA-Z]/))
/*
Output:
[
  'E',
  index: 0,
  input: 'Eu sou uma string, vou ser usada no regexp',
  groups: undefined
]
*/

//Essa é uma consulta global de regex pois é 
//isso que o G após o regex indica , ela vai dar match em todos
//os valores que forem encontrados até o final da string, ela não
//para na primeira
console.log(regextext.match(/[a-zA-Z]/g))
/*
Output:
[
  'E', 'u', 's', 'o', 'u', 'u',
  'm', 'a', 's', 't', 'r', 'i',
  'n', 'g', 'v', 'o', 'u', 's',
  'e', 'r', 'u', 's', 'a', 'd',
  'a', 'n', 'o', 'r', 'e', 'g',
  'e', 'x', 'p'
]
*/

//Caso queiramos achar o index que algum valor dentro da string,
//podemos utilizar o search do regex passando o regex, no caso abaixo
//estamos procurando todas as letras minusculas e maisculas,
//mas o search sempre vai parar no primeiro valor que ele encontrar
//que bate com o regex.
console.log(regextext.search(/[a-zA-Z]/))
//Output : 0
//O output foi zero pois a primeira letra que ele encontrou estava
//logo no inicio da string.


/*
Usando Replace
=======================================================================================================================
*/

//Caso queiramos mudar o valor de algo escrito dentro de uma string para outro, podemos utilizar o replace.

let textoreplace = "Henrique programava em python, mas recentemente Henrique começou a aprender Javascript";

console.log(textoreplace.replace("Henrique","Um cara"));
//Output: Um cara programava em python, mas recentemente Henrique começou a aprender Javascript
//Conseguimos ver que Henrique só foi substituido uma vez, pois o replace substitui somente a primeira string que ele encontrar



console.log(textoreplace.replaceAll("Henrique","Um cara"));
//Output:Um cara programava em python, mas recentemente Um cara começou a aprender Javascript
//Nesse caso o replace procura a string inteira por matches e substitui.


//Também podemos utilizar regex globais para fazermos uma substituição em todos os valores.
let replaceregex = "aaabbbcccddd"

console.log(replaceregex.replace(/a/g,"1"))
//Output:111bbbcccddd
//Todas as letras a foram trocadas por 1 pois o regex é global, fazendo assim replace ser global tbm.




/*
Cortando uma string.
=======================================================================================================================
*/

//Caso queiramos pegar uma parte de uma string e remover todo o restante podemos utilizar o slice.


let slicestring = "Eu quero tirar tudo isso aqui. Eu quero tirar essa parte também."


//Estamos passando como parametro de qual index até qual index ele vai cortar.
//No caso abaixo de 0 até 30
console.log(slicestring.slice(0,30))
//Output : Eu quero tirar tudo isso aqui.

//No caso abaixo estamos fazendo de trás pra frente, pois estamos utilizando um index negativo, então
//os valores dos index começam contando do fim da string pra frente.
console.log(slicestring.slice(-33))
//Output:Eu quero tirar essa parte também.

/*
Desmembrando uma string
=======================================================================================================================
*/

let splitstring = "Olha eu sou uma string enorme , vou ser transformara em um array , cada palavra em um valor ."

//Utilizando o metodo split do objeto string, passando como parametro qual é o valor que ele deve utilizar como separador, no caso é um espaço " "

console.log(splitstring.split(" "))
/*
Output:
[
  'Olha',   'eu',
  'sou',    'uma',
  'string', 'enorme',
  ',',      'vou',
  'ser',    'transformara',
  'em',     'um',
  'array',  ',',
  'cada',   'palavra',
  'em',     'um',
  'valor',  '.'
]
*/


/*
Deixando tudo em maisculo ou tudo em minusculo
=======================================================================================================================
*/

let lowerupper = "OLA EU TO EM MAISCULO,mas agora eu to em minusculo."

//Printa tudo em minusculo
console.log(lowerupper.toLowerCase());
//Output:ola eu to em maisculo,mas agora eu to em minusculo.

console.log(lowerupper.toUpperCase());