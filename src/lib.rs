#![feature(process)]
#![feature(collections)]

use std::str::StrExt;
use std::string::FromUtf8Error;
use std::process::{Command, ExitStatus};

fn verify_status(status: ExitStatus) {
    if !status.success() {
        panic!("Nonzero exit status: {}", status);
    }
}

fn run(cmd: &mut Command) {
    println!("Running: {:?}", cmd);
    match cmd.status() {
        Ok(status) => verify_status(status),
        Err(e) => panic!("Failed to spawn process: {}", e),
    };

}

fn get_output(cmd: &mut Command) -> Result<String, FromUtf8Error> {
    println!("Running: {:?}", cmd);
    match cmd.output() {
        Ok(output) => {
            verify_status(output.status);
            String::from_utf8(output.stdout)
        },
        Err(e) => panic!("Failed to retrive rustc version: {}", e)
    }
}

pub fn fetch() {
    let mut get_version = Command::new("rustc");
    get_version.arg("--version");

    let hash = get_output(&mut get_version)
        .unwrap()
        .split(' ')
        .nth(2)
        .unwrap()
        .trim_matches('(')
        .to_string();
    println!("hash={}", hash);

    let mut mkdir_rust = Command::new("mkdir");
    mkdir_rust.arg("-p").arg("rust");
    run(&mut mkdir_rust);

    let url = String::from_str("https://github.com/rust-lang/rust/tarball/") + &hash[..];
    let mut download_rust = Command::new("wget");
    download_rust.arg("-q")
        .arg("-O")
        .arg("rust/rust.tar.gz")
        .arg(&url[..]);
    run(&mut download_rust);

    let mut untar_rust = Command::new("tar");
    untar_rust.arg("-zxf")
        .arg("rust/rust.tar.gz")
        .arg("-C")
        .arg("rust")
        .arg("--strip-components=1");
    run(&mut untar_rust);
}
