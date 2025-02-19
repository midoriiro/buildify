use crate::generator::Generator;
use crate::test_utils::asserts::{assert_builder_with_rules, assert_method};
use crate::test_utils::fixtures::{generator, struct_with_field_attributes, struct_with_required_field, struct_with_vec_of_primitive_field};
use ast_shaper::items::module_item::ModuleItem;
use ast_shaper::utils::path::Path;
use quote::quote;
use rstest::rstest;

#[rstest]
fn with_incomplete_rule(
    mut generator: Generator,
    struct_with_required_field: ModuleItem
) {
    let (generator, _, _) = assert_builder_with_rules(
        &struct_with_required_field,
        &mut generator,
        Path::new("Option").with(Path::new("u32")).to_owned(),
        vec![
            |generator: &mut Generator| {
                generator.with_rule();
            },
            |generator: &mut Generator| {
                generator.with_rule().for_all();
            },
            |generator: &mut Generator| {
                generator.with_rule().for_all().and_all_fields();
            }
        ]
    );
    assert_eq!(0, generator.field_rules.borrow().len());
}

#[rstest]
fn with_non_existing_item(
    mut generator: Generator,
    struct_with_required_field: ModuleItem
) {
    let (generator, item_ident, item) = assert_builder_with_rules(
        &struct_with_required_field,
        &mut generator,
        Path::new("Option").with(Path::new("u32")).to_owned(),
        vec![
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_item("ItemThatDoesNotExist")
                    .and_all_fields()
                    .then_discard_attribute("an_attribute");
            }
        ]
    );
    assert_eq!(1, generator.field_rules.borrow().len());
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
fn with_non_existing_field(
    mut generator: Generator,
    struct_with_required_field: ModuleItem
) {
    let (generator, item_ident, item) = assert_builder_with_rules(
        &struct_with_required_field,
        &mut generator,
        Path::new("Option").with(Path::new("u32")).to_owned(),
        vec![
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_all()
                    .with_field_ident("field_that_does_not_exist")
                    .then_rename("a_name");
            }
        ]
    );
    assert_eq!(1, generator.field_rules.borrow().len());
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
#[should_panic]
fn rename_all(
    mut generator: Generator,
    struct_with_required_field: ModuleItem
) {
    let (_, _, _) = assert_builder_with_rules(
        &struct_with_required_field,
        &mut generator,
        Path::new("Option").with(Path::new("u32")).to_owned(),
        vec![
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_all()
                    .and_all_fields()
                    .then_rename("a_name");
            }
        ]
    );
}

#[rstest]
#[should_panic]
fn map_all(
    mut generator: Generator,
    struct_with_required_field: ModuleItem
) {
    let (_, _, _) = assert_builder_with_rules(
        &struct_with_required_field,
        &mut generator,
        Path::new("Option").with(Path::new("u32")).to_owned(),
        vec![
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_all()
                    .and_all_fields()
                    .then_map(Path::new("a_type"));
            }
        ]
    );
}

#[rstest]
#[should_panic]
fn map_all_to_vec(
    mut generator: Generator,
    struct_with_required_field: ModuleItem
) {
    let (_, _, _) = assert_builder_with_rules(
        &struct_with_required_field,
        &mut generator,
        Path::new("Option").with(Path::new("u32")).to_owned(),
        vec![
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_all()
                    .and_all_fields()
                    .then_map_to_vec(Path::new("a_type"));
            }
        ]
    );
}

#[rstest]
fn field_type_selector(
    mut generator: Generator,
    struct_with_field_attributes: ModuleItem
) {
    let (generator, _, item) = assert_builder_with_rules(
        &struct_with_field_attributes,
        &mut generator,
        Path::new("Option").with(Path::new("u32")).to_owned(),
        vec![
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_item("StructWithFieldAttributes")
                    .with_field_type("u32")
                    .then_discard_attribute("attribute_as_path");
            },
        ]
    );
    assert_eq!(1, generator.field_rules.borrow().len());
    let struct_item = item.item;
    for (index, field) in struct_item.fields.iter().enumerate() {
        assert_eq!(
            match index {
                0 => {
                    format!("{}: {}", index, 0)
                }
                _ => format!("{}: {}", index, 1)
            },
            format!("{}: {}", index, field.attrs.len())
        );
    }
}

