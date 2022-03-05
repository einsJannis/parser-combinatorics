#![feature(proc_macro_quote)]

extern crate proc_macro;

use proc_macro::*;

enum ParseError {
    UnexpectedToken(TokenTree),
    NoTokensLeft
}

enum Child {
    Ident(Ident),
    Literal(Literal)
}

struct ConcatinativeRuleChild {
    child: Child,
    imediate_next: bool,
    repeated: bool,
}

enum GrammarRuleContent {
    Options(Vec<Child>),
    Concatinative(Vec<ConcatinativeRuleChild>),
}

struct GrammarRule {
    name: Ident,
    content: GrammarRuleContent
}

#[proc_macro]
pub fn grammar_rules(input: TokenStream) -> TokenStream {
    let grammar_rules = parse_grammar_rules(&mut input.into_iter());
    todo!()
}

fn parse_grammar_rules(input: &mut token_stream::IntoIter) -> Result<Vec<GrammarRule>, ParseError> {
    let mut res = vec![];
    loop {
        let next = parse_grammar_rule(input);
        match next {
            Ok(it) => res.push(it),
            Err(ParseError::NoTokensLeft) => return Ok(res),
            Err(it) => return Err(it)
        }
    }
}

fn parse_grammar_rule(input: &mut token_stream::IntoIter) -> Result<GrammarRule, ParseError> {
    let name = parse_name(input)?;
    parse_arrow(input)?;
    let content = parse_rule_content(input)?;
    Ok(GrammarRule { name, content })
}

fn parse_name(input: &mut token_stream::IntoIter) -> Result<Ident, ParseError> {
    match input.next().ok_or(ParseError::NoTokensLeft)? {
        TokenTree::Ident(it) => Ok(it),
        it => Err(ParseError::UnexpectedToken(it))
    }
}

fn parse_arrow(input: &mut token_stream::IntoIter) -> Result<(), ParseError> {
    parse_symbol(input, '-', true)?;
    parse_symbol(input, '>', false)?;
    Ok(())
}

