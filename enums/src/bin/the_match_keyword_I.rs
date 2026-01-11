#![allow(dead_code)]

use chrono::prelude::*;

#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

#[derive(Debug, Copy, Clone)]
enum OS {
    Windows = 1980,
    MacOS = 2001,
    Linux = 1991,
}

fn main() {
    let my_computer = OperatingSystem::MacOS;
    let age = years_since_release(my_computer);
    println!("My computer's operating system is {age} years old");

    let my_os = OS::MacOS;
    println!("My OS is {} years old.", years_since_release_os(&my_os));
    println!("My OS is {my_os:#?}.");
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => 39,
        OperatingSystem::MacOS => 20,
        OperatingSystem::Linux => 25,
    }
}

fn years_since_release_os(os: &OS) -> i32 {
    Local::now().year() - *os as i32
}
