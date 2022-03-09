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

    let unparsed_file = fs::read_to_string(&path).expect("cannot read file");
    match LouisParser::parse(Rule::table, &unparsed_file) {
	Ok(mut pairs) => {
	    let table = pairs.next().unwrap();
	    for rule in table.into_inner() {
		match rule.as_rule() {
		    Rule::comment => { print!("{}", rule.as_str()) },
		    Rule::empty_line => { print!("{}", rule.as_str()) },

		    Rule::rule_with_comment => {
			let mut inner_rules = rule.into_inner();
			let rule = inner_rules.next().unwrap().as_str();
			let end_comment = inner_rules.next();
			match end_comment {
			    Some(pair) => {
				let comment = pair.as_str().trim().trim_start_matches('#').trim_start();
				if comment.is_empty() {
				    println!("{}", rule);
				} else {
				    println!("{}\t# {}", rule, comment);
				}
			    },
			    None => {
				println!("{}", rule);
			    }
			}
		    }
		    // // Miscellaneous Opcodes
		    // Rule::include => (),
		    // Rule::undefined => (),
		    // Rule::display => (),
		    // Rule::multind => (),
		    // // Character-Definition Opcodes
		    // Rule::space => (),
		    // Rule::punctuation => (),
		    // Rule::digit => (),
		    // Rule::grouping => (),
		    // Rule::letter => (),
		    // Rule::lowercase => (),
		    // Rule::uppercase => (),
		    // Rule::litdigit => (),
		    // Rule::sign => {
		    //		// let mut inner_rules = rule.into_inner();
		    //		// let chars = inner_rules.next().unwrap().as_str();
		    //		// let dots = inner_rules.next().unwrap().as_str();
		    //		// println!("\tsign\t{}\t{}", chars, dots);
		    // },
		    // Rule::math => (),
		    // // Braille Indicator Opcodes
		    // Rule::capsletter => (),
		    // Rule::begcapsword => (),
		    // Rule::endcapsword => (),
		    // Rule::capsmodechars => (),
		    // Rule::begcaps => (),
		    // Rule::endcaps => (),
		    // Rule::begcapsphrase => (),
		    // Rule::endcapsphrase => (),
		    // Rule::lencapsphrase => (),
		    // Rule::letsign => (),
		    // Rule::noletsign => (),
		    // Rule::noletsignbefore => (),
		    // Rule::noletsignafter => (),
		    // Rule::nocontractsign => (),
		    // Rule::numsign => (),
		    // Rule::numericnocontchars => (),
		    // Rule::numericmodechars => (),
		    // Rule::midendnumericmodechars => (),
		    // // Opcodes for Standing Alone Sequences
		    // // Emphasis Opcodes
		    // Rule::emphclass => (),
		    // Rule::begemph => (),
		    // Rule::endemph => (),
		    // Rule::noemphchars => (),
		    // Rule::emphletter => (),
		    // Rule::begemphword => (),
		    // Rule::endemphword => (),
		    // Rule::emphmodechars => (),
		    // Rule::begemphphrase => (),
		    // Rule::endemphphrase => (),
		    // Rule::lenemphphrase => (),
		    // // Computer braille
		    // // Special Symbol Opcodes
		    // Rule::decpoint => (),
		    // Rule::hyphen => (),
		    // // Special Processing Opcodes
		    // Rule::capsnocont => (),

		    // // Translation Opcodes
		    // Rule::compbrl => (),
		    // Rule::comp6 => (),
		    // Rule::nocont => (),
		    // Rule::replace => (),
		    // Rule::always => (),
		    // Rule::repeated => (),
		    // Rule::repword => (),
		    // Rule::rependword => (),
		    // Rule::largesign => (),
		    // Rule::word => (),
		    // Rule::syllable => (),
		    // Rule::joinword => (),
		    // Rule::lowword => (),
		    // Rule::contraction => (),
		    // Rule::sufword => (),
		    // Rule::prfword => (),
		    // Rule::begword => (),
		    // Rule::begmidword => (),
		    // Rule::midword => (),
		    // Rule::midendword => (),
		    // Rule::endword => (),
		    // Rule::partword => (),
		    // Rule::prepunc => (),
		    // Rule::postpunc => (),
		    // Rule::begnum => (),
		    // Rule::midnum => (),
		    // Rule::endnum => (),
		    // Rule::joinnum => (),

		    // // Computer braille
		    // Rule::begcomp => (),
		    // Rule::endcomp => (),

		    // // Character-Class Opcodes
		    // Rule::attribute => (),
		    // // Swap Opcodes
		    // Rule::swapcd => (),
		    // Rule::swapdd => (),
		    // Rule::swapcc => (),
		    // // Context and Multipass Opcodes
		    // Rule::context => (),
		    // Rule::pass2 => (),
		    // Rule::pass3 => (),
		    // Rule::pass4 => (),
		    // // The correct Opcode
		    // Rule::correct => (),
		    // // The match Opcode
		    // Rule::match_rule => (),
		    // Rule::pre_pattern => (),
		    // Rule::post_pattern => (),

		    Rule::EOI => (),
		    _ => {
			println!("{}", rule.as_str());
			unreachable!()
		    },
		}
	    }
	}
	Err(error) => {
	    println!("{:?}", error.with_path(&path));
	}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_ok, assert_err};

    #[test]
    fn dot() {
	assert_ok!(LouisParser::parse(Rule::dot, "1"));
    }

    #[test]
    fn virtual_dot() {
	assert_ok!(LouisParser::parse(Rule::dot, "a"));
    }

    #[test]
    fn dot_faulty() {
	assert_err!(LouisParser::parse(Rule::dot, "z"));
    }

    #[test]
    fn dot_zero() {
	assert_ok!(LouisParser::parse(Rule::dot, "0"));
    }

    #[test]
    fn dots() {
	assert_ok!(LouisParser::parse(Rule::dots, "56-45-245"));
    }

    #[test]
    fn dots_with_chars() {
	assert_err!(LouisParser::parse(Rule::dots, "56-45-245zzzzz"), "Dots should not contain any characters other that 1-9 and a-f");
    }

    #[test]
    fn dots_with_chars_2() {
	assert_err!(LouisParser::parse(Rule::dots, "56-45-zzzzz-245"), "Dots should not contain any characters other that 1-9 and a-f");
    }

    #[test]
    fn dots_with_chars_3() {
	assert_err!(LouisParser::parse(Rule::dots, "zzz-56-45-245"), "Dots should not contain any characters other that 1-9 and a-f");
    }

    #[test]
    fn dots_with_doubledash() {
	assert_err!(LouisParser::parse(Rule::dots, "56-45--245"), "Dots should not contain double dashes");
    }

    #[test]
    fn dots_starting_with_dash() {
	assert_err!(LouisParser::parse(Rule::dots, "-56-45-245"), "Dots shouldn't start with a dash");
    }

    #[test]
    fn hex_char() {
	assert_ok!(LouisParser::parse(Rule::hex_char, r"\x00b0"));
    }

    #[test]
    fn hex_char_faulty() {
	assert_err!(LouisParser::parse(Rule::hex_char, r"\x00z0"));
    }

    #[test]
    fn character() {
	assert_ok!(LouisParser::parse(Rule::char, r"\x00b0"));
    }

    #[test]
    fn sign() {
	assert_ok!(LouisParser::parse(Rule::sign, "sign a 56-45-245"));
    }

    #[test]
    fn sign_with_comment() {
	let parse_result = LouisParser::parse(Rule::rule_with_comment, "sign a 56-45-245 °")
	    .unwrap().next().unwrap();

	assert_eq!(parse_result.as_rule(), Rule::sign);

	let mut inner_rules = parse_result.into_inner();

	let chars = inner_rules.next().unwrap();
	let dots = inner_rules.next().unwrap();

	assert_eq!(chars.as_str(), "a");
	assert_eq!(dots.as_str(), "56-45-245");
    }

    #[test]
    fn sign_with_escaped_unicode() {
	assert_ok!(LouisParser::parse(Rule::sign, r"sign \x00b0 56-45-245"));
    }

    #[test]
    fn punctuation() {
	assert_ok!(LouisParser::parse(Rule::punctuation, r"punctuation \x00a0 0		NO-BREAK SPACE"));
    }
}
