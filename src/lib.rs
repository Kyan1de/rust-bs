use std::{collections::HashMap, io::Write, process::{Command, Output, Stdio}};
use std::fs;
use regex::Regex;

pub type CommandID = usize;

/// somewhat thin wrapper over std::process objects
/// 
/// # Examples
/// 
/// ```
/// use rust_bs::BuildSys;
/// 
/// let mut build = BuildSys::new();
/// let _id = build.add_command("powershell", &["echo", "'this is a test!'"]);
/// build.run();
/// 
/// build.outputs.iter().for_each(|o| {
///     let s = String::from_utf8(o.stdout.clone()).unwrap();
///     assert_eq!(s, "this is a test!\r\n".to_string());
///     let s = String::from_utf8(o.stderr.clone()).unwrap();
///     assert_eq!(s, "".to_string());
/// });
/// ```
#[derive(Debug)]
pub struct BuildSys {
    pub tasks: Vec<Command>, // ordered list of commands to run
    pub outputs: Vec<Output>, // matches order of commands
}

impl BuildSys {

    pub fn new() -> Self {
        Self{tasks:vec![], outputs:vec![]}
    }

    pub fn add_command(&mut self, command_name: &str, args: &[&str]) -> CommandID {
        let mut c = Command::new(command_name);
        c.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
        self.tasks.push(c);
        self.tasks.len() - 1
    }

    pub fn add_arguments(&mut self, command_id: CommandID, args: &[&str]) {
        match self.tasks.get_mut(command_id) {
            Some(c) => {c.args(args);},
            None => {},
        }
    }

    pub fn remove_command(&mut self, _command_id: CommandID) {
        todo!();
    }

    pub fn run(&mut self) {
        self.tasks.iter_mut().for_each(|task|{
            self.outputs = vec![];
            let task_name = task.get_program().to_str().unwrap().to_string();
            let outp: std::process::Output = task.spawn().expect(&format!("failed to run task {task_name}"))
                                                 .wait_with_output().expect(&format!("failed to run task {task_name}"));
            if outp.status.success() {
                println!("{task_name} ran successfully");
                self.outputs.push(outp);
            } else {
                let code = outp.status.code().unwrap();
                let stdout = String::from_utf8(outp.stdout).expect("msg");
                let stderr = String::from_utf8(outp.stderr).expect("msg");
                println!("{task_name} exited with error code {code}");
                println!("STDOUT: \n{stdout}");
                println!("STDERR: \n{stderr}");
            }
            
        });
    }

}


/// saves and loads BuildSys structs
#[derive(Debug)]
pub struct BuildSerializer;

impl BuildSerializer {
    

