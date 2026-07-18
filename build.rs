use std::fs;
use std::path::Path;

const PROJECT_NAME:&str = env!("CARGO_PKG_NAME");
const PROJECT_AUTHOR:&str = "Joel Ortega";
const PROJECT_VERSION:&str = env!("CARGO_PKG_VERSION");
const PROJECT_URL:&str = "github/user/project.com";
const YEAR:&str = "2026";

fn generate_wordlist(out_dir:&std::ffi::OsStr){
    let destination= Path::new(&out_dir).join("wordlist_data.rs");
    let file_content = fs::read_to_string("assets/eff_large_wordlist.txt").expect("Wordlist file not found.");
    let mut words:Vec<&str> = Vec::new();
    for line in file_content.lines(){
        words.push(line.split_once('\t').expect("Error generating wordlist").1);
    }
    let mut wordlist_code:String = String::new();
    wordlist_code.push_str(&format!("pub static WORDS:[&str;{}] = [\n",words.len()));
    for word in &words{
        wordlist_code.push_str(&format!("    \"{word}\",\n"));
    }
    wordlist_code.push_str("];\n");
    fs::write(&destination,wordlist_code).expect("Failed to write generated wordlist code.");
}

fn generate_help_text(out_dir:&std::ffi::OsStr){
    let destination = Path::new(&out_dir).join("help.rs");
    let help_template = fs::read_to_string("assets/help_template.txt").expect("Help file not found.");
    let help_text = help_template
        .replace("{PROJECT_NAME}",PROJECT_NAME)
        .replace("{PROJECT_URL}",PROJECT_URL);
    fs::write(&destination,format!("pub const HELP:&str = r#\"{}\"#;",help_text)).expect("Failed to write generated help.");
}

fn generate_version_text(out_dir:&std::ffi::OsStr){
    let destination = Path::new(&out_dir).join("version.rs");
    let version_template = fs::read_to_string("assets/version_template.txt").expect("Version file not found.");
    let version_text = version_template
        .replace("{PROJECT_NAME}",PROJECT_NAME)
        .replace("{PROJECT_AUTHOR}",PROJECT_AUTHOR)
        .replace("{PROJECT_VERSION}",PROJECT_VERSION)
        .replace("{YEAR}",YEAR);
    fs::write(&destination,format!("pub const VERSION:&str = r#\"{}\"#;",version_text)).expect("Failed to write generated version.");
}
fn main(){
    let out_dir = std::env::var_os("OUT_DIR").expect("OUT_DIR must exist");
    generate_wordlist(&out_dir);
    generate_help_text(&out_dir);
    generate_version_text(&out_dir);
}
