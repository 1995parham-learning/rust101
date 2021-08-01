mod contact;
mod phonebook;

use std::io::Write;

use crate::contact::Contact;
use crate::phonebook::PhoneBook;

fn main() {
    let mut phonebook = PhoneBook::new();
    let stdin = std::io::stdin();

    loop {
        print!("> ");
        let mut input = String::new();

        std::io::stdout()
            .flush()
            .expect("flushing the output failed");

        stdin.read_line(&mut input).unwrap();

        let mut cmds = input.split_whitespace();

        match cmds.next() {
            Some(cmd) => match cmd {
                "set" => {
                    let mut contact = Contact {
                        first_name: String::from(""),
                        last_name: String::from(""),
                        phone: String::from(""),
                        id: String::from(""),
                    };

                    if let Some(first_name) = cmds.next() {
                        contact.first_name = String::from(first_name);
                    } else {
                        println!("set <first_name> <last_name>");
                        continue;
                    }

                    if let Some(last_name) = cmds.next() {
                        contact.last_name = String::from(last_name);
                    } else {
                        println!("set <first_name> <last_name> <phone> <id>");
                        continue;
                    }

                    if let Some(phone) = cmds.next() {
                        contact.phone = String::from(phone);
                    } else {
                        println!("set <first_name> <last_name> <phone> <id>");
                        continue;
                    }

                    if let Some(id) = cmds.next() {
                        contact.id = String::from(id);
                    } else {
                        println!("set <first_name> <last_name> <phone> <id>");
                        continue;
                    }

                    phonebook.insert(contact);
                }
                "get" => {
                    if let Some(id) = cmds.next() {
                        match phonebook.get(id) {
                            Some(contact) => {
                                println!("first_name: {}", contact.first_name);
                                println!("last_name: {}", contact.last_name);
                                println!("phone: {}", contact.phone);
                                println!("id: {}", contact.id);
                            }
                            None => {
                                println!("there is no contact with {}", id);
                                continue;
                            }
                        }
                    } else {
                        println!("get <id>");
                        continue;
                    }
                }
                "exit" | "quit" => break,
                _ => {
                    println!("unkown command {}", cmd);
                    continue;
                }
            },
            None => {
                println!("you need to specify a command");
                continue;
            }
        }
    }
}
