use sway_types::{Ident, Named, Span, Spanned};

use crate::{decl_engine::DeclRefMixedInterface, language::CallPath, type_system::*};

use super::TyTraitItem;

pub type TyImplItem = TyTraitItem;

// impl <A, B, C> Trait<Arg, Arg> for Type<Arg, Arg>
#[derive(Clone, Debug)]
pub struct TyImplTrait {
    pub impl_type_parameters: Vec<TypeParameter>,
    pub trait_name: CallPath,
    pub trait_type_arguments: Vec<TypeArgument>,
    pub items: Vec<TyImplItem>,
    pub trait_decl_ref: Option<DeclRefMixedInterface>,
    pub implementing_for: TypeArgument,
    pub span: Span,
}

impl Named for TyImplTrait {
    fn name(&self) -> &Ident {
        &self.trait_name.suffix
    }
}

impl Spanned for TyImplTrait {
    fn span(&self) -> Span {
        self.span.clone()
    }
}
