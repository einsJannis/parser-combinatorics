#![feature(proc_macro_quote)]

extern crate proc_macro;

use std::str::FromStr;

use proc_macro::*;

#[derive(Debug)]
enum ParseError {
    NoTokensLeft(),
    UnexpectedToken()
}

type ChildrenOption = Vec<Child>;

struct Child {
    value: ChildValue,
    repeatable: bool,
    next_imediat: bool
}

enum ChildValue {
    Ident(Ident),
    Literal(Literal)
}

struct GrammarRule {
    parent: Ident,
    children_options: Vec<ChildrenOption>,
}

#[proc_macro]
pub fn grammar_rules(input: TokenStream) -> TokenStream {
    let grammar_rules = match parse_grammar_rules(&mut input.into_iter()) {
        Err(it) => return quote! { compiler_error!(fmt!("Grammar rule parsing error: {}", it)) },
        Ok(it) => it
    };
    todo!()
}

/*
fn generate_enum(grammar_rules: Vec<GrammarRule>) -> TokenStream {
    let name = grammar_rules.first().unwrap().parent;
    let children_options: Vec<ChildrenOption> = grammar_rules.iter().map(|it| it.children_options).flatten().collect();
    let generated_children = children_options.iter().map(generate_enum_children);
    quote! {
        enum #name {
            #(#token_stream),*
        }
    }
}

fn generate_enum_children(children_option: &ChildrenOption) -> TokenStream {
    let mut name_string = String::from("r#");
    for (i, child) in children_option.iter().enumerate() {
        match child.value {
            ChildValue::Ident(it) => name_string.push_str(it.to_string().as_str()),
            ChildValue::Literal(it) => name_string.push_str(format!("literal_{}", it.to_string()).as_str())
        }
        if i < (children_option.len()-1) {
            name_string.push('_');
        }
    }
    let name = TokenStream::from_str(name_string.as_str()).unwrap();
    let children = children_option.iter().map(|it| match it.value {
        ChildValue::Ident(it) => TokenTree::Ident(it),
        ChildValue::Literal(it) => TokenTree::Literal(it)
    });
    quote! {
        #name(#(#token),*)
    }
}
*/

fn parse_grammar_rules(input: &mut token_stream::IntoIter) -> Result<Vec<GrammarRule>, ParseError> {
    let mut grammar_rules = Vec::new();
    while let Some(grammar_rule) = parse_grammar_rule(input)? {
        grammar_rules.push(grammar_rule);
    }
    Ok(grammar_rules)
}

macro_rules! next_or_ret_none {
    ($input:expr) => {
        match $input.next() {
            None => return Ok(None),
            Some(it) => it
        }
    };
}

macro_rules! next_or_ret_error { 
    ($input:expr) => {
        match $input.next() {
            None => return Err(ParseError::NoTokensLeft()),
            Some(it) => it
        }
    }
}

fn parse_grammar_rule(input: &mut token_stream::IntoIter) -> Result<Option<GrammarRule>, ParseError> {
    match next_or_ret_none!(input) {
        TokenTree::Ident(parent) => {
            let children_options = parse_children_options(input)?;
            Ok(Some(GrammarRule { parent, children_options }))
        }
        _ => return Err(ParseError::UnexpectedToken())
    }
}

fn parse_children_options(input: &mut token_stream::IntoIter) -> Result<Vec<ChildrenOption>, ParseError> {
    parse_arrow(input)?;
    let mut children_options = Vec::new();
    recursive_parse_children_options(input, &mut children_options)?;
    Ok(children_options)
}

fn recursive_parse_children_options(input: &mut token_stream::IntoIter, children_options: &mut Vec<ChildrenOption>) -> Result<(), ParseError> {
    let mut children_option = Vec::new();
    let next = recursive_parse_children_option(input, &mut children_option)?;
    children_options.push(children_option);
    if next { return recursive_parse_children_options(input, children_options); } else { return Ok(()); }
}

fn recursive_parse_children_option(input: &mut token_stream::IntoIter, children_option: &mut ChildrenOption) -> Result<bool, ParseError> {
    children_option.push(parse_child(input)?);
    do_next(input, children_option)
}

fn do_next(input: &mut token_stream::IntoIter, children_option: &mut ChildrenOption) -> Result<bool, ParseError> {
    if let TokenTree::Punct(it) = next_or_ret_error!(input) {
        if it.as_char() == '.' { children_option.last_mut().unwrap().next_imediat = true; return do_next(input, children_option); }
        if it.as_char() == '*' { children_option.last_mut().unwrap().repeatable = true; return do_next(input, children_option); }
        if it.as_char() == '&' { return recursive_parse_children_option(input, children_option); }
        if it.as_char() == '|' { return Ok(true); }
        if it.as_char() == ';' { return Ok(false); }
    }
    return Err(ParseError::UnexpectedToken());
}

fn parse_child(input: &mut token_stream::IntoIter) -> Result<Child, ParseError> {
    let child_value = match next_or_ret_error!(input) {
        TokenTree::Ident(it) => ChildValue::Ident(it),
        TokenTree::Literal(it) => ChildValue::Literal(it),
        _ => return Err(ParseError::UnexpectedToken())
    };
    Ok(Child { value: child_value, next_imediat: false, repeatable: false })
}

fn parse_arrow(input: &mut token_stream::IntoIter) -> Result<(), ParseError> {
    match next_or_ret_error!(input) {
        TokenTree::Punct(it) => if it.as_char() == '-' || it.as_char() == '=' {
            match it.spacing() {
                Spacing::Joint => match next_or_ret_error!(input) {
                    TokenTree::Punct(it) => if it.as_char() == '>' {
                        return Ok(());
                    } else { Err(ParseError::UnexpectedToken()) },
                    _ => Err(ParseError::UnexpectedToken())
                },
                _ => Err(ParseError::UnexpectedToken())
            }
        } else { Err(ParseError::UnexpectedToken()) },
        _ => return Err(ParseError::UnexpectedToken())
    }
}

