use std::{
    io,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    common::{Location, Requirement},
    errors,
};

use syn::*;

struct Parser {
    requirements: Vec<Rc<Requirement>>,
    errors: Vec<errors::Error>,
    symbol_stack: Vec<String>,
}

impl Parser {
    fn parse_item(&mut self, item: &Item) {
        {
            match item {
                Item::Fn(function) => {
                    self.symbol_stack.push(function.sig.ident.to_string());
                    self.parse_block(&function.block);
                    self.symbol_stack.pop().expect("balanced stack");
                }
                Item::Impl(i) => {
                    let typ = &i.self_ty;
                    let symbol = match &i.trait_ {
                        Some((bang, path, _for_tok)) => {
                            let bang = {
                                if bang.is_some() {
                                    "!"
                                } else {
                                    ""
                                }
                            };
                            format!("({typ:?} as {bang}{path:?})")
                        }
                        None => format!("{typ:?}"),
                    };
                    self.symbol_stack.push(symbol);

                    for itm in &i.items {
                        match itm {
                            ImplItem::Method(meth) => {
                                self.symbol_stack.push(meth.sig.ident.to_string());
                                self.parse_block(&meth.block);
                                self.symbol_stack.pop().expect("balanced stack");
                            }
                            _ => { /* ignore */ }
                        }
                    }

                    self.symbol_stack.pop().expect("balanced stack");
                }
                Item::Mod(m) => m.content.iter().for_each(|(_brace_tok, items)| {
                    items.iter().for_each(|item| self.parse_item(&item))
                }),
                Item::Verbatim(_) => unreachable!("should have caused parsing error"),
                _ => { /* ignore */ }
            }
        }
    }
    fn parse_block(&mut self, block: &Block) {
        for s in &block.stmts {
            match s {
                Stmt::Local(local) => 
                    local
                        .init
                        .iter()
                        .for_each(|(_eq_tok, exp)| self.parse_expression(&exp)),
                
                Stmt::Item(item) => self.parse_item(item),
                Stmt::Expr(exp) | Stmt::Semi(exp, _) => self.parse_expression(&exp),
            }
        }
    }

    fn parse_expression(&mut self, expression: &Expr) {
        match expression {
            Expr::Array(a) => a.elems.iter().for_each(|exp| self.parse_expression(exp)),
            Expr::Assign(a) => self.parse_expression(a.right),
            Expr::AssignOp(a) => self.parse_expression(a.right),
            Expr::Async(a) => self.parse_block(a.block),
            Expr::Await(_) => todo!(),
            Expr::Binary(_) => todo!(),
            Expr::Block(_) => todo!(),
            Expr::Box(_) => todo!(),
            Expr::Break(_) => todo!(),
            Expr::Call(_) => todo!(),
            Expr::Cast(_) => todo!(),
            Expr::Closure(_) => todo!(),
            Expr::Continue(_) => todo!(),
            Expr::Field(_) => todo!(),
            Expr::ForLoop(_) => todo!(),
            Expr::Group(_) => todo!(),
            Expr::If(_) => todo!(),
            Expr::Index(_) => todo!(),
            Expr::Let(_) => todo!(),
            Expr::Lit(_) => todo!(),
            Expr::Loop(_) => todo!(),
            Expr::Macro(_) => todo!(),
            Expr::Match(_) => todo!(),
            Expr::MethodCall(_) => todo!(),
            Expr::Paren(_) => todo!(),
            Expr::Path(_) => todo!(),
            Expr::Range(_) => todo!(),
            Expr::Reference(_) => todo!(),
            Expr::Repeat(_) => todo!(),
            Expr::Return(_) => todo!(),
            Expr::Struct(_) => todo!(),
            Expr::Try(_) => todo!(),
            Expr::TryBlock(_) => todo!(),
            Expr::Tuple(_) => todo!(),
            Expr::Type(_) => todo!(),
            Expr::Unary(_) => todo!(),
            Expr::Unsafe(_) => todo!(),
            Expr::Verbatim(_) => todo!(),
            Expr::While(_) => todo!(),
            Expr::Yield(_) => todo!(),
            _ => { /* ignore */ }
        }
    }
}

pub fn parse(
    reader: &mut impl io::BufRead,
    path: &Path,
) -> (Vec<Rc<Requirement>>, Vec<errors::Error>) {
    let requirements = Vec::new();
    let mut errors = Vec::new();

    let mut code = String::new();
    match reader.read_to_string(&mut code) {
        Err(e) => {
            errors.push(errors::Error::IoError(path.into(), e));
        }
        Ok(_bytes) => match parse_file(&code) {
            Err(e) => {
                errors.push(errors::Error::FormatError(
                    Location {
                        file: path.into(),
                        line: e
                            .span()
                            .start()
                            .line
                            .try_into()
                            .expect("line numbers fir in u32"),
                        // TODO: use LocationFileLineColumn
                    },
                    e.to_string(),
                ));
            }
            Ok(file) => {
                let mut p = Parser {
                    requirements,
                    errors,
                    symbol_stack: Vec::new(),
                };
                for i in &file.items {
                    p.parse_item(i);
                }
                return (p.requirements, p.errors);
            }
        },
    }

    (requirements, errors)
}


#[cfg(test)]
mod test{
    use super::*;

    fn test_simple() {
        s = "
            fn foo() {
                cov_mark::hit!(REQ_ID); # Requirement Title
            }
            ";

        let (reqs, errors) = parse(s, &PathBuf("filename"));
        assert!(errors.is_empty());
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs.covers.len(), 1);
        let req = reqs[0];
        assert_eq!(req.id, "foo");
        assert_eq!(req.covers[0].id, "REQ_ID");
        assert_eq!(req.covers[0].title, "Requirement Title");
    }
}
