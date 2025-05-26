fn main() {

    vetores();

    vec_multi_valorado();

}



fn vetores() {
    //Vetores só podem possuir items de um tipo e não tem tamanho definido(diferente do array).
    //Precisamos tipar o Vec<> caso inicializarmos ele vazio.
    let mut vec1:Vec<u32> = Vec::new();


    //Caso inicializarmos o Vetor com valores o compilador consegue inferir o tipo.
    //Abaixo seguem dois exemplos de criar e já inicializar o vetor.
    vec1 = vec![1,2,3];
    vec1 = Vec::from([1,2,3]);

    vec1.push(1);

    vec1.push(2);

    //Pegamos uma refêrencia do primeiro item do vetor.
    let item = &vec1[0];
    //O problema disso é que se acessarmos um indice que não existe o programa vai dar panic.
    //item = &vec1[5];

    println!("{:?}", item);


    //Para evitarmos acessar um indice que não existe podemos usar o metodo get do vetor.
    //Ele retorna uma Option, então podemos ver se é Some() ou None
    let item2 = vec1.get(10);

    //Quando criarmos a variavel item2 dando vec1.get criamos uma refêrencia imutável para o vec1;
    //Então não podemos fazer vec1.push(10) aqui.

    if let Some(value) = item2 {
        println!("{:?}", value);
    }
    else {
        println!("Não tinha nenhum valor no indice.");
    }

    //Mas após utilizarmos a variavel item2 podemos alterar o vetor

    vec1.push(10);

    //Loop imutavel do vec1
    for i in &vec1 {
        println!("{}", i);
    }

    //Loop mutavel do vec1
    for i in &mut vec1 {
        //O * é um dereference para seguirmos a referencia e acessarmos o valor.
        *i += 50;
        println!("{}", i);
    }
} // Aqui o vec1 é dropado, sua referencia removida da stack e seus valores apagados do heap.


//Vetores só podem armazenar um tipo.
//Se definirmos um Enum com várias variações dentro dele, podemos usar ele como o tipo do Vec.
//Permitindo termos valores diferentes dentro do Vec, mesmo todos sendo do tipo VecValues.
#[derive(Debug)]
enum VecValues {
    Number(i32),
    String(String),
    Boolean(bool)
}

fn vec_multi_valorado() {

    let mut vec:Vec<VecValues> = Vec::new();

    vec.push(
        VecValues::Number(
            20
        )
    );
    vec.push(
        VecValues::String(
            String::from("hello")
        )
    );
    vec.push(
        VecValues::Boolean(
            true
        )
    );

    println!("{:#?}",vec)

    //[
    //     Number(
    //         20,
    //     ),
    //     String(
    //         "hello",
    //     ),
    //     Boolean(
    //         true,
    //     ),
    //]
}