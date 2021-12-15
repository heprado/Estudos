const nome = String(prompt("Digite seu nome completo:"));

let array_nome = nome.split(" ")



document.body.innerHTML += `Seu nome é <strong>${nome}</strong> </br>`;
document.body.innerHTML += `Seu nome tem <strong>${array_nome.join("").length} </strong>letras</br>`
document.body.innerHTML += `A segunda letra do seu nome é <strong>${nome[1]}</strong></br>`
document.body.innerHTML += `O indice da primeira letra R no seu nome é <strong>${nome.indexOf("r")}</strong></br>`
document.body.innerHTML += `O indice da ultima letra R no seu nome é <strong>${nome.lastIndexOf("r")}</strong></br>`
document.body.innerHTML += `As ultimas três letras do seu nome são <strong>${nome.slice(-3)}</strong></br>`
document.body.innerHTML += `As palavras do seu nome são <strong>${array_nome.join(",")}</strong></br>`
document.body.innerHTML += `Seu nome em maisculo é <strong>${nome.toUpperCase()}</strong></br>`
document.body.innerHTML += `Seu nome em minusculo é <strong>${nome.toLowerCase()}</strong></br>`