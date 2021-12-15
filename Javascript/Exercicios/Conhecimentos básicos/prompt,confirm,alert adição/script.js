//Script de adição simples

/*

prompt, confirm alert e close não metodos do objeto window do
javascript, mas não necessariamente precisamos especificar isso no
código (window.prompt(),window.confirm() etc.), podemos utilizar 
elas como funções comuns do Javascript como abaixo:

alert(`${a} + ${b} = ${a+b}`);



*/

const a =Number(prompt("Digite um número:"));

const b = Number(prompt("Digite outro número:"));

const confirma = confirm("Deseja fazer a adição?");

if (confirma){
    alert(`${a} + ${b} = ${a+b}`);

} else {
    close()
}


    




