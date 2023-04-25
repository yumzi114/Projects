use std::env;
use dirs;
use std::path::{Path, PathBuf};
use std::io;
use std::process::{Command, Stdio};
use colored::Colorize;
use terminal_menu::*;


fn default (){
    let default_pack=["linux-headers","dkms","base-devel","terminus-font",
    "noto-fonts-cjk","ttf-dejavu","tldr","ibus","ibus-hangul","mesa","mesa-utils","lib32-mesa"];
    runas::Command::new("pacman")
    .arg("-Sy")
    .args(&default_pack)
    .status()
    .unwrap();
}
fn yay (root:&str){
    assert!(env::set_current_dir(&root).is_ok());
    let yay_path = Path::new("./yay-git");
    if !yay_path.is_dir(){
        Command::new("git")
        .arg("clone")
        .arg("https://aur.archlinux.org/yay-git.git")
        .status()
        // .spawn()
        .unwrap();
        // .expect("failed to execute process");
    }
    let yaybin = Path::new("/bin/yay").is_file();
    if !yaybin{
        assert!(env::set_current_dir(&yay_path).is_ok());
        Command::new("makepkg")
        .arg("-si")
        .arg("--noconfirm")
        .status()
        .unwrap();
    }
}
fn mymenu (){
    use crossterm::style::Color;
    let menu = menu(vec![
        // label("archlinux default set").colorize(Color::Red),
        label("archlinux default set").colorize(Color::Blue),
        label("press the back or hit 'q' or esc!").colorize(Color::Red),
        list("pacman ParallelDownloads", vec!["None", "True", "False"]),
        list("show system boot log", vec!["None", "True", "False"]),
        button("set"),
        button("back"),
    ]);
    run(&menu);
    println!("Selected: {}", mut_menu(&menu).selected_item_name());
    // println!("{}", mut_menu(&menu).);


}
fn main() {
    let homedir = dirs::home_dir().expect("Fail not found home");
    let root = Path::new(&homedir);
    assert!(env::set_current_dir(&root).is_ok());
    let temp = env::current_dir().expect("Fail working dir");
    let pwd =  temp.to_str().expect("Fail can't dir to string");
    loop {
        let mut ask = String::new();
        println!("choise work num(working dir:{}) ",pwd.bold().red());
        println!("First set {} plz","pacman speed[5]".bold().blue());
        println!("{} (default{} yay{} wildan(Hyprland){} Nvidia{} set menu{} exit{})",
        "install".bold().green(),"[1]".bold().green(),"[2]".bold().green(),"[3]".bold().green(),
        "[4]".bold().green(),"[5]".bold().green(),"[0]".bold().red()
    );
        io::stdin().read_line(&mut ask)
            .expect("error");
        let ask=match ask.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        match ask{
            1=>default(),
            2=>yay(pwd),
            3=>println!("3"),
            4=>println!("4"),
            5=>mymenu(),
            0=>break,
            _=>println!("{}","Not found menu".bold().red()),
        }
    }
}
