mod parse;
use std::{env, fs};
use dirs::home_dir;
use std::path::{Path, PathBuf};
use clap::{Parser};
#[cfg(windows)]
use windows_elevate::{check_elevated, elevate};
#[cfg(unix)]
use sudo::escalate_if_needed;
#[cfg(unix)]
use permissions::{is_readable, is_writable};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Directory (cargo project -> cargo new) or file (cargo +nightly -Zscript)
    #[arg(short, long)]
    mode: Option<String>,
}

#[cfg(unix)]
fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
    std::os::unix::fs:: symlink(from, to)?;
    Ok(())
}
#[cfg(windows)]
fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
    junction::create(from, to)?;
    Ok(())
}

#[cfg(unix)]
fn admin_privileges () {
    escalate_if_needed().expect("Fallo al elevar privilegios");
}
#[cfg(windows)]
fn admin_privileges () {
    let is_elevated = check_elevated().expect("Failed to call check_elevated");

    if !is_elevated {
        elevate().expect("Failed to elevate");
    }
}

fn main() {
    let cli = Cli::parse();
    let path = env::current_dir().unwrap();
    let mut cache_path:PathBuf = home_dir().unwrap();
    cache_path.push(".cargo-cache");
    
    let dirs=vec!["debug","release"];
    let dirs_in_dirs=vec![".fingerprint","build","deps"];

    if !cache_path.exists() {
        let path:&str= &cache_path.to_string_lossy();
        fs::create_dir(path).expect("Failed to create $HOME/.cargo-cache");

        for dir in dirs.clone() {
            #[cfg(windows)]
            fs::create_dir(path.to_owned()+r#"\"#+dir).expect("Failed to create $HOME/.cargo-cache");
            #[cfg(unix)]
            fs::create_dir(path.to_owned()+"/"+dir).expect("Failed to create $HOME/.cargo-cache");

            for subdir in dirs_in_dirs.clone(){
                #[cfg(windows)]
                fs::create_dir(path.to_owned() +r#"\"#+dir+r#"\"#+subdir).expect("Failed to create $HOME/.cargo-cache");
                #[cfg(unix)]
                fs::create_dir(path.to_owned()+"/"+dir+"/"+subdir).expect("Failed to create $HOME/.cargo-cache")
            }
        }
    }

    match cli.mode{
        Some(String)=>{

        }
        None => {
            if path.to_string_lossy().contains("src") || path.to_string_lossy().contains("target") {
                println!("Please run this command on the root of the rust project");
                std::process::exit(1);
            }

            let mut target_path=path.clone();
            target_path.push("target");


            if !target_path.exists(){
                fs::create_dir(&target_path).unwrap();
            }

            // get admin privileges if necessary
            #[cfg(windows)]
            admin_privileges();
            #[cfg(unix)] {
                if !is_writable(target_path.clone()).unwrap() || !is_readable(target_path.clone()).unwrap(){
                    admin_privileges();
                }
            }

            for dir in dirs.clone() {
                #[cfg(windows)]
                let the_path:String=target_path.to_string_lossy().as_ref().to_owned() +r#"\"#+dir;
                #[cfg(unix)]
                let the_path:String=target_path.to_string_lossy().as_ref().to_owned()+"/"+dir;
                if !PathBuf::from(the_path.clone()).exists(){
                    fs::create_dir(the_path.clone()).unwrap();
                }
                for subdir in dirs_in_dirs.clone(){
                    #[cfg(windows)]
                    let the_subpath:String=the_path.clone()+r#"\"#+subdir;
                    #[cfg(windows)]
                    let cache_subpath:String=cache_path.to_string_lossy().as_ref().to_owned()+r#"\"#+dir+r#"\"#+subdir;
                    #[cfg(unix)]
                    let the_subpath=the_path.clone()+"/"+subdir;
                    #[cfg(unix)]
                    let cache_subpath:String=cache_path.to_string_lossy().as_ref().to_owned()+"/"+dir+"/"+subdir;
                    if PathBuf::from(the_subpath.clone()).exists(){
                        fs::remove_dir_all(the_subpath.clone()).unwrap();
                    }
                    symlink_dir(cache_subpath,the_subpath).expect("Failed while creating simbolic links");
                }
            }
            println!("Successfully connected cache to this project");
        }
    }
}