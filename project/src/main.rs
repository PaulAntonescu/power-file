use std::process::Command;
use std::{fs, io, io::Write, env};

static EXE_FILE_PATH: &str = "C:\\Windows\\PowerFile\\.executables\\";

fn main() {
    let files = get_executable_files();

    print_executable(&files);

    loop {
        let index = match user_index() {
            Some(v) => v,
            None => continue
        };
        
        match files.get(index) {
            Some(file) => {
                file.run();
                break;
            },
            None => {
                println!("Index: {index} is out of bounds!");
                continue;
            }
        }
    }
}

pub struct ExecutableFile {
    name: String
}

impl ExecutableFile {
    pub fn new(name: &str) -> ExecutableFile {
        ExecutableFile {
            name:  name.to_string()
        }
    }

    fn run(&self) {
        let executable_command = self.create_executable_command();

        match Command::new("powershell.exe")
            .args(["start", "powershell.exe", &executable_command])
            .spawn() {
                Ok(_) => { }
                Err(error) => { panic!("Problem has occured when spawning powershell {:?}", error) }
            }
    }

    fn create_executable_command(&self) -> String {
        let mut selected_files: Vec<String> = env::args().collect();
        selected_files.remove(0);
        let selected_files: Vec<String> = selected_files.iter().map(|f| format!("\'{f}\'")).collect();

        let arg = selected_files.join(",");
        let executable_command = format!("\"{}{} {}\"", EXE_FILE_PATH, &self.name, arg);

        executable_command
    }
}

fn get_executable_files() -> Vec<ExecutableFile> {
    let mut exe_files: Vec<ExecutableFile> = Vec::new();

    let files = match fs::read_dir(EXE_FILE_PATH) {
        Ok(v) => v,
        Err(error) => {
            println!("Problem has occured when accessing files: {:?}", error);
            let mut dummy = String::new();
            io::stdin().read_line(&mut dummy).unwrap();
            panic!()
        }
    };
    
    for file in files {
        exe_files.push(
            ExecutableFile::new(file.unwrap().file_name().to_str().unwrap())
        );
    }

    exe_files
}

fn print_executable(files: &Vec<ExecutableFile>) {
    println!("");
    println!("To add to PowerFile go to '{EXE_FILE_PATH}' and add!");
    println!("----------------------------------------------------------------------------------");
    for (i, file) in files.iter().enumerate() {
        println!("{} {}", i, file.name);
    }
    println!("----------------------------------------------------------------------------------");
    print!("index: ");
    io::stdout().flush().unwrap();
}

fn user_index() -> Option<usize> {
    let mut index = String::new();
    match io::stdin().read_line(&mut index) {
        Ok(_) => { }
        Err(error) => {
            println!("Problem has occured when reading input: {error}");
            return None;
        }
    }
    let index = match index.trim().parse::<usize>() {
        Ok(v) => { v }
        Err(_) => {
            println!("Not a valid int");
            return None;
        }
    };

    Some(index)
}