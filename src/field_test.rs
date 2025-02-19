use crate::generator::Generator;
use crate::test_utils::fixtures::{generator, struct_with_boxed_field, struct_with_complex_field, struct_with_map_of_primitive_field, struct_with_optional_and_boxed_field, struct_with_optional_and_optional_field, struct_with_optional_field, struct_with_ref_counter_and_refcell_field, struct_with_required_field, struct_with_vec_of_primitive_field};
use ast_shaper::items::fn_item::FnItem;
use ast_shaper::items::item::ItemTrait;
use ast_shaper::items::module_item::ModuleItems;
use ast_shaper::utils::path::Path;
use pretty_assertions::assert_eq;
use quote::__private::TokenStream;
use quote::quote;
use rstest::rstest;
use syn::{parse2, Fields, ImplItemFn, Type};

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
