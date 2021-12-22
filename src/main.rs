use laundry::*;
use laundry::cuci::*;

fn main() {

    let mut Antrian: Vec<Pesanan> = Vec::new();
    let mut input = String::new();

    // Welcoming User and Show the menu
    welcome(&mut input);

    println!("Hasil :{}", input);

}

fn welcome(input: &mut String) {
    println!("
Selamat datang ke Rusty Laundry!!!
----------------------------------
");
    menu(input);
}
