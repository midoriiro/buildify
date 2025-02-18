use syn::PathArguments;
use ast_shaper::items::item::{Item, ItemTrait};
use ast_shaper::utils::create_generic_type;
use ast_shaper::utils::path::Path;
use crate::builder_generator::BuilderGenerator;
use crate::constants::{DECOMPOSABLE_TYPES, RESERVED_TYPES};
use crate::field::Field;

pub(crate) struct FieldTypeSegment {
    pub ty: Path,
    pub underlying_ty: Option<Vec<FieldTypeSegment>>,
    pub inner_fields: Option<Vec<Field>>,
    pub is_complex: bool,
    pub is_vector: bool,
    pub is_map: bool,
}

impl FieldTypeSegment {
    pub fn new(generator: &BuilderGenerator, ty: Path) -> Self {
        let ty = ty.flatten();
        let (ty, underlying_ty) = Self::decompose_underlying(generator, &ty);
        let ty_segment = ty.last().unwrap().clone();
        let ty_ident = ty_segment.ident.to_string();
        let is_vector = ty_ident == "Vec";
        let is_map = ty_ident == "HashMap";
        let complex_item = generator.find_item(&ty_ident);
        let is_complex = complex_item.is_some() && RESERVED_TYPES.contains(&ty_ident.as_str()) == false;
        let inner_fields = match is_complex {
            true => Some(Self::compose(generator, complex_item.unwrap())),
            false => None
        };
        let ty = match &inner_fields {
            Some((ident, _)) => Path::new(ident),
            None => ty
        };
        let underlying_ty = match &inner_fields {
            Some(_) => None,
            None => underlying_ty
        };
        let inner_fields = match inner_fields {
            Some((_, fields)) => Some(fields),
            None => None
        };
        Self {
            ty,
            underlying_ty,
            inner_fields,
            is_complex,
            is_vector,
            is_map,
        }
    }

    pub(self) fn compose(generator: &BuilderGenerator, item: Item) -> (String, Vec<Field>) {
        let (item_ident, item) = match &item {
            Item::Struct(value) => {
                let item = value.clone();
                (item.ident(), syn::Item::Struct(item.item().clone()))
            },
            Item::Enum(value) => {
                let item = value.clone();
                (item.ident(), syn::Item::Enum(item.item.clone()))
            },
            _ => panic!("Expected struct or enum item")
        };
        let fields = generator.generate_fields(&item);
        (item_ident, fields)
    }

    pub(self) fn unwrap_underlying_type(&self) -> &FieldTypeSegment {
        for underlying_type in self.underlying_ty.as_ref().unwrap().iter() {
            if DECOMPOSABLE_TYPES.contains(&underlying_type.ty.to_string().as_str()) {
                continue;
            }
            return underlying_type;
        }
        panic!("Underlying type cannot be unwrapped.")
    }

    pub(crate) fn decompose(&self) -> Path {
        match self.inner_fields {
            Some(_) => {
                Path::from(BuilderGenerator::ident(&self.ty.last().unwrap().ident))
            }
            None => {
                match &self.underlying_ty {
                    Some(_) => {
                        let argument = self.unwrap_underlying_type().decompose();
                        let path = create_generic_type(
                            self.ty.last().unwrap().ident.to_string(),
                            vec![argument]
                        );
                        path
                    }
                    None => self.ty.clone()
                }
            }
        }
    }

    pub(self) fn decompose_underlying(generator: &BuilderGenerator, path: &Path) -> (Path, Option<Vec<FieldTypeSegment>>) {
        let segment = path.last().cloned().unwrap();
        match segment.arguments {
            PathArguments::AngleBracketed(_) => {
                let underlying_ty = path.decompose_arguments()
                    .unwrap()
                    .iter()
                    .map(|ty| FieldTypeSegment::new(generator, ty.clone()))
                    .collect::<Vec<_>>();
                (path.clone(), Some(underlying_ty))
            }
            _ => (path.clone(), None)
        }
    }
}