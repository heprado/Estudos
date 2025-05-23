fn main() {


    let x = Some("OIEEEEE");



    match x {
        Some(str) => println!("match tem valor {}", str), //Printa
        None => () //Faz nada
    }

    //Se não nos importamos com o None podemos usar isso ao em vez de escrever
    //None => ()
    if let Some(str)  = x {
        println!("if let tem valor {}", str); // Printa
    }

    let mut message = Message::Receive;

    //Os dois abaixo são equivalentes.
    //O else do if lef é executado em todos os casos que a variável message não for Message::Send
    match message {
        Message::Send => println!("Mensagem para enviar"),
        _ => println!("Mensagem não é para enviar, pode ser Quit ou Receive")
    }

    if let Message::Send = message {
        println!("Mensagem para enviar");
    }
    else {
        println!("Mensagem não é para enviar, pode ser Quit ou Receive");
    }

    message = Message::Send;

    if let Some(message) = message.pode_enviar("Teste") {
        println!("Enviar essa mensagem")
    }
    else {
        println!("Mensagem não pode enviar.")
    }
}

enum Message {
    Quit,
    Send,
    Receive
}

impl Message {

    fn pode_enviar(&self,str:&str) -> Option<String> {

        //Checando se self é do tipo Message::Send,
        //Se não for já retorna None;
        let Message::Send = self else {
            return None;
        };

        //Aqui só vai ser executado se for Message::Send.
        Some(String::from(str))

    }
}

