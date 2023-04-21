use std::env;
use dirs;
use std::path::Path;
use std::io;
use cmd_lib::*;
fn test (){
    run_fun!(
        sudo pacman -Ss rust;
    ).unwrap_or_else(|_| cmd_die!("deaded"));
    
}
fn main() {
    let homedir = dirs::home_dir().expect("Fail not found home");
    let root = Path::new(&homedir);
    assert!(env::set_current_dir(&root).is_ok());
    let temp = env::current_dir().expect("Fail working dir");
    let pwd =  temp.to_str().expect("Fail can't dir to string");
    loop {
        let mut ask = String::new();
        println!("choise work num(working dir:{}) ",pwd);
        println!("install (default[1] yay[2] wildan(Hyprland)[3] Nvidia[4] set menu[5] exit[0])");
        io::stdin().read_line(&mut ask)
            .expect("error");
        let ask=match ask.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        match ask{
            1=>println!("1"),
            2=>println!("2"),
            3=>println!("3"),
            4=>println!("4"),
            5=>println!("5"),
            0=>break,
            _=>println!("Not found menu"),
        }
    }
    
}
