//Criando um objeto e atribuindo a uma constante
//nome = {chave:valor,chave:valor,...}
const pessoa = {
    nome:"Henrique",
    idade:"26",
};

//ID para as pessoas
let contador_pessoa = 0

//Função anõnima para criar objetos de pessoas
const construtor_pessoa = function (nome_pessoa,idade_pessoa) {

    //Incrementando o ID por 1
    contador_pessoa += 1

    //Objeto que vamos retornar 
    return {
        nome:nome_pessoa,
        idade:idade_pessoa,
        id:contador_pessoa ,

        //Podemos definir uma função como valor desse objeto.
        falar () {

            //Para acessar os atributos do objeto devemos utilizar o this
            console.log(`Olá eu sou ${this.nome}, tenho ${this.idade} e sou o visitante nº ${this.id}`)

        }
    };
};


//Criando constantes e guardando os objetos de pessoas que a função nos retornou
const henrique = construtor_pessoa("Henrique",24)
const joao = construtor_pessoa("Joao",69)
const robo = construtor_pessoa("Robo",0)

//Chamando metodo do objeto
henrique.falar()
joao.falar()
robo.falar()