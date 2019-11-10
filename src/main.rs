extern crate git2;
extern crate args;
extern crate getopts;
extern crate subprocess;
use subprocess::Exec;
use getopts::{Options, Occur, HasArg};
use args::{Args, ArgsError};
use git2::Repository;

const IDE_LINUX: &'static str = "vscode-server-linux";
const IDE_DARWIN: &'static str = "vscode-server-dosx";

fn main() {
    arg_parse();
}
fn arg_parse() {
    let mut args = Args::new("kwik", "quickly scaffold from a repo");
    args.flag("h", "help", "Show Help");
    args.option("o", "os", "OS to download and bootstrap IDE for <[linux]/osx>", "<os>", Occur::Req, Option::from(String::from("linux")));
    args.option("p", "project", "The Scaffold you wish to use i.e sam-aldis/RustPython", "<Scaffold>", Occur::Req, Option::from(String::from("sam-aldis/RustPython")));
    args.option("d", "dir", "Out Directory", "<Directory>", Occur::Req, Option::from(String::from("./")));
    args.parse_from_cli().unwrap();
    let help = args.value_of("help").unwrap();
    if help {
        print!("{}",args.full_usage());
        
    } else {
        let os = args.value_of::<String>("os").unwrap();
        let out = args.value_of::<String>("dir").unwrap();
        let repo = args.value_of::<String>("project").unwrap();
        let res = clone(&repo, &out);
        get_ide(&os, &out);    
        match res {
            Ok(_repo) => println!("Ok"),
            Err(_e) => println!("Error"),
        };
        
    }
}

fn get_ide(os : &str, out : &str) {
    let vscode = match os {
        "linux" => IDE_LINUX,
        "osx" => IDE_DARWIN,
        _ => IDE_LINUX,
    };
    let _ = Repository::clone("https://github.com/sam-aldis/IDEs", &format!("{}/ides", out));
    let _cmd = Exec::cmd(format!("{}/ides/{}",out,vscode)).args(&["--open","--port","9091"]).stream_stdout();
}
fn clone(url : &str, path : &str) -> Result<git2::Repository, git2::Error> {
    //let url = "https://github.com/sam-aldis/RedTeam-Toolkit";
    //let path = "";
    return Repository::clone(&format!("https://github.com/{}",url), path);
}