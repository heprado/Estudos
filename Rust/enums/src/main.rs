fn main() {
    let v4 = IpAddress::V4(
        Ipv4Address {string:String::from("192.168.0.1")}
    );

    let v6 = IpAddress::V6(
        Ipv6Address{string:String::from("2001:fefe::1")}
    );

    v4.print_ip();

    v6.print_ip();

    optional_enum()
}

#[derive(Debug)]
struct Ipv4Address{
    string:String
}

#[derive(Debug)]
struct Ipv6Address {
    string:String
}

#[derive(Debug)]
enum IpAddress {
    V4(Ipv4Address),
    V6(Ipv6Address),
}

//Podemos implementar metodos para o enum também,
//só que precisamos sempre checar o tipo.
impl IpAddress {
    fn print_ip(&self) {
        match self {
            IpAddress::V4(v4) => println!("IPv4 address: {}", v4.string),
            IpAddress::V6(v6) => println!("IPv6 address: {}", v6.string)
        }
    }
}

fn optional_enum () {
    let mut ia: Option<&str> = Some("TETÃO DE ANIME GERADO POR IA");

    //Para acessarmos o valor de uma option a gente precisa lidar com todas as possibilidades (Some e None).

    match ia {
        Some(value) => println!("CRIE 2 TRILHÕES DE DATACENTER NO MEU PAIS PRA GERAR 2 TRILHOES DE {}",value),
        None => println!("AFF QUE IA SEM GRAÇA"),
    }

    ia = None;

    match ia {
        Some(value) => println!("CRIE 2 TRILHÕES DE DATACENTER NO MEU PAIS PRA GERAR 2 TRILHOES DE {}",value),
        None => println!("AFF QUE IA SEM GRAÇA"),
    }

}