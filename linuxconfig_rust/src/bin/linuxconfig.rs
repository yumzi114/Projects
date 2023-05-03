use std::env;
use std::path::{Path, PathBuf};
use crossterm::style::Stylize;
use dirs;
use std::fs::File;
use std::io::{BufReader, Read, BufRead,BufWriter, Write, Seek};
use std::process;
use regex::Regex;
use clap::{Parser,Subcommand};
use std::fs;
use std::fs::OpenOptions;
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
struct sysinfo {
    pacman:bool,
    system:bool,
    yay:bool,
}

impl sysinfo{
    fn new(pwd:&str)->sysinfo{
        let temp= pwd.to_string()+"/.config/pacman.conf";
        println!("{}",temp);
        sysinfo{
            yay: Path::new("/bin/yay").is_file(),
            pacman: Path::new("/etc/pacman.conf").is_file(),
            system: Path::new("/etc/systemd/system.conf").is_file(),
        }
    }
}
fn main() {
    let homedir = dirs::home_dir().expect("Not Found HOME directory");
    let root = Path::new(&homedir);
    assert!(env::set_current_dir(&root).is_ok());
    let temp = env::current_dir().unwrap();
    let pwd =  temp.to_str().unwrap();
    let args = Args::parse();
    let info =sysinfo::new(pwd);
    println!("{:?}",info.pacman);
    match &args.exes {
        Some(Commands::default)=>{
            println!("install default pacman package");
            default();
        }
        Some(Commands::yay)=>{
            println!("install yay package manager");
            yay(pwd);
        }
        Some(Commands::set)=>{
            println!("archlinux config setting");
            setmenu();
        }
        None=>{println!("command list.. {}",Stylize::bold("--help").red())}
    }
}
fn process_lines<T:BufRead +Sized>(reader:T, re:Regex){
    for line_ in reader.lines().enumerate() {
        let (num, line) = line_;
        let text = line.unwrap();
        match re.find(&text) {
            Some(_) =>println!("{},{}",text,num),
            None=>(),
        }
    }
}
fn editFile()-> std::io::Result<()>{
    assert!(env::set_current_dir("/home/yum/").is_ok());
    let file = File::open("pacman.conf").expect("Not Fount pacman.conf");
    let mut buf_reader = BufReader::new(file);
    // let mut buf_reader = BufWriter::new(file);
    let mut contents = String::new();
    let temp=Regex::new("#").unwrap();
    process_lines(buf_reader, temp);
    // buf_reader.read_to_string(&mut contents)?;
    // println!("{:?}",contents);
    contents.truncate(0);
    Ok(())
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
    ]);
    run(&menu);
    println!("Selected: {}", mut_menu(&menu).selected_item_name());
    editFile().unwrap();
    // println!("{}", mut_menu(&menu).);
}
fn default(){
    runas::Command::new("pacman")
    .arg("-Sy")
    .args(&PACMAN_LIST)
    .status()
    .unwrap();
}
fn yay(root:&str){
    assert!(env::set_current_dir(root).is_ok());
    let yaybin = Path::new("/bin/yay").is_file();
    let yay_path = Path::new("./yay-git");
    if !yay_path.is_dir(){
        std::process::Command::new("git")
        .arg("clone")
        .arg(&GIT_LIST[0])
        .status()
        // .spawn()
        .unwrap();
        // .expect("failed to execute process");
    }
    if !yaybin{
        assert!(env::set_current_dir(&yay_path).is_ok());
        std::process::Command::new("makepkg")
        .arg("-si")
        .arg("--noconfirm")
        .status()
        .unwrap();
    }
}
