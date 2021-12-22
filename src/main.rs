use laundry::*;

fn main() {

    let mut antrian: Vec<Pesanan> = Vec::new();
    let mut input = String::new();
    let mut id_antrian = antrian.len() as u16;

    loop {
        // Welcoming User and Show the menu
        welcome(&mut input);

        if input.trim() == "1" {
            match menu_cuci(&mut input, &mut id_antrian) {
                (p, false) => {
                    antrian.push(p);
                    println!("Cucian berhasil didaftarkan\n=======================\n")
                },
                (_, true) => println!("\nPendaftaran Dibatalkan\n=======================\n"),
            }
        } else if input.trim() == "2" {
            match menu_bayar(&mut input, &antrian) {
                (order, true) => {
                    let buf_pesanan = antrian.get(order as usize).expect("Pesanan tidak terdaftar");
                    buf_pesanan.bill();
                },
                (_, false) => println!("\nPembayaran Dibatalkan\n=======================\n"),
            }
        } else {
            println!("\nThank you\n");
            break;
        }

    }

}

fn welcome(input: &mut String,) {
    println!("
Selamat datang ke Rusty Laundry!!!
----------------------------------");
    menu(input);
}
