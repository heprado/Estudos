function calcular_imc() {


    
    //Criando objeto com a pessoa que vamos calcular o IMC 
    let pessoa = {
        nome : document.querySelector("#nome").value, //Pegando valores preenchidos no formulario via id do input
        sobrenome : document.querySelector("#sobrenome").value,
        peso :document.querySelector("#peso").value,
        altura : document.querySelector("#altura").value
    }

    let imc = pessoa.peso/(pessoa.altura**2) //Calculo do IMC

    let imc_media;
    
    //Calculo da media do IMC
    if (imc < 18.5) {
        imc_media = "Abaixo do peso normal.";
    }    
    else if (imc > 18.5 && imc < 25) {
        imc_media = "Peso normal"
    }
    else if (imc > 25 && imc < 30) {
        imc_media = "Sobrepeso"
    }
    else {
        imc_media = "Obesidade"
    };

    //Variavel com string contendo o resultado do IMC
    let resultado = `Olá ${pessoa.nome} ${pessoa.sobrenome}, baseado na sua altura de ${pessoa.altura} e peso de ${pessoa.peso} seu IMC é ${imc} e na media é considerado ${imc_media}`

    let form = document.querySelector("form")

    //Checando se o resultado não existe na pagina e criando ele.
    if (!document.querySelector("#resultado_imc")) {

        let imc_element = document.createElement("p")

        imc_element.id = "resultado_imc"
        
        imc_element.innerHTML = resultado;

        form.appendChild(imc_element)

    }

    else {
        let imc_element = document.querySelector("#resultado_imc").innerHTML = resultado //Substituindo o resultado exista.
    }  
}