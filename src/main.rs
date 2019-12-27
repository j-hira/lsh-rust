extern crate nix;

use std::io::{self, Write};
use std::fs;
use std::env;
use std::process::exit;
use nix::unistd::*;
use std::ffi::CString;
use nix::sys::wait::*;

fn lsh_help() {
    println!("LSH shell written by Rust.");
    println!("You can use there commands and run a program with arguments.");
    println!("> cd");
    println!("> help");
    println!("> exit");
    println!("> ls");
    println!("> pwd");
}

fn lsh_pwd() {
    let path = env::current_dir().unwrap();
    println!("{}", path.display());
}

fn lsh_ls() {
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        let tmp = path.unwrap().path();
        let p_path = tmp.to_str().unwrap().replace("./", "");
        print!("{} ", p_path);
    }
    println!("");
}

fn lsh_launch(args: Vec<&str>) {
    match fork() {
        Ok(ForkResult::Parent { child }) => {
            match waitpid(child, None) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e.to_string()),
            }
        }
        Ok(ForkResult::Child) => {
            let path = CString::new(args[0].to_string()).unwrap();
            let args = if args.len() > 1 {
                CString::new(args[1].to_string()).unwrap()
            } else {
                CString::new("").unwrap()
            };
            match execv(&path, &[&path.clone(), &args]) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e.to_string()),
            }
            exit(0);
        }
        Err(_) => println!("Fork failed"),
    }
}

fn lsh_cd(args: Vec<&str>) {
    if args.len() == 1 {
        eprintln!("lsh: expected argument to cd");
        return;
    }
    match chdir(args[1]) {
        Err(e) => eprintln!("{}", e.to_string()),
        _ => (),
    }
}

fn lsh_execute(args: Vec<&str>) -> i32 {
    if args.len() == 0 {
        return 1;
    }

    match args[0] {
        "cd" => lsh_cd(args),
        "ls" => lsh_ls(),
        "pwd" => lsh_pwd(),
        "help" => lsh_help(),
        "exit" => { return 0 },
         _ => lsh_launch(args),
    }
    return 1;
}

fn lsh_print(buf: &str) {
    print!("{}", buf);
    io::stdout().flush().unwrap();
}

fn lsh_read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf = buf.replace("\n", "");
    buf
}

fn lsh_loop() {
    let mut status = 1;
    while status == 1 {
        lsh_print("> ");
        let line = lsh_read_line();
        let args: Vec<&str> = line.split_whitespace().collect();
        status = lsh_execute(args);
    }
}

fn main() {
    lsh_loop();
}
