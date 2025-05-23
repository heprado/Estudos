
fn main() {

    //Instanciando a struct
    let user1 = User {
        active:true,
        email:String::from("teste@teste.com"),
        name:String::from("teste"),
        sign_in_count:1,
    };

    let email = String::from("teste2@teste.com");
    let name = String::from("teste2");

    //Se a variavel tiver o mesmo nome do campo da struct, podemos so passar ela,
    //evitando ter que escrever email:email ou name:name
    let user2 = User {
        active:true,
        email,
        name,
        sign_in_count:1,
    };

    //Podemos utilizar os valores de uma struct e passar para os campos serem inicializados.
    //Aqui o user3 terá um nome diferente, mas todos os outros campos como active,email e sign_in_count serão os mesmos do user1;
    let user3 = User {
        name:String::from("teste3"),
        ..user1
    };

    let numeros = Numeros(10,10,10,255,256);

    let robot = Robot;

    println!("{}",user1.name);
    println!("{}",user2.name);
    println!("{}",user3.name);
    println!("{}",user3.email);
    println!("{}",user3.active);
    println!("{}",user3.sign_in_count);
    println!("{:?}",numeros.0);
    println!("{:?}",numeros.1);
    println!("{:?}",numeros.2);
    println!("{:?}",numeros.3);
    println!("{:?}",numeros.4);

    robot.beep();

    robot.boop();

    robot.falar();

    let rec1 = Rectangle {width:10, height:20};

    let rec2 = Rectangle::square(20);

    let pode = rec1.can_hold(&rec2);

    match pode {
        true =>  println!("{:#?} cabe no  {:#?}",rec2,rec1),
        false =>  println!("{:#?} não cabe no {:#?}",rec2,rec1)
    }



}



struct User {
    active: bool,
    email: String,
    name: String,
    sign_in_count: u64,
}

//Também podemos definir structs parecidas com tuplas, tirando que podemos ter valores diferentes.
struct Numeros (u8,u8,u8,u8,u16);

//Podemos ter structs sem campos também, elas podem ser usadas para implementar uma função ou trait dps.
struct Robot;

trait BeepBoop {
    fn beep(&self) {
        println!("beep");
    }
    fn boop (&self) {
        println!("boop");
    }
}

impl Robot {
    fn falar(&self) {
        println!("Eu sou um robo.");
    }
}

impl BeepBoop for Robot {}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

//Podemos ter duas implementações de uma struct.
impl Rectangle {
    //Retorna um quadrado do tipo Rectangle
    //Self significa Rectangle, ele sempre vai ser do tipo que estamos implementando
    //Metódos que retornam self devem ser chamados com ::

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}