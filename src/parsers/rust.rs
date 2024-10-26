use std::collections::BTreeMap;
use std::fs;
use std::io::{self};
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use log::warn;
use proc_macro2::Span;
use quote::ToTokens;
use syn::parse_file;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::visit::{self};
use syn::ItemFn;

use crate::models::ArtefactConfig;
use crate::models::Error;
use crate::models::Location;
use crate::models::Reference;
use crate::models::Requirement;
use crate::models::{self};

pub fn parse(reader: &mut impl io::BufRead, path: &Path) -> (Vec<Rc<Requirement>>, Vec<Error>) {
    let requirements = Vec::new();
    let mut errors = Vec::new();

    let mut source = String::new();
    match reader.read_to_string(&mut source) {
        Err(e) => {
            errors.push(Error::io(path, &e));
        }
        Ok(_bytes) => match parse_file(&source) {
            Err(e) => {
                let pos = e.span().start();
                errors.push(Error::Format(
                    Location::new_with_line_and_column(path.into(), pos.line, pos.column),
                    e.to_string(),
                ));
            }
            Ok(file) => {
                let mut p = Parser {
                    source,
                    requirements,
                    errors,
                    path,
                    scope: Vec::new(),
                    locations: Vec::new(),
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
    errors: Vec<Error>,
    scope: Vec<String>,
    locations: Vec<Location>,
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
        let macro_location = location_from_span(self.path, &seg[0].ident.span());

        if let Some((referenced_id, title)) = match tokens.len() {
            0 => {
                // requirement_covered!()
                self.errors.push(Error::Format(
                    macro_location.clone(),
                    "requirement_covered!() has no arguments".to_owned(),
                ));
                None
            }
            1 => {
                // requirement_covered!(IDENT)
                if let proc_macro2::TokenTree::Ident(id) = &tokens[0] {
                    Some((id.to_string(), None))
                } else {
                    let mut id = tokens[0].to_string();
                    if id.starts_with('"') && id.ends_with('"') {
                        // requirement_covered!("REQU_ID")
                        id = id.replace('"', ""); // TODO: better string parsing?
                        Some((id.to_string(), None))
                    } else {
                        self.errors.push(Error::Format(
                            location_from_span(self.path, &tokens[0].span()),
                            "requirement_covered!() single argument is not an identifier"
                                .to_owned(),
                        ));
                        None
                    }
                }
            }

            3 => {
                // requirement_covered!(IDENT,"string_literal")
                if let proc_macro2::TokenTree::Ident(id) = &tokens[0] {
                    let mut title = tokens[2].to_string(); // a literal String
                    if title.starts_with('"') && title.ends_with('"') {
                        title = title.replace('"', ""); // TODO: better string parsing?
                        Some((id.to_string(), Some(title)))
                    } else {
                        // requirement_covered!(IDENT,17)
                        self.errors.push(Error::Format(
                            location_from_span(self.path, &tokens[2].span()),
                            "requirement_covered!() 2nd argument is not String".to_owned(),
                        ));
                        None
                    }
                } else {
                    // requirement_covered!(17, ...)
                    self.errors.push(Error::Format(
                        location_from_span(self.path, &tokens[0].span()),
                        "requirement_covered!() 1st argument is not an identifier".to_owned(),
                    ));
                    None
                }
            }
            _ => {
                self.errors.push(Error::Format(
                    macro_location.clone(),
                    format!(
                        "requirement_covered!() called with more than 2 argument/token {tokens:?}"
                    ),
                ));
                None
            }
        } {
            let reference = Reference {
                id: referenced_id.into(),
                title,
                location: macro_location,
            };

            // TODO: add info from path of source file to make symbols unique
            let id = self.scope.join("::").into();

            match self.requirements.last_mut() {
                Some(last) if last.id == id => {
                    Rc::get_mut(last).unwrap().covers.push(reference);
                }
                _ => {
                    let covers = vec![reference];
                    let req = Requirement {
                        id: id.into(),
                        location: self.locations.last().unwrap().clone(),
                        covers,
                        depends: vec![],
                        tags: vec![],
                        attributes: BTreeMap::new(),
                        title: None,
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
        self.locations
            .push(location_from_span(self.path, &node.sig.ident.span()));
        visit::visit_item_fn(self, node);
        self.locations.pop().unwrap();
        self.scope.pop().unwrap();
    }

    fn visit_impl_item_method(&mut self, node: &'ast syn::ImplItemMethod) {
        self.scope.push(node.sig.ident.to_string());
        self.locations
            .push(location_from_span(self.path, &node.sig.ident.span()));
        visit::visit_impl_item_method(self, node);
        self.locations.pop().unwrap();
        self.scope.pop().unwrap();
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        let type_ = node.self_ty.to_token_stream().to_string();
        let symbol;
        if let Some((bang, trait_, for_tok)) = &node.trait_ {
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
        self.locations
            .push(location_from_span(self.path, &node.self_ty.span()));
        visit::visit_item_impl(self, node);
        self.locations.pop().unwrap();
        self.scope.pop().unwrap();
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        self.scope.push(node.ident.to_string());
        self.locations
            .push(location_from_span(self.path, &node.ident.span()));
        visit::visit_item_trait(self, node);
        self.locations.pop().unwrap();
        self.scope.pop().unwrap();
    }

    fn visit_trait_item_method(&mut self, node: &'ast syn::TraitItemMethod) {
        self.scope.push(node.sig.ident.to_string());
        self.locations
            .push(location_from_span(self.path, &node.sig.ident.span()));
        visit::visit_trait_item_method(self, node);
        self.locations.pop().unwrap();
        self.scope.pop().unwrap();
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        self.scope.push(node.ident.to_string());
        self.locations
            .push(location_from_span(self.path, &node.ident.span()));
        visit::visit_item_mod(self, node);
        self.locations.pop().unwrap();
        self.scope.pop().unwrap();
    }
}

fn location_from_span(path: &Path, span: &Span) -> Location {
    let start = span.start();
    Location::new_with_line_and_column(path.to_path_buf(), start.line, start.column)
}

#[cfg(test)]
mod test {
    use std::io::BufReader;
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::util::RequirementBuilder;

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
                .location("src/filename.rs:4:23")
                .unwrap()
                .covers("REQ_ID", None, "src/filename.rs:5:24")
                .unwrap()
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
                .location("src/filename.rs:4:23")
                .unwrap()
                .covers("REQ_ID", Some("Title String"), "src/filename.rs:10:32")
                .unwrap()
                .build()
        );
    }
}
