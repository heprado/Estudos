//

// Inicializando as constantes perguntando varios valores para os usuarios;
// window é um objeto que podemos usar para interagir com o navegador,
// no caso estou utilizando o metodo prompt do objeto window e passando dois valores,
// o que ele deve perguntar e um valor para ficar dentro do campo onde a pessoa coloca a pergunta.

// Tambem estamos utilizando o metodo objeto Math do javascript, e chamando o metodo round que vai retornar o valor com 2 casas decimais
// e dentro estamos passando o objeto parseInt que retorna de uma string que no caso o windows.prompt que passou.




const nome = window.prompt("Digite seu nome","nome");
const ano = window.prompt("Digite seu ano de nascimento","ano de nascimento");
const peso = Math.round(parseInt(window.prompt("Digite seu peso","peso")));
const altura = Math.round(parseInt(window.prompt("Digite sua altura","altura")));
const hoje = new Date().getFullYear();

const idade = hoje - ano; // Calculado idade


const imc = peso / (altura**2); // Calculado IMC 

window.alert(`Seu imc é ${imc} e você tem ${idade} anos`); // mostrando resultado ao usuario utilizando o objeto window
