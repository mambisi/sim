use notus::nutos::Notus;

fn main() {
    let n = 10_000_000;
    let notus = Notus::temp("./temp_dir").unwrap();
    for i in 0..n {
        if i % 100_000 == 0 {
            println!("Block Level {}", i);
        }
        let i = (i % u8::MAX as u32) as u8;
        let key = vec![i;32];
        let value = vec![i;32];
        notus.put(key,value).unwrap();
    }
}
