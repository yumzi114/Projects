use std::env;
use std::path::{Path, PathBuf};
use crossterm::style::Stylize;
use dirs;
use regex::Regex;
use clap::{Parser,Command,ArgAction,Arg,Subcommand};
use terminal_menu::{run,label,list,button,menu,mut_menu};


const PACMAN_LIST:[&str;14]= [ "linux-headers","dkms","base-devel","terminus-font",
"noto-fonts-cjk","ttf-dejavu","tldr","ibus","ibus-hangul","mesa","mesa-utils","lib32-mesa", "pciutils","usbutils"];
const GIT_LIST:[&str;1]=["https://aur.archlinux.org/yay-git.git"];

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///default pacman package install
    // #[arg(short, long)]
    // default: Option<String>,
    #[command(subcommand)]
    exes: Option<Commands>,
    // Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}
#[derive(Subcommand)]
enum Commands {
    /// install default pacman package
    default ,
    /// install yay package manager(git - binary)
    yay,
    /// open linux config settings menu
    set,
}
fn main() {
    let homedir = dirs::home_dir();
    match homedir {
        Some(dir)=>{
            assert!(env::set_current_dir(Path::new(&dir)).is_ok());
            let temp = env::current_dir().unwrap();
            let pwd =  temp.to_str().unwrap();
            println!(" HOME directory -{}-", Stylize::bold(pwd).green());
        }
        None=>{println!("Not found {}",Stylize::bold("HOME dir").red())}        
    }
    let args = Args::parse();
    match &args.exes {
        Some(Commands::default)=>{
            println!("install default pacman package");
        }
        Some(Commands::yay)=>{
            println!("yay~~");
        }
        Some(Commands::set)=>{
            println!("archlinux config setting");
            setmenu();
        }
        None=>{println!("command list.. {}",Stylize::bold("--help").red())}
    }
}
fn setmenu (){
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