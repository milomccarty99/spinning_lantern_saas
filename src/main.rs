use std::io;
use std::io::{stdin, stdout, Write};
//use std::cmp;
//use termion::color;


fn main() {
    loop {
    
        let mut command: String = String::from("");
        stdin().read_line(&mut command);
        let mut tokenized_command = command.trim().split(' ');
        let mut tc = tokenized_command.collect::<Vec<&str>>();
        //if tokenized_command.count() <= 0 
        //{
        //    continue;
        //}
        let mut cmd = tc[0];
        match cmd {
            "help" => {print_help_msg()},
            "sale" => {println!("selling")},// {tokenized_command}");},
            "invoice" => {println!("invoice for {}",tc[1])},
            "q" => {break;},
            "quit" => {break;},
            _ => {println!("command not recognized:")},// {tokenized_command}")},
        }


    }
}

fn print_help_msg() {
    println!("commands that you can run:");
    println!("help - display this help message");
    println!("sale - start as sale");
    println!("invoice - invoice someone for a product");
    println!("quit - quit the application");

}
