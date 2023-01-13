use std::collections::BTreeMap;

fn main() {
    let mut voc = BTreeMap::new();
    voc.insert(3_333_3333, "A");
    voc.insert(1_300_000, "M");
    voc.insert(540_000, "E");
    voc.insert(469_000, "D");
    voc.insert(266_000, "H");
    voc.insert(173_000, "R");

    for (guilders, kamer) in &voc {
        println!("{} の出資額は {}", kamer, guilders);
    }

    println!("小規模な出資");

    for (_g, kamer) in voc.range(0..500_000) {
        println!("{}", kamer)
    }
}
