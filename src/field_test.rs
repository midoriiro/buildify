use crate::generator::Generator;
use ast_shaper::items::fn_item::FnItem;
use ast_shaper::items::item::ItemTrait;
use ast_shaper::items::module_item::ModuleItems;
use ast_shaper::items::struct_item::StructItem;
use ast_shaper::utils::create_ident;
use ast_shaper::utils::path::Path;
use ast_shaper::utils::punctuated::PunctuatedExt;
use quote::{quote};
use rstest::{fixture, rstest};
use std::cell::RefCell;
use std::rc::Rc;
use quote::__private::TokenStream;
use syn::punctuated::Punctuated;
use syn::{parse2, FieldMutability, Fields, FieldsNamed, FnArg, ImplItemFn, ItemStruct, Type, TypePath, Visibility};
use pretty_assertions::{assert_eq, assert_ne};

#[fixture]
fn module() -> ModuleItems {
    ModuleItems::new("test_module")
}

#[fixture]
fn required_field() -> syn::Field {
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
fn optional_field() -> syn::Field {
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
fn optional_and_optional_field() -> syn::Field {
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
fn boxed_field() -> syn::Field {
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
fn optional_and_boxed_field() -> syn::Field {
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
fn ref_counter_and_refcell_field() -> syn::Field {
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
fn vec_of_primitive_field() -> syn::Field {
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
fn map_of_primitive_field() -> syn::Field {
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
fn complex_field() -> syn::Field {
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
fn struct_with_required_field(
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
fn struct_with_optional_field(
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
fn struct_with_optional_and_optional_field(
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
fn struct_with_boxed_field(
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
fn struct_with_optional_and_boxed_field(
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
fn struct_with_ref_counter_and_refcell_field(
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
fn struct_with_vec_of_primitive_field(
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
fn struct_with_map_of_primitive_field(
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
fn struct_with_complex_field(
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
fn generator(
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

fn assert_builder(
    module: ModuleItems,
    generator: Generator,
    expected_field_type: Path
) -> (TokenStream, Vec<FnItem>) {
    let item = module.items.first().unwrap();
    let item_ident = item.ident();
    let item_ident_quote = item_ident.parse::<TokenStream>().unwrap();
    let items = generator.generate(&item.to_syn_item());
    let item = items.first().unwrap();
    assert_eq!(
        format!("{}Builder", item_ident), 
        item.ident()
    );
    let field = match &item.item.fields {
        Fields::Named(value) => value.named.first().unwrap(),
        _ => panic!("Expected named field")
    };
    let field_type = match &field.ty {
        Type::Path(value) => Path::from(value.path.clone()),
        _ => panic!("Expected path type")
    };
    assert_eq!(
        expected_field_type,
        field_type
    );
    (item_ident_quote, item.impl_items.first().unwrap().functions.clone())
}

fn assert_method(functions: &Vec<FnItem>, expected_method: TokenStream) {
    let expected_method: ImplItemFn = parse2(expected_method).unwrap();
    let method_ident = expected_method.sig.ident;
    let method = functions.iter()
        .find(|function| {
            function.ident() == method_ident.to_string()
        });
    assert_eq!(true, method.is_some());
    let method = method.unwrap();
    let method_arguments = &method.signature().inputs;
    let expected_method_arguments = &expected_method.sig.inputs;
    assert_eq!(expected_method_arguments.len(), method_arguments.len());
    for index in 0..method_arguments.len() {
        assert_eq!(
            expected_method_arguments[index],
            method_arguments[index]
        );
    }
    assert_eq!(
        expected_method.sig.output,
        method.signature().output
    );
    let method_statements = &method.item.block().stmts;
    let expected_method_statements = &expected_method.block.stmts;
    assert_eq!(expected_method_statements.len(), method_statements.len());
    for index in 0..method_statements.len() {
        assert_eq!(
            expected_method_statements[index],
            method_statements[index]
        );
    }
}

#[rstest]
fn as_required(
    generator: Generator,
    struct_with_required_field: ModuleItems
) {
    let (item_ident, functions) = assert_builder(
        struct_with_required_field, 
        generator,
        Path::new("Option").with(Path::new("u32")).to_owned()
    );
    assert_method(
        &functions,
        quote! {
            pub fn new() -> Self {
                Self {
                    field: None
                }
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn with_field(&mut self, value: u32) -> &mut Self {
                self.field = Some(value);
                self
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn build(&self) -> #item_ident {
                #item_ident {
                    field: if let Some(value) = self.field.clone() {
                        value
                    }
                    else {
                        panic!("field 'field' is required");
                    }
                }
            }
        }
    );
}

#[rstest]
fn as_optional(
    generator: Generator,
    struct_with_optional_field: ModuleItems
) {
    let (item_ident, functions) = assert_builder(
        struct_with_optional_field,
        generator,
        Path::new("Option").with(Path::new("u32")).to_owned()
    );
    assert_method(
        &functions,
        quote! {
            pub fn new() -> Self {
                Self {
                    field: None
                }
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn with_field(&mut self, value: u32) -> &mut Self {
                self.field = Some(value);
                self
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn build(&self) -> #item_ident {
                #item_ident {
                    field: if let Some(value) = self.field.clone() {
                        Some(value)
                    }
                    else {
                        None
                    }
                }
            }
        }
    );
}

#[rstest]
fn as_optional_and_optional(
    generator: Generator,
    struct_with_optional_and_optional_field: ModuleItems
) {
    let (item_ident, functions) = assert_builder(
        struct_with_optional_and_optional_field,
        generator,
        Path::new("Option").with(Path::new("u32")).to_owned()
    );
    assert_method(
        &functions,
        quote! {
            pub fn new() -> Self {
                Self {
                    field: None
                }
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn with_field(&mut self, value: u32) -> &mut Self {
                self.field = Some(value);
                self
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn build(&self) -> #item_ident {
                #item_ident {
                    field: if let Some(value) = self.field.clone() {
                        Some(value)
                    }
                    else {
                        None
                    }
                }
            }
        }
    );
}

#[rstest]
fn as_boxed(
    generator: Generator,
    struct_with_boxed_field: ModuleItems
) {
    let (item_ident, functions) = assert_builder(
        struct_with_boxed_field,
        generator,
        Path::new("Option")
            .with(Path::new("Box").with(Path::new("u32")).to_owned())
            .to_owned()
    );
    assert_method(
        &functions,
        quote! {
            pub fn new() -> Self {
                Self {
                    field: None
                }
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn with_field(&mut self, value: Box<u32>) -> &mut Self {
                self.field = Some(value);
                self
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn build(&self) -> #item_ident {
                #item_ident {
                    field: if let Some(value) = self.field.clone() {
                        value
                    }
                    else {
                        panic!("field 'field' is required");
                    }
                }
            }
        }
    );
}

#[rstest]
fn as_optional_and_boxed(
    generator: Generator,
    struct_with_optional_and_boxed_field: ModuleItems
) {
    let (item_ident, functions) = assert_builder(
        struct_with_optional_and_boxed_field,
        generator,
        Path::new("Option")
            .with(Path::new("Box").with(Path::new("u32")).to_owned())
            .to_owned()
    );
    assert_method(
        &functions,
        quote! {
            pub fn new() -> Self {
                Self {
                    field: None
                }
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn with_field(&mut self, value: Box<u32>) -> &mut Self {
                self.field = Some(value);
                self
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn build(&self) -> #item_ident {
                #item_ident {
                    field: if let Some(value) = self.field.clone() {
                        Some(value)
                    }
                    else {
                        None
                    }
                }
            }
        }
    );
}

#[rstest]
fn as_ref_counter_and_ref_cell(
    generator: Generator,
    struct_with_ref_counter_and_refcell_field: ModuleItems
) {
    let (item_ident, functions) = assert_builder(
        struct_with_ref_counter_and_refcell_field,
        generator,
        Path::new("Option")
            .with(Path::new("Rc")
                .with(Path::new("RefCell").with(Path::new("u32")).to_owned())
                .to_owned()
            )
            .to_owned()
    );
    assert_method(
        &functions,
        quote! {
            pub fn new() -> Self {
                Self {
                    field: None
                }
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn with_field(&mut self, value: Rc<RefCell<u32>>) -> &mut Self {
                self.field = Some(value);
                self
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn build(&self) -> #item_ident {
                #item_ident {
                    field: if let Some(value) = self.field.clone() {
                        value
                    }
                    else {
                        panic!("field 'field' is required");
                    }
                }
            }
        }
    );
}

#[rstest]
fn as_vec_of_primitive(
    generator: Generator,
    struct_with_vec_of_primitive_field: ModuleItems
) {
    let (item_ident, functions) = assert_builder(
        struct_with_vec_of_primitive_field,
        generator,
        Path::new("Option")
            .with(Path::new("Vec").with(Path::new("u32")).to_owned())
            .to_owned()
    );
    assert_method(
        &functions,
        quote! {
            pub fn new() -> Self {
                Self {
                    field: None
                }
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn with_field(&mut self, value: u32) -> &mut Self {
                if let None = self.field {
                    self.field = Some(Vec::new());
                }
                self.field.as_mut().unwrap().push(value);
                self
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn build(&self) -> #item_ident {
                #item_ident {
                    field: if let Some(value) = self.field.clone() {
                        value
                    }
                    else {
                        panic!("field 'field' is required");
                    }
                }
            }
        }
    );
}

#[rstest]
fn as_map_of_primitive(
    generator: Generator,
    struct_with_map_of_primitive_field: ModuleItems
) {
    let (item_ident, functions) = assert_builder(
        struct_with_map_of_primitive_field,
        generator,
        Path::new("Option")
            .with(Path::new("HashMap")
                .with(Path::new("u32"))
                .with(Path::new("u32"))
                .to_owned()
            )
            .to_owned()
    );
    assert_method(
        &functions,
        quote! {
            pub fn new() -> Self {
                Self {
                    field: None
                }
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn with_field(&mut self, key: u32, value: u32) -> &mut Self {
                if let None = self.field {
                    self.field = Some(HashMap::new());
                }
                self.field.as_mut().unwrap().insert(key, value);
                self
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn build(&self) -> #item_ident {
                #item_ident {
                    field: if let Some(value) = self.field.clone() {
                        value
                    }
                    else {
                        panic!("field 'field' is required");
                    }
                }
            }
        }
    );
}

#[rstest]
fn as_complex(
    generator: Generator,
    struct_with_complex_field: ModuleItems
) {
    let (item_ident, functions) = assert_builder(
        struct_with_complex_field,
        generator,
        Path::new("Option")
            .with(Path::new("ComplexTypeBuilder"))
            .to_owned()
    );
    assert_method(
        &functions,
        quote! {
            pub fn new() -> Self {
                Self {
                    field: None
                }
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn with_field(&mut self) -> &mut ComplexTypeBuilder {
                if let None = self.field {
                    self.field = Some(ComplexTypeBuilder::new());
                }
                self.field.as_mut().unwrap()
            }
        }
    );
    assert_method(
        &functions,
        quote! {
            pub fn build(&self) -> #item_ident {
                #item_ident {
                    field: if let Some(value) = self.field.clone() {
                        value.build()
                    }
                    else {
                        panic!("field 'field' is required");
                    }
                }
            }
        }
    );
}
