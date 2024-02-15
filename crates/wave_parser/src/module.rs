use wave_allocator::Vec;
use wave_ast::ast::{
    IdentifierName, ImportDeclarationSpecifier, ImportSpecifier, ModuleDeclaration,
    ModuleExportName, Statement,
};
use wave_diagnostics::Result;
use wave_lexer::Kind;

use crate::{
    list::{ImportSpecifierList, SeparatedList},
    Parser,
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_import_declaration(&mut self) -> Result<Statement<'a>> {
        let span = self.start_span();

        self.bump_any();

        let specifiers = Some(self.parse_import_declaration_specifiers()?);
        let source = self.parse_literal_string()?;

        self.asi()?;
        let span = self.end_span(span);
        let decl = ModuleDeclaration::ImportDeclaration(
            self.ast.import_declaration(span, specifiers, source),
        );
        Ok(self.ast.module_declaration(decl))
    }

    fn parse_import_declaration_specifiers(
        &mut self,
    ) -> Result<Vec<'a, ImportDeclarationSpecifier>> {
        let mut specifiers = self.ast.new_vec();

        if self.at(Kind::LCurly) {
            let mut import_specifiers = self.parse_import_specifiers()?;
            specifiers.append(&mut import_specifiers);
        };

        self.expect(Kind::From)?;
        Ok(specifiers)
    }

    fn parse_import_specifiers(&mut self) -> Result<Vec<'a, ImportDeclarationSpecifier>> {
        let specifiers = ImportSpecifierList::parse(self)?.import_specifiers;
        Ok(specifiers)
    }

    pub(crate) fn parse_import_specifier(&mut self) -> Result<ImportSpecifier> {
        let specifier_span = self.start_span();
        let local = self.parse_binding_identifier()?;
        let imported = IdentifierName {
            span: local.span,
            name: local.name.clone(),
        };

        Ok(ImportSpecifier {
            span: self.end_span(specifier_span),
            imported: ModuleExportName::Identifier(imported),
        })
    }
}
