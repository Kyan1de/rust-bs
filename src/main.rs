use rust_bs::*;

fn main() {

    let mut build = BuildSys::new();
    let _id = build.add_command("powershell", &["echo \"this is a test\""]);
    build.run();

    build.outputs.iter().for_each(|o| {
        let p = String::from_utf8(o.stdout.clone()).unwrap();
        println!("{p}");
        let p = String::from_utf8(o.stderr.clone()).unwrap();
        println!("{p}");
    });

}
