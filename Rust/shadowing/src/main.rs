
fn novo_escopo() {
    //Ponteiro imutável(Não pode ser outra string em tempo de execução)
    //para uma string.
    let nome:& str = "Henrique";

    //Imprimindo no STDIN o nome dentro da variável nome
    println!("Nome no escopo 0 {}", nome);

    //Criando um novo escopo
    {
        //Criando outro ponteiro imutável para uma string, utilizando o mesmo nome de variável.
        //Isso só é possivel pois estamos em outro escopo.
        let nome:& str = "Sombra Henrique";

        //Imprimindo no STDIN o novo nome dentro da variavel nome
        println!("Nome no escopo 1 {}", nome);
    }
}

fn if_novo_escopo() {

    //Ponteiro imutável(Não pode ser outra string em tempo de execução)
    //para uma string.
    let nome:&str = "Henrique";

    //Imprimindo no STDIN o nome dentro da variável nome
    println!("Nome no escopo {}", nome);

    //Criando um novo escopo caso o nome seja Henrique.
    if nome == "Henrique" {

        //Criando outro ponteiro imutável para uma string, utilizando o mesmo nome de variável.
        //Isso só é possivel pois estamos em outro escopo.
        //O interessante aqui é que estamos dentro de um escopo contextual,
        // podemos criar um escopo diferente para cada valor atribuido a variavel quando ela for inicializada.
        let nome:&str = "Sombra Henrique";
        println!("Nome dentro do escopo IF {}", nome);
    }

}

fn main() {

    println!("=========================================================");
    novo_escopo();
    println!("=========================================================");
    if_novo_escopo();
    println!("=========================================================");
}
