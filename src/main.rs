extern crate git2;
extern crate args;
extern crate getopts;
extern crate subprocess;
use subprocess::Exec;
use getopts::{Options, Occur, HasArg};
use args::{Args, ArgsError};
use git2::Repository;

const IDE_LINUX: &'static str = "https://github.com/cdr/code-server/releases/download/2.1688-vsc1.39.2/code-server2.1688-vsc1.39.2-linux-x86_64.tar.gz";
const IDE_DARWIN: &'static str = "https://github.com/cdr/code-server/releases/download/2.1688-vsc1.39.2/code-server2.1688-vsc1.39.2-darwin-x86_64.zip";
const IDE_EXTRACTED_LINUX: &'static str = "code-server2.1688-vsc1.39.2-linux-x86_64";
const IDE_EXTRACTED_DARWIN: &'static str = "code-server2.1688-vsc1.39.2-darwin-x86_64";

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
    let bundle = match os {
        "linux" => "tar --gzip -xf",
        "osx" => "unzip",
        _ => "tar --gzip -xf",
    };
    let outdir = match os {
        "linux" => IDE_EXTRACTED_LINUX,
        "osx" => IDE_EXTRACTED_DARWIN,
        _ => IDE_EXTRACTED_LINUX,
    };
    let _ = Exec::shell(&format!("wget {} -O ./{}/code-server && cd ./{}/ && {} ./code-server && && rm ./code-server && pwd && cd {} && ./code-server --open --port 9091", vscode, out, out, bundle, outdir)).join();
}
fn clone(url : &str, path : &str) -> Result<git2::Repository, git2::Error> {
    //let url = "https://github.com/sam-aldis/RedTeam-Toolkit";
    //let path = "";
    return Repository::clone(&format!("https://github.com/{}",url), path);
}