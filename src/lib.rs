pub mod cuci;

use std::io::{stdin, stdout, Write};

pub fn read(input: &mut String) {
    input.clear();
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read input");
}

pub fn menu(input: &mut String) -> bool {
    print!("
Silahkan Pilih Menu:
1. Cuci Pakaian
2. Bayar Cucian
3. Exit
-> ");

read(input);

    if input.trim() == "1" {
        menu_cuci(input);
        return true;
    } else if input.trim() == "2" {
        menu_bayar(input);
        return true;
    }
    false
}

pub fn menu_cuci(input: &mut String) {
    
    let mut next = true;

    while next {
        
        println!("
Pilih Pakaian
1. Baju
2. Celana
3. Jaket
4. Jeans
5. Sprei
6. Selimut
7. Bantal
8. Guling
9. Cancel
-> ");

        read(input);

    }

}

pub fn menu_bayar(input: &mut String) {
    println!("
1. Proses pembayaran
2. Cancel
-> ");
}