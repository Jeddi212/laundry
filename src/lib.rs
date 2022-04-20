pub mod cuci;

pub use cuci::*;

use std::io::{stdin, stdout, Write};

pub fn read(input: &mut String) {
    input.clear();
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read input");
}

pub fn menu(input: &mut String) {
    print!("
Silahkan Pilih Menu:
1. Cuci Pakaian
2. Bayar Cucian
3. Exit
-> ");

read(input);

}

pub fn menu_cuci(input: &mut String, id_antrian: &mut u16) -> (Pesanan, bool) {
    
    print!("Nama: ");
    let mut nama = String::new();
    read(&mut nama);

    let mut next = true;
    let mut pesanan: Pesanan = Pesanan::new(0, nama.trim().to_owned());

    while next {
        
        print!("
Pilih Pakaian
1.  Baju
2.  Celana
3.  Jaket
4.  Jeans
5.  Sprei
6.  Selimut
7.  Bantal
8.  Guling
9.  Proses (p)
10. Cancel
-> ");

        read(input);
        
        let mut jumlah = String::new();
        
        if "12345678".contains(input.trim()) {
            // Ambil Jumlah Pakaian
            print!("Jumlah: ");
            read(&mut jumlah);
        }
        
        match input.trim() {
            "1" => {
                let jumlah = jumlah.to_string().trim().parse::<u8>().expect("The input should be a number");
                pesanan.cucian.push(Barang::new("Baju".to_owned(), jumlah, 80));
            }
            "2" => {
                let jumlah = jumlah.to_string().trim().parse::<u8>().expect("The input should be a number");
                pesanan.cucian.push(Barang::new("Celana".to_owned(), jumlah, 100));
            }
            "3" => {
                let jumlah = jumlah.to_string().trim().parse::<u8>().expect("The input should be a number");
                pesanan.cucian.push(Barang::new("Jaket".to_owned(), jumlah, 160));
            }
            "4" => {
                let jumlah = jumlah.to_string().trim().parse::<u8>().expect("The input should be a number");
                pesanan.cucian.push(Barang::new("Jeans".to_owned(), jumlah, 275));
            }
            "5" => {
                let jumlah = jumlah.to_string().trim().parse::<u8>().expect("The input should be a number");
                pesanan.cucian.push(Barang::new("Sprei".to_owned(), jumlah, 1500));
            }
            "6" => {
                let jumlah = jumlah.to_string().trim().parse::<u8>().expect("The input should be a number");
                pesanan.cucian.push(Barang::new("Selimut".to_owned(), jumlah, 1245));
            }
            "7" => {
                let jumlah = jumlah.to_string().trim().parse::<u8>().expect("The input should be a number");
                pesanan.cucian.push(Barang::new("Bantal".to_owned(), jumlah, 350));
            }
            "8" => {
                let jumlah = jumlah.to_string().trim().parse::<u8>().expect("The input should be a number");
                pesanan.cucian.push(Barang::new("Guling".to_owned(), jumlah, 400));
            }
            "9" | "p" => {
                *id_antrian += 1;

                // Set order id ke bill pelanggan
                pesanan.id = *id_antrian;

                // Hitung biaya akhir
                pesanan.hitung_biaya();

                next = false;
            }
            _ => {
                break;
            },
        }
    }

    (pesanan, next)

}

pub fn menu_bayar(input: &mut String, antrian: &Vec<Pesanan>) -> (u16, bool) {
    print!("
1. Proses pembayaran
2. Cancel
-> ");

    read(input);

    if input.trim() == "1" {
        let mut order = String::new();
        
        // Print all Order
        for pesanan in antrian {
            pesanan.header();
        }
        
        read(&mut order);
        let order = order.to_string().trim().parse::<u16>().expect("The input should be a number");

        // Dikurang 1 untuk aray 0
        return (order - 1, true);
    }

    (0, false)
}