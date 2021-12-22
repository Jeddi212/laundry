pub struct Pesanan {
    id: u32,
    nama: String,
    cucian: Vec<Barang>,
    biaya: f64,
}

pub struct Barang {
    barang: String,
    jumlah: u8,
}