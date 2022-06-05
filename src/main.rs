use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Sayıyı tahmin et!");

    let gizli_sayi = rand::thread_rng().gen_range(1..101);

    println!("Gizli sayı : {}", gizli_sayi);

    loop {
        println!("Lütfen tahmininizi girin.");

        let mut tahmin = String::new();

        io::stdin()
            .read_line(&mut tahmin)
            .expect("Satır okunamadı!");

        let tahmin: u32 = match tahmin.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tahminin: {}", tahmin);

        match tahmin.cmp(&gizli_sayi) {
            Ordering::Less => println!("Çok küçük!"),
            Ordering::Greater => println!("Çok büyük!"),
            Ordering::Equal => {
                println!("Doğru Bildin!");
                break;
            }
        }
    }
}
