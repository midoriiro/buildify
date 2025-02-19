use crate::generator::Generator;
use ast_shaper::items::fn_item::FnItem;
use ast_shaper::items::item::ItemTrait;
use ast_shaper::items::module_item::ModuleItem;
use ast_shaper::items::struct_item::StructItem;
use ast_shaper::utils::path::Path;
use pretty_assertions::assert_eq;
use quote::__private::TokenStream;
use syn::{parse2, Fields, ImplItemFn, Type};

fn internal_assert_builder(
    item_ident: String,
    item: syn::Item,
    generator: &Generator,
    expected_field_type: Path,
) -> (TokenStream, StructItem) {
    let item_ident_quote = item_ident.parse::<TokenStream>().unwrap();
    let items = generator.generate(&item);
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
    (item_ident_quote, item.clone())
}

pub fn assert_builder_with_rules(
    module: &ModuleItem,
    generator: &mut Generator,
    expected_field_type: Path,
    mut rules: Vec<fn(&mut Generator)>
) -> (Generator, TokenStream, StructItem) {
    let item = module.items.first().unwrap();
    let item_ident = item.ident();
    for rule in rules.iter_mut() {
        rule(generator);
    }
    let (item_ident, item) = internal_assert_builder(
        item_ident,
        item.to_syn_item(),
        &generator,
        expected_field_type,
    );
    (generator.clone(), item_ident, item)
}

pub fn assert_builder(
    module: &ModuleItem,
    generator: &Generator,
    expected_field_type: Path
) -> (TokenStream, StructItem) {
    let item = module.items.first().unwrap();
    let item_ident = item.ident();
    internal_assert_builder(
        item_ident,
        item.to_syn_item(),
        &generator,
        expected_field_type,
    )
}

pub fn assert_method(functions: &Vec<FnItem>, expected_method: TokenStream) {
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