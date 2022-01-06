use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    // only rebuild if the word list changes
    println!("cargo:rerun-if-changed=word_list.txt");

    // Build the word list
    let word_list = std::fs::read_to_string("word_list.txt").unwrap();
    let mut set = phf_codegen::Set::new();
    for line in word_list.lines() {
        set.entry(line);
    }

    // Write the generated data structure
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(
        &mut file,
        "pub static _WORDS: phf::Set<&'static str> = \n{};\n",
        set.build()
    )
    .unwrap();
}
