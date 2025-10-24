use crate::build_sys::BuildSys;
use std::fs;
use std::io::Write;
use regex::Regex;

/// saves and loads BuildSys structs
#[derive(Debug)]
pub struct BuildSerializer;

impl BuildSerializer {
    
    /// loads a BuildSys from a file
    pub fn load(file_name: &str) -> Option<BuildSys> {
        
        if fs::exists(file_name).unwrap() {
            let contents = fs::read_to_string(file_name).unwrap();
            let contents: Vec<&str> = contents.split("\n").collect();

            let mut build = BuildSys::new();

            contents.iter().for_each(|l|{
                let l: Vec<&str> = Regex::new(r#"((\"|\').*?(\"|\'))|(\b\w+\b)"#).unwrap()
                                    .find_iter(&l).map(|mat|{mat.as_str()})
                                    .collect();
                if l.len() == 0 {return;}
                else if l.len() == 1 {build.add_command(l[0], &[]);}
                else {build.add_command(l[0], &l[1..]);}
            });

            Some(build)
        } else {
            None
        }

    }

    /// writes a BuildSys to a file, returning the file handle
    pub fn write(file_path: &str, build: BuildSys) -> fs::File {

        let mut file = fs::File::create(file_path).unwrap();
        for cmd in build.tasks {
            let command: String = cmd.get_program().to_str().unwrap().to_string();
            let args: String = cmd.get_args().collect::<Vec<_>>().iter()
                                .map(|s|{String::from(s.to_str().unwrap())})
                                .collect::<Vec<_>>().join(" ");
            file.write((command + &String::from(" ") + &args + &String::from("\n")).as_bytes()).unwrap();
        }
        file

    }

}