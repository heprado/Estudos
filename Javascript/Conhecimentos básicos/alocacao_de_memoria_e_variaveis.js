const array = [1,2]

const array_copy = array;

array_copy.push(3)


console.log(array,array_copy);
//Output : [ 1, 2, 3 ] [ 1, 2, 3 ]
//Quando você cria uma constante ou variavel no js
//e passa ela como parametro para a outra, o javascript não cria uma copia
//nova (as duas apontam para o mesmo endereço, na memoria)
//por isso quando atribuimos tentando enfiar um novo valor dentro
//do array os dois ficaram com o mesmo valor, para isso precisamos
//criar uma copia do array.



