
fn main() {
    let i = isize::MAX;
    let u = usize::MAX;

    let a = isize::BITS;
    let b = usize::BITS;

    println!("Maiores números baseados na arquitetura");

    println!("Maior número com sinal : {}\n\
    Maior número sem sinal: {}\n\
    Quantidade de bits do maior numero com sinal: {}\n\
    Quantidade de bits do maior numero sem sinal: {}", i, u , a , b);
}
