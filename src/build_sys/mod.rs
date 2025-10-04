use std::process::{Output, Command, Stdio};

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