    pub fn load(file_name: &str) -> Option<BuildSys> {
        
        if fs::exists(file_name).unwrap() {
            let contents = fs::read_to_string(file_name).unwrap();
            let contents: Vec<&str> = contents.split("\n").collect();

            let mut build = BuildSys::new();

            contents.iter().for_each(|l|{
                let l: Vec<&str> = Regex::new("((\\\"|\\\').*?(\\\"|\\\'))|(\\b\\w+\\b)").unwrap()
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


pub enum VarVal {
    String(String),
    Number(f64),
}
pub type VarTable = HashMap<String, VarVal>;
/// generates BuildSys structs from .rbs files 
#[derive(Debug)]
pub struct BuildParser;

// used to construct the AST for the parser
#[derive(Debug)]
pub enum BSAst {
    Prog(Vec<BSAst>), // root node, program
    Ident(String), // identifier
    Batch(Vec<BSAst>), // batch of commands to run at once, once generated
    Stmt(Vec<BSAst>), // statement line, may generate a command or do some logic shit idk
    
    // literals
    Num(String), // number literal
    Str(String), // string literal
    Arr(Vec<BSAst>), // array literal
    
    // operations
    SetVar(Box<BSAst>, Box<BSAst>), // set <Iden> = <Ident||Num||Str||Expr||Generate>
    Expr(Vec<BSAst>), // <term> <+||-||*||/> <term> || <term>
    Term(Vec<BSAst>), // (<expr>) || <Num||Ident||Str>
    Generate(Vec<BSAst>), // gen <((Iden )||(*Iden ))*>
    Unpack(Box<BSAst>), // *iden from the above, unpacks an array
    None
}


impl BuildParser {

    pub fn lex(input: &str) -> Vec<&str> {
        let mut out = vec![];

        input.split("\n").for_each(|l|{
            let mut split: Vec<&str> = Regex::new("(\\\".*?\\\")|(#.*)|[\\+\\-\\*\\/\\=\\(\\)\\[\\]\\,]|(\\b\\S+?\\b)").unwrap()
                                    .find_iter(&l).map(|mat|{
                                        if mat.as_str().ends_with("\r") {&mat.as_str()[..(mat.len()-1)]} else {mat.as_str()}
                                    })
                                    .collect();
            out.append(&mut split);
            out.push("\n");
        });
        
        out
    }

    pub fn parse(input: Vec<&str>) -> BSAst {
        
        let mut clean: Vec<&str> = vec![];

        for token in input.windows(2) {
            match token {
                [A, B] => {
                    if (*A).eq("\n") && (*B).eq("\n") {
                        () // dont push the token if followed immediately by another \n
                    } else if !A.starts_with("#") {
                        clean.push(A);
                    }
                },
                [A] => {
                    if (*A).eq("\n") {
                        () // dont push the token if no tokens follow
                    } else if !A.starts_with("#") {
                        clean.push(A);
                    }        
                },
                _ => {()}
            }
        }

        BSAst::Prog(Self::parse_lines(&mut clean))
    }

    fn parse_lines(global_toks: &mut Vec<&str>) -> Vec<BSAst>{
        let mut statement: Vec<&str>;
        let mut parsed = vec![];
        loop {
            let idx = global_toks.iter().position(|a| (*a).eq("\n")).unwrap_or(0);
            if idx == global_toks.len() {break;}
            (statement, *global_toks) = {
                let (a, b) = global_toks.split_at(idx);
                let (_, b) = b.split_first().unwrap();
                (Vec::from(a), Vec::from(b))
            };
            println!("{:?}", statement);
            parsed.push(Self::parse_part(statement.as_slice(), global_toks));
        }
        parsed
    }

    fn parse_part(statement: &[&str], global_toks: &mut Vec<&str>) -> BSAst {
        match statement {
            ["batch"] => {
                let mut inner: Vec<&str>;
                let idx = global_toks.iter().position(|a| (*a).eq("end")).unwrap_or(0);
                (inner, *global_toks) = {
                    let (a, b) = global_toks.split_at(idx);
                    let (_, b) = b.split_first().unwrap();
                    (Vec::from(a), Vec::from(b))
                };
                BSAst::Batch(Self::parse_lines(&mut inner))
            },
            ["set", iden, "=", tail @ ..] => {
                BSAst::SetVar(Box::new(BSAst::Ident(iden.to_string())), Box::new(Self::parse_part(tail, &mut vec![])))
            },
            ["gen", tail @ ..] => {
                let mut args = vec![];
                let mut unpack_next: bool = false;
                for token in tail {
                    if (*token).eq("*") {unpack_next = true;}
                    else if unpack_next {
                        args.push(BSAst::Unpack(Box::new(BSAst::Ident(token.to_string()))));
                        unpack_next = false;
                    } else {
                        args.push(BSAst::Ident(token.to_string()));
                    }
                }
                BSAst::Generate(args)
            },
            ["[", content @ .., "]"] => {
                let mut internal = vec![]; 
                for element in content.split(|s| (*s).eq(",")) {
                    internal.push(Self::parse_part(element, &mut vec![]));
                }
                BSAst::Arr(internal)
            },
            [single] => {
                match single.chars().nth(0).unwrap() {
                    '\"' => BSAst::Str(single.to_string()),
                    '0'..='9' => BSAst::Num(single.to_string()),
                    _ => BSAst::Ident(single.to_string())
                }
            }
            _ => {BSAst::None},
        }
    }

}
