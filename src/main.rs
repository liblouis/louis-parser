extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io;
use std::fs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "louis.pest"]
pub struct LouisParser;

fn main() -> io::Result<()> {
    let path = std::env::args().nth(1).expect("no path given");

    let unparsed_file = fs::read_to_string(path).expect("cannot read file");
    let file = LouisParser::parse(Rule::table, &unparsed_file)
	.expect("unsuccessful parse")
	.next().unwrap();

    for rule in file.into_inner() {
	match rule.as_rule() {
	    // Miscellaneous Opcodes
	    Rule::include => (),
	    Rule::undefined => (),
	    Rule::display => (),
	    Rule::multind => (),
	    // Character-Definition Opcodes
	    Rule::space => (),
	    Rule::punctuation => (),
	    Rule::digit => (),
	    Rule::grouping => (),
	    Rule::uplow => (),
	    Rule::letter => (),
	    Rule::lowercase => (),
	    Rule::uppercase => (),
	    Rule::sign => (),
	    Rule::math => (),
	    // Braille Indicator Opcodes
	    Rule::capsletter => (),
	    Rule::begcapsword => (),
	    Rule::endcapsword => (),
	    Rule::capsmodechars => (),
	    Rule::begcaps => (),
	    Rule::endcaps => (),
	    Rule::begcapsphrase => (),
	    Rule::endcapsphrase => (),
	    Rule::lencapsphrase => (),
	    Rule::letsign => (),
	    Rule::noletsign => (),
	    Rule::noletsignbefore => (),
	    Rule::noletsignafter => (),
	    Rule::nocontractsign => (),
	    Rule::numsign => (),
	    Rule::numericnocontchars => (),
	    Rule::numericmodechars => (),
	    Rule::midendnumericmodechars => (),
	    // Opcodes for Standing Alone Sequences
	    // Emphasis Opcodes
	    Rule::emphclass => (),
	    Rule::begemph => (),
	    Rule::endemph => (),
	    Rule::noemphchars => (),
	    Rule::emphletter => (),
	    Rule::begemphword => (),
	    Rule::endemphword => (),
	    Rule::emphmodechars => (),
	    Rule::begemphphrase => (),
	    Rule::endemphphrase => (),
	    Rule::lenemphphrase => (),
	    // Computer braille
	    // Special Symbol Opcodes
	    Rule::decpoint => (),
	    Rule::hyphen => (),
	    // Special Processing Opcodes
	    Rule::capsnocont => (),
	    // Translation Opcodes
	    Rule::compbrl => (),
	    Rule::comp6 => (),
	    Rule::nocont => (),
	    Rule::replace => (),
	    Rule::always => (),
	    Rule::repeated => (),
	    Rule::repword => (),
	    Rule::rependword => (),
	    Rule::largesign => (),
	    Rule::word => (),
	    Rule::syllable => (),
	    Rule::joinword => (),
	    Rule::lowword => (),
	    Rule::contraction => (),
	    Rule::sufword => (),
	    Rule::prfword => (),
	    Rule::begword => (),
	    Rule::begmidword => (),
	    Rule::midword => (),
	    Rule::midendword => (),
	    Rule::endword => (),
	    Rule::partword => (),
	    Rule::prepunc => (),
	    Rule::postpunc => (),
	    Rule::midnum => (),
	    Rule::endnum => (),
	    Rule::joinnum => (),
	    // Character-Class Opcodes
	    Rule::attribute => (),
	    // Swap Opcodes
	    Rule::swapcd => (),
	    Rule::swapdd => (),
	    Rule::swapcc => (),
	    // Context and Multipass Opcodes
	    Rule::context => (),
	    Rule::pass2 => (),
	    Rule::pass3 => (),
	    Rule::pass4 => (),
	    // The correct Opcode
	    Rule::correct => (),
	    // The match Opcode
	    Rule::match_rule => (),
	    Rule::pre_pattern => (),
	    Rule::post_pattern => (),

	    Rule::EOI => (),
	    Rule::unknown_rule => {
		println!("{:?}, {:?}", rule.as_rule(), rule.as_str());
	    }
	    _ => unreachable!(),
	}
    }

    Ok(())
}
