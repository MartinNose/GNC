extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate inkwell;

use clap::{App, Arg};
use std::path::Path;
use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use inkwell::context::Context;

mod parser;
mod codegen;
mod checker;

use codegen::CodeGen;

fn main() {
    let mut app = App::new("gnalcc")
        .version("0.1.0")
        .author("iamNCJ ~ MartinNose ~ Ziyue")
        .about("gnalc is going to fuck c-lang")
        .arg(Arg::with_name("version").short("v").long("version").help("Show version of gnalcc"))
        .arg(Arg::with_name("FILE").short("c").help("File ready to be compiled").index(1));

    let app_gnalcc = app.clone().get_matches();

    if let Some(file_path) = app_gnalcc.value_of("FILE") {
        
        let ast = parser::parse(file_path);

        checker::check(&ast);

        let context = Context::create();
        let code_gen = CodeGen::new(&context, file_path);
        code_gen.gen(&ast);
    } else {
        app.print_help().unwrap();
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_basic_parse() {
        let file_path = "./test/basic.c";

        let ast = parser::parse(file_path);

        checker::check(&ast);

        let context = Context::create();
        let code_gen = CodeGen::new(&context, file_path);
        code_gen.gen(&ast);
    }
}