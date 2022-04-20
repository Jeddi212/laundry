pub struct Pesanan {
    pub id: u16,
    pub nama: String,
    pub cucian: Vec<Barang>,
    pub biaya: u64,
}

impl Pesanan {
    pub fn new(id: u16, nama: String) -> Self {
        Pesanan {
            id,
            nama,
            cucian: vec![],
            biaya: 0,
        }
    }

    pub fn hitung_biaya(&mut self) {
        for barang in self.cucian.iter() {
            self.biaya = self.biaya + barang.hitung();
        }
    }

    pub fn header(&self) {
        print!("
    Id   : {}
    Nama : {}
    -> ", self.id, self.nama);
    }

    pub fn bill(&self) {
        println!("
  ----------------------
    Bill   |
  ----------------------
    Id     : {}
    Nama   : {}
    Cucian : {:#?}
    biaya  : {}
  ----------------------
  ", self.id, self.nama, self.cucian, self.biaya);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Barang {
    barang: String,
    jumlah: u8,
    harga: u64,
}

impl Barang {
    pub fn new(barang: String, jumlah: u8, harga: u64) -> Self {
        Barang {
            barang,
            jumlah,
            harga,
        }
    }

    pub fn hitung(&self) -> u64 {
        (self.jumlah) as u64 * self.harga
    }
}
