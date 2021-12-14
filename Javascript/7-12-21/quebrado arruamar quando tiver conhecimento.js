const readline = require("readline");

const r1 = readline.createInterface({
    input:process.stdin,
    output:process.stdout
});

r1.question("Qual seu nome?", function(name) {
    r1.question("Qual seu peso?", function(peso) {
        r1.question("Qual sua altura?", function(altura) {

            let imc = peso / (altura**2);
            let nivel_de_magreza;
            

            if (imc <= 0 || imc >= 18.5){
                console.log("Desculpa mas esse valor não é aceito.")
            }
            else if (imc <= 18.5) {
                let nivel_de_magreza = "MAGREZA"
            }
            else if (imc >= 18.5  || imc <= 24.9) {
                let nivel_de_magreza = "NORMAL"
            }
            else if (imc >= 25.0 || imc <= 29.9){
                let nivel_de_magreza = "SOBREPESO"
            }
            else if (imc >= 30.0 || imc <= 39.9){
                let nivel_de_magreza = "OBESIDADE"
            }
            else {
                let nivel_de_magreza = "OBESIDADE GRAVE"
            }
            
            console.log(`${name} seu IMC é 
                        ${imc} e seu nivel de magreza é 
                        ${nivel_de_magreza}`)
        r1.close();
        });
    });
});

