fn main() {



    string_literal();
    string_slice();
    string_type();
    string_indexing();


}


fn string_literal() {
    //Strings em rust são UTF-8, então podemos ter caracteres de diversos alfabetos.
    //Temos dois tipos de strings;

    //Uma string literal, os dados desse tipo são guardados diretamente no código compilado,
    //pois ele não muda na execução.
    //Abaixo é uma referencia a um string literal.
    //String literais também são strings slices (&str) o unico quesito diferente é que
    //o lifetime dele é &'static, ou seja, NÃO MUDA EM TEMPO DE EXECUÇÃO.
    let string_literal = "Meu string literal";

    println!("{} {}", stringify!(string_literal), string_literal);
}

fn string_slice() {

    let string = String::from("Meu string slice");

    //O string slice é diferente do string literal no quesito de poder mudar em tempo de execução.

    let mut string_slice = &string[..3];

    //Além de ele poder conter TODA a string ou parte dela.
    let slice_complete = &string[..];



    println!("{} {}",stringify!(string_slice) ,string_slice);
    println!("{} {}",stringify!(slice_complete), slice_complete);

    //Mudando o slice em tempo de execução.
    string_slice = &string[4..];

    println!("{} {}",stringify!(string_slice) ,string_slice);
}

fn string_type() {
    //O tipo String é armazenado no heap, pois ele pode crescer ou mudar, diferente do literal.
    //A string abaixo está sendo criada de uma string literal.
    let mut string_type = String::from("Meu String Type");

    //Podemos criar uma string vazia para guardarmos valores depois.
    let string_vazia = String::new();

    //Adicionando valores em uma string.
    string_type.push_str(" atualizado");

    //Concatenando duas strings.
    let string_add = String::from("Oie") + &String::from("mundo");

    let x = "x";
    let y = "y";
    let z = "z";

    //Podemos usar o macro format para juntar as strings em algum padrão também
    let string_format = format!("{x}-{y}-{z}");

    println!("{} {}", stringify!(string_type), string_type);
    println!("{} {}",stringify!(string_vizia) ,string_vazia);
    println!("{} {}",stringify!(string_add) ,string_add);
    println!("{} {}",stringify!(string_format) ,string_format);
}

fn string_indexing() {

    //Em rust o tamanho de uma string não é a quantidade de caracteres que ela possui e sim a quantidade de bytes necessarios
    //para encodar ela em UTF-8

    let string = String::from("ÇÇÇ");

    //A string abaixo tem o tamanho de 6 bytes, isso pois cada "Ç" ocupa dois bytes.
    println!("{:#?}", &string.as_bytes().len());

    //Podemos usar um string slice para pegar o primeiro "Ç" da string,
    //como ele ocupa dois bytes, vamos precisar pegar do inicio até o segundo byte.
    println!("{}", &string[..2]);

    //Podemos iterar pelos caracteres
    for c in string.chars() {
        println!("{c}");
    }

    //Ou pelos bytes
    for b in string.bytes() {
        println!("{b}");
    }
}