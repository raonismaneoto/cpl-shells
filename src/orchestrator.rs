use std::{fs::{self, OpenOptions}, process::Command, result};
use std::io::prelude::*;

pub trait LanguageShellOrchestrator {
    fn new() -> Self;
    fn run(&self, command: String) -> bool;
    fn clear(&self);
}

pub struct CSharpOrchestrator {
    program_path: String
}

impl LanguageShellOrchestrator for CSharpOrchestrator {
    fn new() -> Self {
        Command::new("mkdir")
            .arg("/tmp/c#-shell/")
            .output()
            .expect("failed to execute project directory creation");
        
        Command::new("dotnet")
            .arg("new")
            .arg("console")
            .arg("-o")
            .arg("/tmp/c#-shell")
            .output()
            .expect("failed to execute project creation");

        Self {
            program_path: String::from("/tmp/c#-shell/Program.cs")
        }
    }

    fn run(&self, cmd: String) -> bool {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.program_path.clone())
            .unwrap();

        if let Err(e) = writeln!(&file, "{}", cmd) {
            return false;
        }

        let mut run_command = Command::new("dotnet")
            .arg("run")
            .arg("--project")
            .arg("/tmp/c#-shell")
            .output();

        match run_command {
            Ok(result) =>  {
                println!("{}", String::from_utf8(result.stdout).unwrap());
                true
            },
            Err(err) => {
                println!("{}", err);
                false
            }
        }

    }

    fn clear(&self) {}
}
