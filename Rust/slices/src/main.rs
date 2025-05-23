fn main() {

    let mut string = String::from("Hello world teste");

    let first = first_word(&string);

    println!("{}", first);

    //Slice da string inteira;
    println!("{}", &string[..]);

    //Slice da string inteira menos a primeira letra.
    println!("{}", &string[1..]);

    //Slice da string inteira menos a ultima letra.
    println!("{}", &string[..string.len()-1]);

    let first_slice = first_word_slice(&string);

    println!("{}", first_slice);




}

fn first_word(string:&str) -> usize {

    //A gente itera pois String e &str não tem um metodo iter que retorna cada caracter,
    //precisamos castear elas em bytes e iterar pelos bytes.
    for (i, &item) in string.as_bytes().iter().enumerate() {

        //Se for igual a representação binaria de espaço (\s)
        if item == b' ' {
            //Retorna o index.
            return i;
        }
    }
    string.len()

}

fn first_word_slice(string:&str) -> &str {

    //A gente itera pois String e &str não tem um metodo iter que retorna cada caracter,
    //precisamos castear elas em bytes e iterar pelos bytes.
    for (i,&item) in string.as_bytes().iter().enumerate() {
        if item == b' ' {
            //Retornamos um slice da string toda até o primeiro espaço.
            return &string[..i];
        }
    }

    &string[..]
}

