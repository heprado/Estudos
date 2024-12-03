
fn tuplas() {

    //Caso formos tipar uma variavel que contem uma tupla
    //Precisamos especificar o tipo de cada valor.
    //Os valores podem ser alterados (caso a variavel sejá mutavel),
    //só necessitam ser do mesmo tipo.
    let tup:(u8,u8,u8) = (1,2,3);

    //Descontruindo os valores de dentro da variavel tup em 3 variaveis.
    //Underline somente para o compilador ignorar que não estamos usando essas variaveis.
    let (_x,_y,_z) = tup;

    //Podemos acessar valores das tuplas atráves dos indexes.
    let _primeiro_valor : u8 = tup.0;


    println!("Tupla: {:#?}\n",tup);
}

fn arrays() {
    //Caso formos tipar uma variavel que contem um array
    //Precisamos especificar o tipo dos valores do array e o tamanho dele
    //Arrays possuem tamanhos fixos.
    //Arrays só podem ser de um tipo.
    let arr1: [u8;10] = [1,2,3,4,5,6,7,8,9,10];

    //Podemos também inicializar um array com valores iguais.
    // Precisamos especificar primeiro qual o valor e depois a quantidade.
    // No caso abaixo vamos ter 10 unidades do número 3 dentro do array.
    let arr2 : [u8;10] = [3;10];

    //Podemos acessar valores das tuplas atráves dos indexes.
    let _primeiro_valor : u8 = arr1[0];

    println!("Array 1: {:#?}\n
    Array 2: {:#?}\n",
             arr1,
             arr2
    );
}

fn vetores() {

    //Todos os valores de um Vetor devem possuir o mesmo tipo.
    //A principal diferença entre um vetor e um array é que o
    //array não aumenta conforme colocamos mais valores.
    let mut vec: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10];

    //Colocando mais dois valores no vetor pois é possivel.
    vec.push(11);
    vec.push(12);

    println!("Vetor: {:#?}\n",vec);

}

fn main() {
    tuplas();
    arrays();
    vetores();
}
