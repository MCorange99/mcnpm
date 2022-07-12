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
    pub preset: &'lt str,
    pub exit:  bool
}



pub fn parse() -> Opts<'static> {
    let SUPPORTED_LANGUAGES: [&str; 1] = ["javascript"];
    // default values
    let mut args: Vec<String> = env::args().collect();
    let mut err = "none";
    let exec_name = args.remove(0);
    let help = false;
    let dependencies = Vec::new();
    // dependencies.push("none");
    let lang = "js".to_string();
    let preset = "none";
    let exit = false;

    let mut options = Opts{err, exec_name, help, dependencies, lang, preset, exit};

   
    if args.len() < 1 {

    }
    // for i in &args 
    println!("ln: {}", args.len());
    if args.len() == 0 {
        options.help = true;
        return options;
    }
    for i in 0..args.len(){
        println!("i: {}, ln: {}", i, args.len());
        if args[i] == "-h" || args[i] == "--help" {
            options.help = true;
        } else 
        if args[i] == "--language" || args[i] == "--lang" || args[i] == "-l" {

            //safety check
            if i == (args.len() - 1) {
                options.err = "Missing values! \nhelp: add the language you want to make the project for.\ntip: use option --supported_langs to get all suported languages";
            } else {
                
                let lang = &args[i +1];
                if lang.substring(0, 1) == "-" {
                    let langs = SUPPORTED_LANGUAGES.join(",\n").as_str();
                    options.err = "Missing or invalid values!\nhelp: add the language you want to make the project for.\ntip: use option --supported_langs to get all suported languages";
                    break;
                }
                if SUPPORTED_LANGUAGES.contains(&lang.as_str()) {
                   options.lang = (&lang).to_string(); 
                }
                
            }
        } else 
        if args[i] == "--supported_langs" || args[i] == "--supported_languages"{
            let mut langs = "".to_string();
            if SUPPORTED_LANGUAGES.len() == 1 {
                langs= format!("    {}\n    ", SUPPORTED_LANGUAGES[0]);
            } else {
                langs= format!("    {},\n    ", SUPPORTED_LANGUAGES[0]);
            }
            
            if SUPPORTED_LANGUAGES.len() > 1 {
               for i in 1..SUPPORTED_LANGUAGES.len() {
                if i != SUPPORTED_LANGUAGES.len(){
                    langs = format!("{}{},\n    ", langs, SUPPORTED_LANGUAGES[i]);
                }else
                {
                    langs = format!("{}{}.\n", langs, SUPPORTED_LANGUAGES[i]);
                    break;
                }
               } 
            }
            
            // SUPPORTED_LANGUAGES[0], (if SUPPORTED_LANGUAGES.len() > 1 { SUPPORTED_LANGUAGES[1..(SUPPORTED_LANGUAGES.len() - 1)].join(",\n    ") } else { " ".to_string() }), SUPPORTED_LANGUAGES[SUPPORTED_LANGUAGES.len() - 1]
            print!("Supported languages: \n");
            println!("{}", langs)
        }
    }
    // println!("{:?}", args);
    

    
    return options;
}