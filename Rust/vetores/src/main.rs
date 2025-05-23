fn main() {

    //Vetores só podem possuir items de um tipo e não tem tamanho definido(diferente do array).
    //Precisamos tipar o Vec<> caso inicializarmos ele vazio.
    let mut vec1:Vec<u32> = Vec::new();


    //Caso inicializarmos o Vetor com valores o compilador consegue inferir o tipo.
    //Abaixo seguem dois exemplos de criar e já inicializar o vetor.
    let mut vec2 = vec![1,2,3];
    vec2 = Vec::from([1,2,3]);

    vec1.push(1);

    vec1.push(2);

    //Pegamos uma refêrencia do primeiro item do vetor.
    let mut item = &vec1[0];
    //O problema disso é que se acessarmos um indice que não existe o programa vai dar panic.
    //item = &vec1[5];

    println!("{:?}", item);

    //Para evitarmos acessar um indice que não existe podemos usar o metodo get do vetor.
    //Ele retorna uma Option, então podemos ver se é Some() ou None
    let item2 = vec1.get(10);

    if let Some(value) = item2 {
        println!("{:?}", value);
    }
    else {
        println!("Não tinha nenhum valor no indice.");
    }

}
