use std::env;
use std::vec::Vec;
// use std::result::Result;
use substring::Substring;

#[derive(Debug)]
pub struct Opts<'lt> {
    pub err: &'lt str,
    pub exec_name: String,
    pub help: bool,
    // dependencies: Box<[&'lt str]>,
    pub dependencies: Vec<&'lt str>,
    pub lang: String,
    pub preset: &'lt str
}

static SUPPORTED_LANGUAGES: [&str; 1] = ["javascript"];

pub fn parse() -> Opts<'static> {
    
    // default values
    let mut args: Vec<String> = env::args().collect();
    let mut err = "none";
    let exec_name = args.remove(0);
    let help = false;
    let dependencies = Vec::new();
    // dependencies.push("none");
    let lang = "js".to_string();
    let preset = "none";

    let mut options = Opts{err, exec_name, help, dependencies, lang, preset};

   
    if args.len() < 1 {

    }
    // for i in &args 
    // println!(" ln: {}", args.len());
    if args.len() == 0 {
        options.help = true;
        return options;
    }
    for i in 0..args.len(){
        // println!("i: {}, ln: {}", i, args.len());
        if args[i] == "-h" || args[i] == "--help" {
            options.help = true;
        } else 
        if args[i] == "--language" || args[i] == "--lang" || args[i] == "-l" {

            //safety check
            if i == (args.len() - 1) {
                options.err = "Missing values! \nhelp: add the language you want to make the project for.";
            } else {
                
                let lang = &args[i +1];
                if lang.substring(0, 1) == "-" {
                    options.err = "Missing values! \nhelp: add the language you want to make the project for.";
                    break;
                }
                options.lang = (&lang).to_string();
            }
        }
    }
    // println!("{:?}", args);
    

    
    return options;
}