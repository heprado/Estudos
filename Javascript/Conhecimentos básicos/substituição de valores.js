/*
No exemplo abaixo temos 3 variaves A B C cada um contendo a string,
"a","b","c", no console.log colocamos elas em ordem log(a,b,c),
agora e se quisermos printar B C A?

Segue solução abaixo:


*/

let a = "a";
let b = "b";
let c = "c";

//Constante para guardar temporariamente o valor para salvarmos ele antes dele ser subscrito.
let saver;

//Salvando A na variavel temporaria
saver = a 
// a = "a" b = "b" c = "c" saver = "a"

//Colocando agora o valor de b dentro de a
a = b
// a = "b", b="b", c="c" , saver="a"

//Colocando agora o valor de c dentro de b
b = c
// a = "b", b="c", c="c", saver="a""


//Usando o "a" que salvamos dentro da constante para c
c = saver
// a = "b", b="c", c="a", saver="a""


console.log(a,b,c); // Queremos que printe B C A 

/*
Temos outra forma de fazer isso utilizando arrays.
*/
let um = 1;
let dois = 2;
let tres = 3;

//Nesse caso abaixo colocamos os 3 valores dentro de um array e dizemos que cada valor desse array é igual ao valores dos outros valores do outro array.
[um,dois,tres] = [tres, dois, um]



console.log(um,dois,tres)