fn parse_rule_content(input: &mut token_stream::IntoIter) -> Result<GrammarRuleContent, ParseError> {
    let first_child = parse_child(input)?;
    match input.next().ok_or(ParseError::NoTokensLeft)? {
        TokenTree::Punct(it) => if it.as_char() == '|' {
            let mut vec = vec![first_child];
            parse_options(input, &mut vec)?;
            Ok(GrammarRuleContent::Options(vec))
        } else if it.as_char() == ';' {
            Ok(GrammarRuleContent::Options(vec![first_child]))
        } else if it.as_char() == '.' {
            match input.next().ok_or(ParseError::NoTokensLeft)? {
                TokenTree::Punct(it) => if it.as_char() == ';' {
                    Ok(GrammarRuleContent::Concatinative(vec![ConcatinativeRuleChild { child: first_child, imediate_next: true, repeated: false }]))
                } else if it.as_char() == '*' {
                    match input.next().ok_or(ParseError::NoTokensLeft)? {
                        TokenTree::Punct(it) => if it.as_char() == ';' {
                            Ok(GrammarRuleContent::Concatinative(vec![ConcatinativeRuleChild { child: first_child, imediate_next: true, repeated: true }]))
                        } else { Err(ParseError::UnexpectedToken(TokenTree::Punct(it))) }
                        TokenTree::Ident(it) => {
                            let mut vec = vec![ConcatinativeRuleChild { child: first_child, imediate_next: true, repeated: true }];
                            parse_concatinative(input, &mut vec, Child::Ident(it))?;
                            Ok(GrammarRuleContent::Concatinative(vec))
                        }
                        TokenTree::Literal(it) => {
                            let mut vec = vec![ConcatinativeRuleChild { child: first_child, imediate_next: true, repeated: true }];
                            parse_concatinative(input, &mut vec, Child::Literal(it))?;
                            Ok(GrammarRuleContent::Concatinative(vec))
                        }
                        it => Err(ParseError::UnexpectedToken(it))
                    }
                } else { Err(ParseError::UnexpectedToken(TokenTree::Punct(it))) }
                TokenTree::Ident(it) => {
                    let mut vec = vec![ConcatinativeRuleChild { child: first_child, imediate_next: true, repeated: false }];
                    parse_concatinative(input, &mut vec, Child::Ident(it))?;
                    Ok(GrammarRuleContent::Concatinative(vec))
                }
                TokenTree::Literal(it) => {
                    let mut vec = vec![ConcatinativeRuleChild { child: first_child, imediate_next: true, repeated: false }];
                    parse_concatinative(input, &mut vec, Child::Literal(it))?;
                    Ok(GrammarRuleContent::Concatinative(vec))
                }
                it => Err(ParseError::UnexpectedToken(it))
            }
        } else if it.as_char() == '*' {
            match input.next().ok_or(ParseError::NoTokensLeft)? {
                TokenTree::Punct(it) => if it.as_char() == ';' {
                    Ok(GrammarRuleContent::Concatinative(vec![ConcatinativeRuleChild { child: first_child, imediate_next: true, repeated: false }]))
                } else if it.as_char() == '.' {
                    match input.next().ok_or(ParseError::NoTokensLeft)? {
                        TokenTree::Punct(it) => if it.as_char() == ';' {
                            Ok(GrammarRuleContent::Concatinative(vec![ConcatinativeRuleChild { child: first_child, imediate_next: true, repeated: true }]))
                        } else { Err(ParseError::UnexpectedToken(TokenTree::Punct(it))) }
                        TokenTree::Ident(it) => {
                            let mut vec = vec![ConcatinativeRuleChild { child: first_child, imediate_next: true, repeated: true }];
                            parse_concatinative(input, &mut vec, Child::Ident(it))?;
                            Ok(GrammarRuleContent::Concatinative(vec))
                        }
                        TokenTree::Literal(it) => {
                            let mut vec = vec![ConcatinativeRuleChild { child: first_child, imediate_next: true, repeated: true }];
                            parse_concatinative(input, &mut vec, Child::Literal(it))?;
                            Ok(GrammarRuleContent::Concatinative(vec))
                        }
                        it => Err(ParseError::UnexpectedToken(it))
                    }
                } else { Err(ParseError::UnexpectedToken(TokenTree::Punct(it))) }
                TokenTree::Ident(it) => {
                    let mut vec = vec![ConcatinativeRuleChild { child: first_child, imediate_next: false, repeated: true }];
                    parse_concatinative(input, &mut vec, Child::Ident(it))?;
                    Ok(GrammarRuleContent::Concatinative(vec))
                }
                TokenTree::Literal(it) => {
                    let mut vec = vec![ConcatinativeRuleChild { child: first_child, imediate_next: false, repeated: true }];
                    parse_concatinative(input, &mut vec, Child::Literal(it))?;
                    Ok(GrammarRuleContent::Concatinative(vec))
                }
                it => Err(ParseError::UnexpectedToken(it))
            }
        } else {
            Err(ParseError::UnexpectedToken(TokenTree::Punct(it)))
        }
        TokenTree::Ident(it) => {
            let mut vec = vec![ConcatinativeRuleChild { child: first_child, imediate_next: false, repeated: false }];
            parse_concatinative(input, &mut vec, Child::Ident(it))?;
            Ok(GrammarRuleContent::Concatinative(vec))
        }
        TokenTree::Literal(it) => {
            let mut vec = vec![ConcatinativeRuleChild { child: first_child, imediate_next: false, repeated: false }];
            parse_concatinative(input, &mut vec, Child::Literal(it))?;
            Ok(GrammarRuleContent::Concatinative(vec))
        }
        it => Err(ParseError::UnexpectedToken(it))
    }
}