#[rstest]
fn discard_attributes(
    mut generator: Generator,
    struct_with_field_attributes: ModuleItem
) {
    let (generator, _, item) = assert_builder_with_rules(
        &struct_with_field_attributes,
        &mut generator,
        Path::new("Option").with(Path::new("u32")).to_owned(),
        vec![
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_all()
                    .with_field_ident("field_with_attribute_as_path")
                    .then_discard_attribute("attribute_as_path");
            },
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_all()
                    .with_field_ident("field_with_attribute_as_name_value")
                    .then_discard_attribute("attribute_as_name_value = value");
            },
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_all()
                    .with_field_ident("field_with_attribute_as_list")
                    .then_discard_attribute("attribute_as_list(value1, value2)");
            }
        ]
    );
    assert_eq!(3, generator.field_rules.borrow().len());
    let struct_item = item.item;
    for (index, field) in struct_item.fields.iter().enumerate() {
        assert_eq!(
            format!("{}: {}", index, 0), 
            format!("{}: {}", index, field.attrs.len())
        );
    }
}

#[rstest]
fn rename(
    mut generator: Generator,
    struct_with_required_field: ModuleItem
) {
    let (generator, _, item) = assert_builder_with_rules(
        &struct_with_required_field,
        &mut generator,
        Path::new("Option").with(Path::new("u32")).to_owned(),
        vec![
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_all()
                    .with_field_ident("field")
                    .then_rename("renamed_field");
            }
        ]
    );
    assert_eq!(1, generator.field_rules.borrow().len());
    let struct_item = item.item;
    let field = struct_item.fields.iter().last().unwrap();
    assert_eq!("renamed_field", field.ident.as_ref().unwrap().to_string());
}

#[rstest]
fn map_from_primitive_to_primitive(
    mut generator: Generator,
    struct_with_required_field: ModuleItem
) {
    let (generator, item_ident, item) = assert_builder_with_rules(
        &struct_with_required_field,
        &mut generator,
        Path::new("Option").with(Path::new("u64")).to_owned(),
        vec![
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_all()
                    .with_field_ident("field")
                    .then_map(Path::new("u64"));
            }
        ]
    );
    assert_eq!(1, generator.field_rules.borrow().len());
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
            pub fn with_field(&mut self, value: u64) -> &mut Self {
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
                        value.into()
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
fn map_from_primitive_to_vec(
    mut generator: Generator,
    struct_with_required_field: ModuleItem
) {
    let (generator, item_ident, item) = assert_builder_with_rules(
        &struct_with_required_field,
        &mut generator,
        Path::new("Option")
            .with(Path::new("Vec").with(Path::new("u64")).to_owned())
            .to_owned(),
        vec![
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_all()
                    .with_field_ident("field")
                    .then_map_to_vec(Path::new("u64"));
            }
        ]
    );
    assert_eq!(1, generator.field_rules.borrow().len());
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
            pub fn with_field(&mut self, value: u64) -> &mut Self {
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
                        value.into()
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
fn map_from_vec_to_vec(
    mut generator: Generator,
    struct_with_vec_of_primitive_field: ModuleItem
) {
    let (generator, item_ident, item) = assert_builder_with_rules(
        &struct_with_vec_of_primitive_field,
        &mut generator,
        Path::new("Option")
            .with(Path::new("Vec").with(Path::new("u64")).to_owned())
            .to_owned(),
        vec![
            |generator: &mut Generator| {
                generator.with_rule()
                    .for_all()
                    .with_field_ident("field")
                    .then_map_to_vec(Path::new("u64"));
            }
        ]
    );
    assert_eq!(1, generator.field_rules.borrow().len());
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
            pub fn with_field(&mut self, value: u64) -> &mut Self {
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
                        value.iter()
                            .map(|value| value.into())
                            .collect()
                    }
                    else {
                        panic!("field 'field' is required");
                    }
                }
            }
        }
    );
}