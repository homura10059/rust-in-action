use std::collections::HashMap;

fn main() {
    let mut capitals = HashMap::new();
    capitals.insert("Cook Islands", "Avarua");
    capitals.insert("Fuji", "Suva");
    capitals.insert("Kiribati", "South Tarawa");
    capitals.insert("Niue", "Alofi");
    capitals.insert("Tonga", "Nuku'alofa");
    capitals.insert("Yuvalu", "Funafuti");

    let tongan_capital = capitals["Tonga"];
    println!("Tongaの首都は {} ", tongan_capital);
}
