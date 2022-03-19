use crate::constants::{PMMP_STABLE_PHAR_URL, PMMP_URL};
use colour::e_green_ln;
use std::env::{current_dir, set_current_dir};
use std::fs::{create_dir_all, write};
use std::path::Path;
use std::process::Command;
use ureq::get;

pub fn makeserver(args: Vec<String>) {
    // Command to create a new pmmp directory
    // Can have configured PHP binaries (8 by default since 7.4 is deprecated) and API versions
    // Will also create the phar file
    let dir: &String = args.get(2).expect("No directory specified");
    // Fuck windows :D
    let dir = dir.replace("\\", "");
    let dir = dir.replace(".", "");
    let api = args.get(3).expect("No API version (4 or stable) specified");
    if api != "4" && api != "stable" {
        e_green_ln!("Invalid API version specified");
        return;
    }
    let rel_dir = Path::new(&current_dir().unwrap()).join(&dir);
    if !rel_dir.exists() {
        create_dir_all(&rel_dir).unwrap();
    }
    let mut flags: Vec<&str> = Vec::new();
    flags.push("clone");
    if api == "4" {
        flags.push("--recursive");
    }
    flags.push(PMMP_URL);
    if api == "4" {
        flags.push("--branch=master")
    }
    flags.push(rel_dir.to_str().unwrap());

    e_green_ln!("Cloning PMMP...");

    Command::new("git")
        .args(&flags)
        .output()
        .expect("Failed to clone PMMP");

    if api == "stable" {
        let phar_data = get(PMMP_STABLE_PHAR_URL)
            .call()
            .unwrap()
            .into_string()
            .unwrap();
        e_green_ln!("Writing PHAR file...");
        write(rel_dir.join("Pocketmine-MP.phar"), phar_data)
            .expect("Failed to write Pocketmine-MP.phar");
        e_green_ln!("Created PHAR file")
    } else {
        set_current_dir(rel_dir).unwrap();
        e_green_ln!("Installing composer dependencies...");
        // CD sys call is not actually doing anything (because of rust process)
        // I think i know how to fix this
        Command::new("cd").arg(&dir).output().unwrap();
        Command::new("composer")
            .args(&["--no-dev", "--classmap-authoritative", "install"])
            .output()
            .expect("Failed to install composer dependencies");
        e_green_ln!("Composer dependencies installed");
        e_green_ln!("Building Pocketmine-MP.phar...");
        Command::new("composer")
            .arg("make-server")
            .output()
            .expect("Failed to build Pocketmine-MP.phar");
    }

    // ./pmtools makeserver <dir> <api: 4 | stable>
    // TODO: Make API argument be branch name
}
