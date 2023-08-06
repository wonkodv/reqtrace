use std::{
    fs,
    io::{self},
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    common::{Location, Reference, Requirement},
    errors::{self, Error},
    util::glob_paths,
};

use log::warn;
use proc_macro2::Span;
use syn::{
    parse_file,
    visit::{self, Visit},
    ItemFn,
};

use quote::ToTokens;

use super::ArtefactConfig;

#[derive(Debug)]
pub struct RustParser {
    paths: Vec<PathBuf>,
}

impl RustParser {
    pub fn from_config(config: ArtefactConfig) -> Result<Self, Error> {
        assert!(config.parser == "rust");
        let paths = glob_paths(&config.paths)?;
        Ok(Self { paths })
    }
}

impl super::Parser for RustParser {
    fn parse(&mut self) -> (Vec<Rc<Requirement>>, Vec<Error>) {
        let mut requirements = Vec::new();
        let mut errors = Vec::new();
        for path in &self.paths {
            let file = fs::File::open(path).map_err(|e| Error::Io(path.into(), e));
            match file {
                Err(err) => {
                    warn!("{}", err);
                    return (vec![], vec![err]);
                }
                Ok(file) => {
                    let mut r = io::BufReader::new(file);
                    let (r, e) = parse(&mut r, path);
                    requirements.extend(r);
                    errors.extend(e);
                }
            }
        }
        (requirements, errors)
    }
}

pub fn parse(
    reader: &mut impl io::BufRead,
    path: &Path,
) -> (Vec<Rc<Requirement>>, Vec<errors::Error>) {
    let requirements = Vec::new();
    let mut errors = Vec::new();

    let mut source = String::new();
    match reader.read_to_string(&mut source) {
        Err(e) => {
            errors.push(errors::Error::Io(path.into(), e));
        }
        Ok(_bytes) => match parse_file(&source) {
            Err(e) => {
                let pos = e.span().start();
                errors.push(errors::Error::Format(
                    Location::new_with_line_and_column(path.into(), pos.line, pos.column),
                    e.to_string(),
                ));
            }
            Ok(file) => {
                //    println!("{file:#?}");
                let mut p = Parser {
                    source,
                    requirements,
                    errors,
                    path,
                    symbol_stack: Vec::new(),
                };
                p.visit_file(&file);
                return (p.requirements, p.errors);
            }
        },
    }

    (requirements, errors)
}

struct Parser<'a> {
    requirements: Vec<Rc<Requirement>>,
    errors: Vec<errors::Error>,
    symbol_stack: Vec<String>,
    source: String,
    path: &'a Path,
}

impl Parser<'_> {
    fn parse_macro(&mut self, node: &syn::ExprMacro) -> bool {
        cov_mark::hit!(FMT_RUST_COV);

        let seg = &node.mac.path.segments;
        if seg.len() != 2 {
            return false;
        }
        if seg[0].ident != "cov_mark" {
            return false;
        }
        if seg[1].ident != "hit" {
            return false;
        }

        let location = location_from_span(self.path, &seg[0].ident.span());

        let tokens: Vec<_> = node.mac.tokens.clone().into_iter().collect();
        if tokens.is_empty() {
            self.errors.push(errors::Error::Format(
                location,
                "cov_mark::hit!() without Requirement Id".to_string(),
            ));
        } else {
            let mut title = None;
            match &node.mac.delimiter {
                syn::MacroDelimiter::Paren(syn::token::Paren { span }) => {
                    // TODO: parse reference title from comment
                    // TODO: title = code[span.start().bytes.....];
                    title = None;
                    let _ = span;
                }
                _ => {
                    self.errors.push(errors::Error::Format(
                        location.clone(),
                        "cov_mark::hit! not using Parentheses".to_string(),
                    ));
                }
            }

            let covers = tokens[0].to_string();

            let reference = Reference {
                id: covers,
                title,
                location: None,
            };

            if tokens.len() > 1 {
                self.errors.push(errors::Error::Format(
                    location.clone(),
                    "cov_mark::hit!() more than 1 argument/token".to_string(),
                ));
            }

            // TODO: add info from path of source file to make symbols unique
            let id = self.symbol_stack.join("::");

            match self.requirements.last_mut() {
                Some(last) if last.id == id => {
                    Rc::get_mut(last).unwrap().covers.push(reference);
                }
                _ => {
                    let covers = vec![reference];
                    let req = Requirement {
                        id,
                        location,
                        covers,
                        ..Requirement::default()
                    };
                    let req = Rc::new(req);
                    self.requirements.push(req);
                }
            }
        }
        true
    }
}

impl<'ast> Visit<'ast> for Parser<'ast> {
    fn visit_expr_macro(&mut self, node: &'ast syn::ExprMacro) {
        if !self.parse_macro(node) {
            visit::visit_expr_macro(self, node);
        }
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.symbol_stack.push(node.sig.ident.to_string());
        visit::visit_item_fn(self, node);
        self.symbol_stack.pop().unwrap();
    }

    fn visit_impl_item_method(&mut self, node: &'ast syn::ImplItemMethod) {
        self.symbol_stack.push(node.sig.ident.to_string());
        visit::visit_impl_item_method(self, node);
        self.symbol_stack.pop().unwrap();
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        let type_ = node.self_ty.to_token_stream().to_string();
        let symbol;
        if let Some((bang, trait_, _for_tok)) = &node.trait_ {
            let trait_ = trait_.to_token_stream().to_string();

            if bang.is_some() {
                symbol = format!("({type_} as !{trait_})");
            } else {
                symbol = format!("({type_} as {trait_})");
            }
        } else {
            symbol = type_;
        }
        self.symbol_stack.push(symbol);
        visit::visit_item_impl(self, node);
        self.symbol_stack.pop().unwrap();
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        self.symbol_stack.push(node.ident.to_string());
        visit::visit_item_trait(self, node);
        self.symbol_stack.pop().unwrap();
    }

    fn visit_trait_item_method(&mut self, node: &'ast syn::TraitItemMethod) {
        self.symbol_stack.push(node.sig.ident.to_string());
        visit::visit_trait_item_method(self, node);
        self.symbol_stack.pop().unwrap();
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        self.symbol_stack.push(node.ident.to_string());
        visit::visit_item_mod(self, node);
        self.symbol_stack.pop().unwrap();
    }
}

fn location_from_span(path: &Path, span: &Span) -> Location {
    let start = span.start();
    Location::new_with_line_and_column(path.to_path_buf(), start.line, start.column)
}

#[cfg(test)]
mod test {
    use std::{io::BufReader, path::PathBuf};

    use crate::common::LocationInFile;

    use super::*;

    #[test]
    fn test_simple() {
        let s = "
            mod module_name {
                impl Trait for Struct {
                    fn foo() {
                        cov_mark::hit!(REQ_ID /* Requirement Title   */);
                    }
                }
            }
        "
        .to_string();

        let (reqs, errors) = parse(
            &mut BufReader::new(s.as_bytes()),
            &PathBuf::from("src/filename.rs"),
        );
        assert!(errors.is_empty());
        assert_eq!(reqs.len(), 1);
        let req = &reqs[0];
        assert_eq!(req.id, "module_name::(Struct as Trait)::foo");
        assert_eq!(req.covers.len(), 1);
        assert_eq!(req.covers[0].id, "REQ_ID");
        assert_eq!(req.location.file, PathBuf::from("src/filename.rs"));
        assert_eq!(
            req.location.location_in_file,
            Some(LocationInFile::LineAndColumn(5, 24))
        );
        // TODO: assert_eq!(req.covers[0].title, Some("Requirement Title".into()));
    }
}
