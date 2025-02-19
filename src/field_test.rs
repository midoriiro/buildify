use crate::generator::Generator;
use crate::test_utils::fixtures::{generator, struct_with_boxed_field, struct_with_complex_field, struct_with_map_of_primitive_field, struct_with_optional_and_boxed_field, struct_with_optional_and_optional_field, struct_with_optional_field, struct_with_ref_counter_and_refcell_field, struct_with_required_field, struct_with_vec_of_primitive_field};
use ast_shaper::items::module_item::ModuleItem;
use ast_shaper::utils::path::Path;
use crate::test_utils::asserts::{assert_builder, assert_method};
use quote::quote;
use rstest::rstest;

#[rstest]
fn as_required(
    generator: Generator,
    struct_with_required_field: ModuleItem
) {
    let (item_ident, item) = assert_builder(
        &struct_with_required_field, 
        &generator,
        Path::new("Option").with(Path::new("u32")).to_owned()
    );
    let functions = &item.impl_items.first().unwrap().functions;
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
    struct_with_optional_field: ModuleItem
) {
    let (item_ident, item) = assert_builder(
        &struct_with_optional_field,
        &generator,
        Path::new("Option").with(Path::new("u32")).to_owned()
    );
    let functions = &item.impl_items.first().unwrap().functions;
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
    struct_with_optional_and_optional_field: ModuleItem
) {
    let (item_ident, item) = assert_builder(
        &struct_with_optional_and_optional_field,
        &generator,
        Path::new("Option").with(Path::new("u32")).to_owned()
    );
    let functions = &item.impl_items.first().unwrap().functions;
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
    struct_with_boxed_field: ModuleItem
) {
    let (item_ident, item) = assert_builder(
        &struct_with_boxed_field,
        &generator,
        Path::new("Option")
            .with(Path::new("Box").with(Path::new("u32")).to_owned())
            .to_owned()
    );
    let functions = &item.impl_items.first().unwrap().functions;
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
    struct_with_optional_and_boxed_field: ModuleItem
) {
    let (item_ident, item) = assert_builder(
        &struct_with_optional_and_boxed_field,
        &generator,
        Path::new("Option")
            .with(Path::new("Box").with(Path::new("u32")).to_owned())
            .to_owned()
    );
    let functions = &item.impl_items.first().unwrap().functions;
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
    struct_with_ref_counter_and_refcell_field: ModuleItem
) {
    let (item_ident, item) = assert_builder(
        &struct_with_ref_counter_and_refcell_field,
        &generator,
        Path::new("Option")
            .with(Path::new("Rc")
                .with(Path::new("RefCell").with(Path::new("u32")).to_owned())
                .to_owned()
            )
            .to_owned()
    );
    let functions = &item.impl_items.first().unwrap().functions;
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
    struct_with_vec_of_primitive_field: ModuleItem
) {
    let (item_ident, item) = assert_builder(
        &struct_with_vec_of_primitive_field,
        &generator,
        Path::new("Option")
            .with(Path::new("Vec").with(Path::new("u32")).to_owned())
            .to_owned()
    );
    let functions = &item.impl_items.first().unwrap().functions;
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
    struct_with_map_of_primitive_field: ModuleItem
) {
    let (item_ident, item) = assert_builder(
        &struct_with_map_of_primitive_field,
        &generator,
        Path::new("Option")
            .with(Path::new("HashMap")
                .with(Path::new("u32"))
                .with(Path::new("u32"))
                .to_owned()
            )
            .to_owned()
    );
    let functions = &item.impl_items.first().unwrap().functions;
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
    struct_with_complex_field: ModuleItem
) {
    let (item_ident, item) = assert_builder(
        &struct_with_complex_field,
        &generator,
        Path::new("Option")
            .with(Path::new("ComplexTypeBuilder"))
            .to_owned()
    );
    let functions = &item.impl_items.first().unwrap().functions;
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
