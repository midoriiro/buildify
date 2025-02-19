use crate::generator::Generator;
use ast_shaper::items::module_item::ModuleItems;
use ast_shaper::items::struct_item::StructItem;
use ast_shaper::utils::create_ident;
use ast_shaper::utils::path::Path;
use ast_shaper::utils::punctuated::PunctuatedExt;
use rstest::fixture;
use std::cell::RefCell;
use std::rc::Rc;
use syn::punctuated::Punctuated;
use syn::{FieldMutability, Fields, FieldsNamed, ItemStruct, Type, TypePath, Visibility};

#[fixture]
pub fn module() -> ModuleItems {
    ModuleItems::new("test_module")
}

#[fixture]
pub fn required_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("u32").to_syn_path(),
        }),
    }
}

#[fixture]
pub fn optional_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("Option").with(Path::new("u32")).to_syn_path(),
        }),
    }
}

#[fixture]
pub fn optional_and_optional_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("Option")
                .with(Path::new("Option").with(Path::new("u32")).to_owned())
                .to_syn_path(),
        }),
    }
}

#[fixture]
pub fn boxed_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("Box").with(Path::new("u32")).to_syn_path(),
        }),
    }
}

#[fixture]
pub fn optional_and_boxed_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("Option")
                .with(Path::new("Box").with(Path::new("u32")).to_owned())
                .to_syn_path(),
        }),
    }
}

#[fixture]
pub fn ref_counter_and_refcell_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("Rc")
                .with(Path::new("RefCell").with(Path::new("u32")).to_owned())
                .to_syn_path(),
        }),
    }
}

#[fixture]
pub fn vec_of_primitive_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("Vec")
                .with(Path::new("u32"))
                .to_syn_path(),
        }),
    }
}

#[fixture]
pub fn map_of_primitive_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("HashMap")
                .with(Path::new("u32"))
                .with(Path::new("u32"))
                .to_syn_path(),
        }),
    }
}

#[fixture]
pub fn complex_field() -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(create_ident("field")),
        colon_token: None,
        ty: Type::Path(TypePath {
            qself: None,
            path: Path::new("ComplexType").to_syn_path(),
        }),
    }
}

#[fixture]
pub fn struct_with_required_field(
    mut module: ModuleItems,
    required_field: syn::Field
) -> ModuleItems {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithRequiredField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(required_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_optional_field(
    mut module: ModuleItems,
    optional_field: syn::Field
) -> ModuleItems {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithOptionalField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(optional_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_optional_and_optional_field(
    mut module: ModuleItems,
    optional_and_optional_field: syn::Field
) -> ModuleItems {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithOptionalAndOptionalField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(optional_and_optional_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_boxed_field(
    mut module: ModuleItems,
    boxed_field: syn::Field
) -> ModuleItems {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithBoxedField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(boxed_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_optional_and_boxed_field(
    mut module: ModuleItems,
    optional_and_boxed_field: syn::Field
) -> ModuleItems {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithOptionalAndBoxedField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(optional_and_boxed_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_ref_counter_and_refcell_field(
    mut module: ModuleItems,
    ref_counter_and_refcell_field: syn::Field
) -> ModuleItems {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithRefCounterAndRefCellField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(ref_counter_and_refcell_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_vec_of_primitive_field(
    mut module: ModuleItems,
    vec_of_primitive_field: syn::Field
) -> ModuleItems {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithVecOfPrimitiveField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(vec_of_primitive_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_map_of_primitive_field(
    mut module: ModuleItems,
    map_of_primitive_field: syn::Field
) -> ModuleItems {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithMapOfPrimitiveField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(map_of_primitive_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn struct_with_complex_field(
    mut module: ModuleItems,
    complex_field: syn::Field,
    required_field: syn::Field
) -> ModuleItems {
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: create_ident("StructWithComplexField"),
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(complex_field.clone()),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    let item = StructItem::new(
        ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Default::default(),
            ident: match &complex_field.ty {
                Type::Path(value) => {
                    create_ident(value.path.segments.last().unwrap().ident.to_string())
                }
                _ => panic!("Expected path type")
            },
            generics: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: Punctuated::single(required_field),
            }),
            semi_token: None,
        },
        vec![]
    );
    module.push_struct_item(item);
    module
}

#[fixture]
pub fn generator(
    struct_with_required_field: ModuleItems,
    struct_with_optional_field: ModuleItems,
    struct_with_optional_and_optional_field: ModuleItems,
    struct_with_boxed_field: ModuleItems,
    struct_with_ref_counter_and_refcell_field: ModuleItems,
    struct_with_optional_and_boxed_field: ModuleItems,
    struct_with_vec_of_primitive_field: ModuleItems,
    struct_with_complex_field: ModuleItems,
) -> Generator {
    let modules = Rc::new(RefCell::new(vec![
        struct_with_required_field,
        struct_with_optional_field,
        struct_with_optional_and_optional_field,
        struct_with_boxed_field,
        struct_with_ref_counter_and_refcell_field,
        struct_with_optional_and_boxed_field,
        struct_with_vec_of_primitive_field,
        struct_with_complex_field
    ]));
    Generator::new(modules)
}