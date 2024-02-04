let nomeAluno; // é um objeto undefined
const sobreNomeAluno = null; // é um objeto undefined também

console.log(`Tipo:${typeof nomeAluno}, Valor: ${nomeAluno}`);
//Output:
//type:undefined : value:undefined

console.log(`Tipo:${typeof sobreNomeAluno}, Valor:${sobreNomeAluno}`);
//Output:
//object null
//O tipo dele ser como objeto é um bug do js pois na spec do js
//todos os valores que poderiam ser atribuidos a uma variavel
//tinham uma tag, cada uma com seu numero, e objeto tinha como
//numero 0 a tag, por conta do valor null no javascript 
//retornar 0x00 na maioria das plataformas x86 esse valor era
//convertido para 0 resultando 0, por isso null
//retorna object
//




