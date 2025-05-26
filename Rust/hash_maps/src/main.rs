use std::collections::HashMap;
fn main() {


    hash_map_inserts();

    hash_map_get_values();

    hash_map_iterator();

    hash_map_ownership();

}

fn hash_map_inserts() {
    //Hashmaps possuem chaves e valores, as chaves e valores precisam ter um tipo.
    //Os inserts do hashmap só vão poder ser desses tipos.

    //Caso você não especifique, o compilador infere no primeiro insert qual os tipos
    let mut map = HashMap::new();

    //Aqui o compilador infere que é <i32,i32>
    map.insert(1, 1);

    //Especificando os tipos
    let mut map1:HashMap<&str,u32> = HashMap::new();

    map1.insert("primeiro", 1);

    //AS CHAVES PRECISAM SER ÚNICAS.
    //No caso abaixo esse insert vai sobrescrever o valor do insert anterior.
    map1.insert("primeiro",2);

    //Para inserirmos uma nova chave:valor no hashmap SOMENTE se a chave não existir podemos utilizar o metodo
    //entry do hashmap, ele retorna uma Entry que representa se um valor existe ou não, o metodo or_insert da Entry
    //insere no hashmap da Entry a chave e valor caso não exista.

    //Aqui ele insere pois a chave "segundo" não existe no hashmap.
    //Nos é retornado uma referencia mutavel para o valor inserido, caso ele seja inserido
    let new_entry = map1.entry("segundo").or_insert(2);

    *new_entry += 1;

    //Já aqui ele não insere.
    //E o valor retornado é o valor que já estava inserido nessa chave.
    let _new_entry1 = map1.entry("primeiro").or_insert(3);

    println!("{:?}",map);
    println!("{:?}",map1);



}

fn hash_map_get_values() {
    let mut map:HashMap<&str,u32> = HashMap::new();

    map.insert("primeiro", 1);
    map.insert("segundo", 2);

    //O get volta uma Option<&T>.
    //No caso abaixo a gente checa se temos Some(&u32) e voltamos essa referencia caso tenha,
    //Se for none voltamos uma referencia a 0
    let primeiro = match map.get("primeiro") {
        Some(value) => value,
        None => &0
    };

    //Forma mais simples de escrever o match a cima.
    let segundo = map.get("segundo").unwrap_or(&0);


    let not_found = match map.get("terceiro") {
        Some(value) => value,
        None => &0
    };

    println!("{:?}",primeiro);

    println!("{:?}",segundo);
    println!("{:?}",not_found);
}

fn hash_map_iterator () {

    let mut map:HashMap<&str,u32> = HashMap::new();

    map.insert("primeiro", 1);
    map.insert("segundo", 2);
    map.insert("terceiro", 3);

    for (k,v) in &map {
        println!("k {:?} v {}",k,v);

    }
}


fn hash_map_ownership () {

    let key = String::from("person");
    let value:i32 = 10;

    let mut map:HashMap<String,i32> = HashMap::new();

    map.insert(key,value);
    //A partir daqui o hashmap vai pegar a ownership da variavel key, pois String não tem a Trait Copy,
    //Já a variavel value continua no escopo, pois i32 tem a trait Copy, então ela é copiada para dentro do hashmap.

    println!("{}",value);

}