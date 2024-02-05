//Valores Primitivos

//Criando uma variavel com uma string
let primitivo = "Oie"

//Tentando alterar o indice 0 da string para a letra o miniscula
primitivo[0] = "o"

//A saida será Oie pois strings são imutaveis no Javascript.
console.log(primitivo)

//Outros exemplos de valores imutaveis são int,boolean,undefined e null



//Valores por Referencia

//Criando uma variavel contendo um array
let minha_lista = [0,1,2,3,4]

//Copiando a variavel minha_lista para variavel sua_lista
let sua_lista = minha_lista

//Adicionando o numero inteiro 5 ao final do array
sua_lista.push(5)

//A saida sera [0,1,2,3,5] [0,1,2,4,5] , isso acontece pois variaveis contendo arrays quando atribuidas a outras variaveis são referenciadas e não copiadas, as duas variaveis apontam para o mesmo array
console.log(minha_lista,sua_lista)

//Para fazer uma copia diferente da variavel devemos utilizar a seguinte sintaxe:

let nossa_lista = [...sua_lista]

//Adicionando o numero inteiro 6 ao final do array
nossa_lista.push(6)

//A saida sera [0,1,2,4,5] [0,1,2,4,5] [0,1,2,4,5,6]
//Podemos checar que somente a variavel nossa_lista possui o numero inteiro 6
console.log(minha_lista,sua_lista,nossa_lista)