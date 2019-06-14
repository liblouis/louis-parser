extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use pest::Parser;

#[derive(Parser)]
#[grammar = "louis.pest"]
pub struct LOUISParser;

fn main() -> io::Result<()> {
    let path = std::env::args().nth(1).expect("no path given");
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    
    // let successful_parse = LOUISParser::parse(Rule::dots, "123");
    // println!("{:?}", successful_parse);

    // let successful_parse = LOUISParser::parse(Rule::comment, "# word foo 123-12");
    // println!("{:?}", successful_parse);

    // let successful_parse = LOUISParser::parse(Rule::comment, "# a little test");
    // println!("{:?}", successful_parse);

   // let successful_parse = LOUISParser::parse(Rule::rule, "word foo 123-12");
    // println!("{:?}", successful_parse);

    // let successful_parse = LOUISParser::parse(Rule::rule, "noback word foo 123-12");
    // println!("{:?}", successful_parse);

    // let successful_parse = LOUISParser::parse(Rule::rule, "include foo.txt");
    // println!("{:?}", successful_parse);

    let unsuccessful_parse = LOUISParser::parse(Rule::table, &buffer);
    println!("{:?}", unsuccessful_parse);

    Ok(())
}
