use crate::{environment::Environment, Runtime};
use std::{cell::RefCell, env, rc::Rc};
use wave_allocator::Allocator;
use wave_ast::ast::{ImportDeclarationSpecifier, ModuleDeclaration, ModuleExportName};
use wave_diagnostics::Result;
use wave_parser::{Parser, ParserReturn};
use wave_span::Span;

use super::Primitive;

impl<'a> Runtime<'a> {
    pub fn eval_import_statement(
        &self,
        statement: &ModuleDeclaration<'a>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        match statement {
            ModuleDeclaration::ImportDeclaration(import_stmt) => {
                let source = &import_stmt.source;
                let source = source.value.to_string();
                let source = source.trim_matches('\"');

                if let Some(specifiers) = &import_stmt.specifiers {
                    let path = env::current_dir()
                        .expect("failed to get current directory")
                        .join(source);

                    let source_text =
                        std::fs::read_to_string(path).expect("failed to read source file");
                    let allocator = Allocator::default();
                    let ret = Parser::new(&allocator, &source_text).parse();

                    unsafe {
                        let ret = std::mem::transmute::<ParserReturn<'_>, ParserReturn<'_>>(ret);

                        if ret.errors.is_empty() {
                            let program = ret.program;
                            let runtime = Runtime::new(program);
                            let imported = Runtime::eval_environment(&runtime);
                            match imported {
                                Ok(env) => {
                                    for specifier in specifiers {
                                        match specifier {
                                            ImportDeclarationSpecifier::ImportSpecifier(
                                                import_specifier,
                                            ) => match &import_specifier.imported {
                                                ModuleExportName::Identifier(identifier) => {
                                                    let value = env.borrow().get(
                                                        identifier.name.clone(),
                                                        Span::default(),
                                                    )?;
                                                    environment
                                                        .borrow_mut()
                                                        .define(identifier.name.clone(), value);
                                                }
                                            },
                                        }
                                    }
                                }
                                Err(error) => {
                                    let error = error.with_source_code(source_text.clone());
                                    println!("{error:?}");
                                }
                            }
                        } else {
                            for error in ret.errors {
                                let error = error.with_source_code(source_text.clone());
                                println!("{error:?}");
                            }
                        }
                    }
                }
            }
        }

        Ok(Primitive::Null)
    }
}
