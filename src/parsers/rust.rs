use std::{
    fs,
    io::{self},
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    common::{Location, Reference, Requirement},
    errors::{self, Error},
    models::ArtefactConfig,
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
            let file = fs::File::open(path).map_err(|e| Error::io(path, &e));
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
            errors.push(errors::Error::io(path, &e));
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
                    scope: Vec::new(),
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
    scope: Vec<String>,
    source: String,
    path: &'a Path,
}

impl Parser<'_> {
    fn parse_macro(&mut self, node: &syn::ExprMacro) -> bool {
        requirement_covered!(FMT_RUST_COV);

        let seg = &node.mac.path.segments;
        if seg.len() != 1 {
            return false;
        }
        if seg[0].ident != "requirement_covered" {
            return false;
        }

        let tokens: Vec<_> = node.mac.tokens.clone().into_iter().collect();

        if let Some((referenced_id, title)) = match tokens.len() {
            0 => {
                self.errors.push(errors::Error::Format(
                    location_from_span(self.path, &seg[0].ident.span()),
                    "requirement_covered!() has no arguments".to_owned(),
                ));
                None
            }
            1 => {
                if let proc_macro2::TokenTree::Ident(id) = &tokens[0] {
                    Some((id.to_string(), None))
                } else {
                    self.errors.push(errors::Error::Format(
                        location_from_span(self.path, &tokens[0].span()),
                        "requirement_covered!() single argument is not an identifier".to_owned(),
                    ));
                    None
                }
            }

            3 => {
                // TODO: match Ident and String Literal
                let id = tokens[0].to_string(); // an identifier
                let mut title = tokens[2].to_string(); // a literal String
                if title.starts_with('"') && title.ends_with('"') {
                    title = title.replace('"', ""); // TODO: better parsing?
                } else {
                    todo!("an error");
                }
                Some((id, Some(title)))
            }
            _ => {
                self.errors.push(errors::Error::Format(
                    location_from_span(self.path, &seg[0].ident.span()),
                    format!("requirement_covered!() called more than 2 argument/token {tokens:?}"),
                ));
                None
            }
        } {
            let reference = Reference {
                id: referenced_id,
                title,
                location: None,
            };

            // TODO: add info from path of source file to make symbols unique
            let id = self.scope.join("::");

            match self.requirements.last_mut() {
                Some(last) if last.id == id => {
                    Rc::get_mut(last).unwrap().covers.push(reference);
                }
                _ => {
                    let covers = vec![reference];
                    let req = Requirement {
                        id,
                        location: location_from_span(self.path, &seg[0].ident.span()),
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
        self.scope.push(node.sig.ident.to_string());
        visit::visit_item_fn(self, node);
        self.scope.pop().unwrap();
    }

    fn visit_impl_item_method(&mut self, node: &'ast syn::ImplItemMethod) {
        self.scope.push(node.sig.ident.to_string());
        visit::visit_impl_item_method(self, node);
        self.scope.pop().unwrap();
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
        self.scope.push(symbol);
        visit::visit_item_impl(self, node);
        self.scope.pop().unwrap();
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        self.scope.push(node.ident.to_string());
        visit::visit_item_trait(self, node);
        self.scope.pop().unwrap();
    }

    fn visit_trait_item_method(&mut self, node: &'ast syn::TraitItemMethod) {
        self.scope.push(node.sig.ident.to_string());
        visit::visit_trait_item_method(self, node);
        self.scope.pop().unwrap();
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        self.scope.push(node.ident.to_string());
        visit::visit_item_mod(self, node);
        self.scope.pop().unwrap();
    }
}

fn location_from_span(path: &Path, span: &Span) -> Location {
    let start = span.start();
    Location::new_with_line_and_column(path.to_path_buf(), start.line, start.column)
}

#[cfg(test)]
mod test {
    use std::{io::BufReader, path::PathBuf};

    use crate::common::RequirementBuilder;

    use super::*;

    use pretty_assertions::assert_eq;

    macro_rules! assert_equal_repr {
        ($expected:expr, $actual:expr) => {
            assert_eq!(format!("{:?}", $expected), format!("{}", $actual))
        };
    }

    #[test]
    fn test_id_only() {
        let s = "
            mod module_name {
                impl Trait for Struct {
                    fn foo() {
                        requirement_covered!(REQ_ID);
                    }
                }
            }
        "
        .to_owned();

        let (reqs, errors) = parse(
            &mut BufReader::new(s.as_bytes()),
            &PathBuf::from("src/filename.rs"),
        );
        assert_equal_repr!(errors.len(), 0);
        assert_eq!(
            *reqs[0],
            RequirementBuilder::new("module_name::(Struct as Trait)::foo")
                .location("src/filename.rs:5:24")
                .unwrap()
                .covers("REQ_ID", None, None)
                .unwrap() // TODO:"src/filename.rs:5:24")?
                .build()
        );
    }
    #[test]
    fn test_id_with_title() {
        let s = r#"
            mod module_name {
                impl Trait for Struct {
                    fn foo() {
                        requirement_covered!(REQ_ID, "Title String");
                    }
                }
            }
        "#
        .to_owned();

        let (reqs, errors) = parse(
            &mut BufReader::new(s.as_bytes()),
            &PathBuf::from("src/filename.rs"),
        );
        assert_equal_repr!(errors, "[]");

        assert_eq!(
            *reqs[0],
            RequirementBuilder::new("module_name::(Struct as Trait)::foo")
                .location("src/filename.rs:5:24")
                .unwrap()
                .covers("REQ_ID", Some("Title String"), None)
                .unwrap() // TODO:"src/filename.rs:5:24")?
                .build()
        );
    }
}
