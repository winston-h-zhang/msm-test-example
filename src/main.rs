use std::io::Read;

use abomonation::Abomonation;
// use ark_ff::PrimeField;

fn read_abomonated<T: Abomonation + Clone>(name: String) -> std::io::Result<T> {
    use std::fs::OpenOptions;
    use std::io::BufReader;

    let arecibo = home::home_dir().unwrap().join(".arecibo_witness");

    let data = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(arecibo.join(name))?;
    let mut reader = BufReader::new(data);
    let mut bytes = vec![];
    reader.read_to_end(&mut bytes)?;

    let (data, _) = unsafe { abomonation::decode::<T>(&mut bytes).unwrap() };

    Ok(data.clone())
}

/// cargo run --release
fn main() {
    println!("Hello, world!");

    let i = 4;
    let witness_i =
        read_abomonated::<Vec<[u64; 4]>>(i.to_string()).unwrap();
    let witness_i = unsafe { std::mem::transmute::<Vec<_>, Vec<ark_bn254::Fr>>(witness_i) };

    println!("success! witness_i.len() = {}", witness_i.len());
}
