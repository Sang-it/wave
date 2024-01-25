use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::evaluator::Primitive;
use crate::Runtime;
use wave_allocator::{Box, Vec};
use wave_ast::ast::{BindingPatternKind, Declaration, VariableDeclaration, VariableDeclarator};
use wave_diagnostics::Result;

impl<'a> Runtime<'a> {
    pub fn eval_declaration(
        &self,
        declaration: &Declaration<'a>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        match declaration {
            Declaration::VariableDeclaration(declaration) => {
                self.eval_variable_declaration(declaration, environment)
            }
            Declaration::FunctionDeclaration(declaration) => {
                self.eval_function(declaration, environment)
            }
            Declaration::ClassDeclaration(declaration) => {
                self.eval_class_declaration(declaration, environment)
            }
        }
    }

    pub fn eval_variable_declaration(
        &self,
        declaration: &Box<'_, VariableDeclaration<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        self.eval_variable_declarator(&declaration.declarations, environment)
    }

    pub fn eval_variable_declarator(
        &self,
        declarators: &Vec<'_, VariableDeclarator<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        for declarator in declarators {
            if declarator.init.is_none() {
                continue;
            }
            match &declarator.id.kind {
                BindingPatternKind::BindingIdentifier(identifier) => {
                    let value = self.eval_expression(
                        declarator.init.as_ref().unwrap(),
                        Rc::clone(&environment),
                    )?;
                    environment
                        .borrow_mut()
                        .define(identifier.name.to_owned(), value);
                }
            }
        }
        Ok(Primitive::Null)
    }
}