fn parse_concatinative(input: &mut token_stream::IntoIter, vec: &mut Vec<ConcatinativeRuleChild>, next: Child) -> Result<(), ParseError> {
    match input.next().ok_or(ParseError::NoTokensLeft)? {
        TokenTree::Punct(it) => if it.as_char() == ';' {
            vec.push(ConcatinativeRuleChild { child: next, imediate_next: false, repeated: false });
            Ok(())
        } else if it.as_char() == '.' { 
            match input.next().ok_or(ParseError::NoTokensLeft)? {
                TokenTree::Punct(it) => if it.as_char() == ';' {
                    vec.push(ConcatinativeRuleChild { child: next, imediate_next: true, repeated: false });
                    Ok(())
                } else if it.as_char() == '*' {
                    match input.next().ok_or(ParseError::NoTokensLeft)? {
                        TokenTree::Punct(it) => if it.as_char() == ';' {
                            vec.push(ConcatinativeRuleChild { child: next, imediate_next: true, repeated: true });
                            Ok(())
                        } else { Err(ParseError::UnexpectedToken(TokenTree::Punct(it))) }
                        TokenTree::Ident(it) => {
                            vec.push(ConcatinativeRuleChild { child: next, imediate_next: true, repeated: true });
                            parse_concatinative(input, vec, Child::Ident(it))?;
                            Ok(())
                        }
                        TokenTree::Literal(it) => {
                            vec.push(ConcatinativeRuleChild { child: next, imediate_next: true, repeated: true });
                            parse_concatinative(input, vec, Child::Literal(it))?;
                            Ok(())
                        }
                        it => Err(ParseError::UnexpectedToken(it))
                    }
                } else { Err(ParseError::UnexpectedToken(TokenTree::Punct(it))) }
                TokenTree::Ident(it) => {
                    vec.push(ConcatinativeRuleChild { child: next, imediate_next: true, repeated: false });
                    parse_concatinative(input, vec, Child::Ident(it))?;
                    Ok(())
                }
                TokenTree::Literal(it) => {
                    vec.push(ConcatinativeRuleChild { child: next, imediate_next: true, repeated: false });
                    parse_concatinative(input, vec, Child::Literal(it))?;
                    Ok(())
                }
                it => Err(ParseError::UnexpectedToken(it))
            }
        } else if it.as_char() == '*' {
            match input.next().ok_or(ParseError::NoTokensLeft)? {
                TokenTree::Punct(it) => if it.as_char() == ';' {
                    vec.push(ConcatinativeRuleChild { child: next, imediate_next: false, repeated: true });
                    Ok(())
                } else if it.as_char() == '.' {
                    match input.next().ok_or(ParseError::NoTokensLeft)? {
                        TokenTree::Punct(it) => if it.as_char() == ';' {
                            vec.push(ConcatinativeRuleChild { child: next, imediate_next: true, repeated: true });
                            Ok(())
                        } else { Err(ParseError::UnexpectedToken(TokenTree::Punct(it))) }
                        TokenTree::Ident(it) => {
                            vec.push(ConcatinativeRuleChild { child: next, imediate_next: true, repeated: true });
                            parse_concatinative(input, vec, Child::Ident(it))?;
                            Ok(())
                        }
                        TokenTree::Literal(it) => {
                            vec.push(ConcatinativeRuleChild { child: next, imediate_next: true, repeated: true });
                            parse_concatinative(input, vec, Child::Literal(it))?;
                            Ok(())
                        }
                        it => Err(ParseError::UnexpectedToken(it))
                    }
                } else { Err(ParseError::UnexpectedToken(TokenTree::Punct(it))) }
                TokenTree::Ident(it) => {
                    vec.push(ConcatinativeRuleChild { child: next, imediate_next: false, repeated: true });
                    parse_concatinative(input, vec, Child::Ident(it))?;
                    Ok(())
                }
                TokenTree::Literal(it) => {
                    vec.push(ConcatinativeRuleChild { child: next, imediate_next: false, repeated: true });
                    parse_concatinative(input, vec, Child::Literal(it))?;
                    Ok(())
                }
                it => Err(ParseError::UnexpectedToken(it))
            }
        } else { Err(ParseError::UnexpectedToken(TokenTree::Punct(it))) }
        TokenTree::Ident(it) => {
            vec.push(ConcatinativeRuleChild { child: next, imediate_next: false, repeated: false });
            parse_concatinative(input, vec, Child::Ident(it))?;
            Ok(())
        }
        TokenTree::Literal(it) => {
            vec.push(ConcatinativeRuleChild { child: next, imediate_next: false, repeated: false });
            parse_concatinative(input, vec, Child::Literal(it))?;
            Ok(())
        }
        it => Err(ParseError::UnexpectedToken(it))
    }
}

fn parse_options(input: &mut token_stream::IntoIter, vec: &mut Vec<Child>) -> Result<(), ParseError> {
    vec.push(parse_child(input)?);
    match input.next().ok_or(ParseError::NoTokensLeft)? {
        TokenTree::Punct(it) => if it.as_char() == ';' {
            Ok(())
        } else if it.as_char() == '|' {
            parse_options(input, vec)
        } else { Err(ParseError::UnexpectedToken(TokenTree::Punct(it))) }
        it => Err(ParseError::UnexpectedToken(it))
    }
}

fn parse_child(input: &mut token_stream::IntoIter) -> Result<Child, ParseError> {
    match input.next().ok_or(ParseError::NoTokensLeft)? {
        TokenTree::Ident(it) => Ok(Child::Ident(it)),
        TokenTree::Literal(it) => Ok(Child::Literal(it)),
        it => Err(ParseError::UnexpectedToken(it))
    }
}

fn parse_symbol(input: &mut token_stream::IntoIter, c: char, imediate: bool) -> Result<(), ParseError> {
    match input.next().ok_or(ParseError::NoTokensLeft)? {
        TokenTree::Punct(it) => if it.as_char() == c && (it.spacing() == Spacing::Joint) == imediate { Ok(()) } else { Err(ParseError::UnexpectedToken(TokenTree::Punct(it))) },
        it => Err(ParseError::UnexpectedToken(it))
    }
}

