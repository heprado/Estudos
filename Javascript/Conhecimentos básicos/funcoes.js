//Contador(int32) de visitantes que a função incrementa
let count = 0

//Função padrão
//function nome(parametros = valor_padrao) {corpo da funcao}
function saudacao(nome = "Anônimo") {

    //Incrementando o contador cada vez que a função for chamada
    count += 1;

    //Retornando string com o nome do visitante e o ID(int32)
    return `Olá ${nome}, você é o visitante ${count}`
    
}

//Constantes com valores retornados pela função
const rafael = saudacao("Rafael");
const henrique = saudacao("Henrique");
const luiz = saudacao("Luiz");
const anonimo = saudacao();

//Valores de saida
console.log(rafael);
console.log(henrique);
console.log(luiz);
console.log(anonimo);


//Função Anônima
const soma = function (numero1 = 0 ,numero2 = 0) {

    return numero1 + numero2;

};

//Constante com o valor retornado pela função anônima
const minha_soma = soma(12,12);

console.log(minha_soma)

//Função flecha (arrow function)
//nome = (parametros = valor_padrao) => {corpo da funcao}
const raiz = (numero = 4) => {
    return numero ** 0.5;
};

//Constante com o valor retornado pela função flecha
const minha_raiz = raiz()

console.log(minha_raiz)

//Função flecha simplificada 
//nome = (parametros = valor_padrao) => retorno
const multiplicar = (numero1 = 0 , numero2 = 0) => numero1 * numero2;

//Constante com o valor retorna pela função flecha simplificada
const minha_multiplicacao = multiplicar(2,2)

console.log(minha_multiplicacao)






