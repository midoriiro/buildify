use crate::constants::RESERVED_TYPES;
use crate::field::Field;
use crate::generator::Generator;
use ast_shaper::items::item::{Item, ItemTrait};
use ast_shaper::utils::path::Path;
use ast_shaper::utils::{create_generic_type, create_ident};
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use syn::PathArguments;

#[derive(Clone)]
pub(crate) struct ComplexField {
    pub ident: String,
    pub inner: Vec<Field>
}

#[derive(Clone)]
pub(crate) struct VecField {
    pub item: Rc<FieldTypeSegment>
}

#[derive(Clone)]
pub(crate) struct MapField {
    pub key: Rc<FieldTypeSegment>,
    pub value: Rc<FieldTypeSegment>,
}

#[derive(Clone)]
pub(crate) struct OptionField {
    pub ty: Path,
    pub underlying_ty: Rc<FieldTypeSegment>,
}

#[derive(Clone)]
pub(crate) struct GenericField {
    pub ty: Path,
    pub underlying_ty: Rc<FieldTypeSegment>,
}

#[derive(Clone)]
pub(crate) struct RemappedField {
    pub source: Rc<FieldTypeSegment>,
    pub target: Rc<FieldTypeSegment>,
}

#[derive(Clone)]
pub(crate) enum InnerFieldTypeSegment {
    Reserved(Path),
    Complex(ComplexField),
    Vec(VecField),
    Map(MapField),
    Option(OptionField),
    Generic(GenericField),
    Remap(RemappedField)
}

impl InnerFieldTypeSegment {
    pub(crate) fn unwrap(&self) -> Path {
        match self {
            InnerFieldTypeSegment::Reserved(value) => {
                value.clone()
            }
            InnerFieldTypeSegment::Complex(value) => {
                let ty = value.ident.clone();
                Path::from(Generator::ident(&create_ident(ty)))
            }
            InnerFieldTypeSegment::Vec(value) => {
                create_generic_type(
                    "Vec",
                    vec![value.item.inner.unwrap()],
                )
            }
            InnerFieldTypeSegment::Map(value) => {
                create_generic_type(
                    "HashMap",
                    vec![value.key.inner.unwrap(), value.value.inner.unwrap()],
                )
            }
            InnerFieldTypeSegment::Option(value) => {
                value.underlying_ty.unwrap()
            }
            InnerFieldTypeSegment::Generic(value) => {
                create_generic_type(
                    value.ty.last().unwrap().ident.to_string(),
                    vec![value.underlying_ty.unwrap()],
                )
            }
            InnerFieldTypeSegment::Remap(value) => {
                value.target.unwrap()
            }
        }
    }
}

impl Display for InnerFieldTypeSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InnerFieldTypeSegment::Reserved(value) => {
                write!(f, "{}", value)
            }
            InnerFieldTypeSegment::Complex(value) => {
                write!(f, "{}", value.ident)
            }
            InnerFieldTypeSegment::Vec(_) => {
                write!(f, "{}", self.unwrap())
            }
            InnerFieldTypeSegment::Map(_) => {
                write!(f, "{}", self.unwrap())
            }
            InnerFieldTypeSegment::Option(value) => {
                let ty = create_generic_type(
                    "Option",
                    vec![value.ty.clone(), value.underlying_ty.unwrap()],
                );
                write!(f, "{}", ty)
            }
            InnerFieldTypeSegment::Generic(_) => {
                write!(f, "{}", self.unwrap())
            }
            InnerFieldTypeSegment::Remap(value) => {
                write!(f, "{}", value.source.unwrap())
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct FieldTypeSegment {
    pub inner: InnerFieldTypeSegment
}

impl FieldTypeSegment {
    pub fn new(generator: &Generator, ty: Path) -> Self {
        let ty = ty.flatten();
        let (ty, underlying_ty) = Self::unwrap_underlying(generator, &ty);
        let ty_segment = ty.last().unwrap().clone();
        let ty_ident = ty_segment.ident.to_string();
        if ty_ident == "Vec" {
            Self {
                inner: InnerFieldTypeSegment::Vec(VecField {
                    item: Rc::new(underlying_ty.unwrap().get(0).unwrap().to_owned()),
                }),
            }
        }
        else if ty_ident == "HashMap" {
            let underlying_ty = underlying_ty.unwrap();
            Self {
                inner: InnerFieldTypeSegment::Map(MapField {
                    key: Rc::new(underlying_ty.get(0).unwrap().to_owned()),
                    value: Rc::new(underlying_ty.get(1).unwrap().to_owned()),
                }),
            }
        }
        else if ty_ident == "Option" {
            Self {
                inner: InnerFieldTypeSegment::Option(OptionField {
                    ty,
                    underlying_ty: Rc::new(underlying_ty.unwrap().get(0).unwrap().to_owned()),
                }),
            }
        }
        else if RESERVED_TYPES.contains(&ty_ident.as_str()) {
            Self {
                inner: InnerFieldTypeSegment::Reserved(ty)
            }
        }
        else if let Some(item) = generator.find_item(&ty_ident) {
            let (ident, inner) = Self::wrap(generator, item);
            Self {
                inner: InnerFieldTypeSegment::Complex(ComplexField {
                    ident,
                    inner
                }),
            }
        }
        else {
            Self {
                inner: InnerFieldTypeSegment::Generic(GenericField {
                    ty,
                    underlying_ty: Rc::new(underlying_ty.unwrap().get(0).unwrap().to_owned()),
                })
            }
        }
    }

    pub fn map(generator: &Generator, source: FieldTypeSegment, target_ty: Path) -> Self {
        let target = Self::new(generator, target_ty);
        Self {
            inner: InnerFieldTypeSegment::Remap(RemappedField {
                source: Rc::new(source),
                target: Rc::new(target),

            }),
        }
    }

    pub(self) fn wrap(generator: &Generator, item: Item) -> (String, Vec<Field>) {
        let (item_ident, item) = match &item {
            Item::Struct(value) => {
                let item = value.clone();
                (item.ident(), syn::Item::Struct(item.item.clone()))
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

    pub(self) fn unwrap_underlying(generator: &Generator, path: &Path) -> (Path, Option<Vec<FieldTypeSegment>>) {
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
    
    pub(crate) fn is_complex(&self) -> bool {
        match self.inner {
            InnerFieldTypeSegment::Complex(_) => true,
            _ => false
        }
    }

    pub(crate) fn unwrap(&self) -> Path {
        self.inner.unwrap()
    }
}

impl Display for FieldTypeSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}