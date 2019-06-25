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

    let pairs = LOUISParser::parse(Rule::table, &buffer).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            println!("  Rule:    {:?}", inner_pair.as_rule());
            println!("  Span:    {:?}", inner_pair.as_span());
            println!("  Text:    {}", inner_pair.as_str());
            // match inner_pair.as_rule() {
            //     Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
            //     Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
            //     _ => unreachable!()
            // };
        }
    }

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

    // let unsuccessful_parse = LOUISParser::parse(Rule::table, &buffer);
    // println!("{:?}", unsuccessful_parse);

    Ok(())
}
