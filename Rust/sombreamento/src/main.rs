

fn main() {

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
