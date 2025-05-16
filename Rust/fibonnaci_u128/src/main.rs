
fn main() {

    let mut right:u128= 1;
    let mut left:u128 = 0;

    for i in 1..=185 {


        let result:u128 =  right + left;

        println!("{}",result);

        left = right;

        right = result;


    }
}
