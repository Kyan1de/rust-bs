use rust_bs::*;

fn main() {

    let mut bs = BuildConfig::load("./test.txt").unwrap();
    
    bs.run();

    println!("{:?}", bs.outputs);
    bs.outputs.iter_mut().for_each(|a|{println!("{}", String::from_utf8(a.stdout.clone()).unwrap())});

}
