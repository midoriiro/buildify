use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use itertools::Itertools;
use syn::{Attribute, Block, Fields, FieldsNamed, FnArg, Generics, Ident, ImplItem, ImplItemFn, ItemImpl, ItemStruct, Receiver, ReturnType, Signature, Token, Type, TypePath, TypeReference, Visibility};
use syn::punctuated::Punctuated;
use ast_shaper::items::item::{Item, ItemTrait};
use ast_shaper::items::module_item::ModuleItems;
use ast_shaper::items::struct_item::StructItem;
use ast_shaper::utils::create_ident;
use ast_shaper::utils::path::Path;
use ast_shaper::utils::punctuated::PunctuatedExt;
use ast_shaper::utils::statement::{Expr, Statement};
use crate::field::Field;
use crate::field_rule::{FieldRule, FieldRuleItemSelectorBuilder};

pub struct Generator {
    modules: Rc<RefCell<Vec<ModuleItems>>>,
    field_rules: Rc<RefCell<Vec<FieldRule>>>
}

impl Generator {
    pub fn new(modules: Rc<RefCell<Vec<ModuleItems>>>) -> Self {
        Self {
            modules,
            field_rules: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn with_rule(&mut self) -> FieldRuleItemSelectorBuilder
    {
        FieldRuleItemSelectorBuilder::new(self.field_rules.clone())
    }
    
    pub(crate) fn ident(ident: &Ident) -> Ident {
        let item_ident = format!("{}Builder", ident.to_string());
        create_ident(&item_ident)
    }
    
    pub(crate) fn find_item(&self, ident: &String) -> Option<Item> {
        self.modules.borrow().iter()
            .find_map(|module| {
                let item = module.find_item_by(|item| {
                    item.ident() == *ident
                });
                match item {
                    Some(value) => Some(value.clone()),
                    None => None
                }
            })
    } 

    pub fn generate(&mut self, item: &syn::Item) -> Vec<StructItem> {
        let attributes = match &item {
            syn::Item::Struct(value) => &value.attrs,
            syn::Item::Enum(value) => &value.attrs,
            _ => panic!("Unexpected item type")
        };
        let generics = match &item {
            syn::Item::Struct(value) => &value.generics,
            syn::Item::Enum(value) => &value.generics,
            _ => panic!("Unexpected item type")
        };
        let ident = match &item {
            syn::Item::Struct(value) => &value.ident,
            syn::Item::Enum(value) => &value.ident,
            _ => panic!("Unexpected item type")
        };
        let fields = self.generate_fields(&item);
        let struct_item = self.generate_struct_item(
            attributes.clone(),
            Visibility::Public(Default::default()),
            generics.clone(),
            ident.clone(),
            &fields
        );
        let struct_impl_item = self.generate_struct_impl_item(
            generics.clone(),
            ident.clone(),
            &fields
        );
        let mut builders = vec![
            StructItem::new(struct_item, vec![struct_impl_item])
        ];
        let mut inner_builders = self.generate_inner_builders(
            attributes.clone(),
            Visibility::Public(Default::default()),
            generics.clone(),
            &fields
        );
        builders.append(&mut inner_builders);
        builders
    }

    pub(crate) fn generate_fields(&self, item: &syn::Item) -> Vec<Field> {
        let ident = match item {
            syn::Item::Enum(value) => {
                value.ident.to_string()
            }
            syn::Item::Struct(value) => {
                value.ident.to_string()
            }
            _ => panic!("Expected struct or enum item")
        };
        let mut fields: Vec<&syn::Field> = match &item {
            syn::Item::Struct(value) => {
                match &value.fields {
                    Fields::Named(value) => value.named.iter().collect(),
                    Fields::Unnamed(value) => value.unnamed.iter().collect(),
                    _ => panic!("Unexpected fields type")
                }
            }
            syn::Item::Enum(value) => {
                value.variants.iter()
                    .filter_map(|variant| {
                        match &variant.fields {
                            Fields::Named(value) => Some(value.named.iter().collect::<Vec<_>>()),
                            Fields::Unnamed(value) => Some(value.unnamed.iter().collect::<Vec<_>>()),
                            _ => None
                        }
                    })
                    .flatten()
                    .collect()
            }
            _ => panic!("Expected struct or enum item")
        };
        fields.iter_mut()
            .map(|field| {
                let field = field.clone();
                let mut field = Field::new(self.clone(), field);
                self.field_rules.borrow().iter()
                    .for_each(|rule: &FieldRule| {
                        rule.apply(&ident, &field.ident.clone(), &mut field);
                    });
                field
            })
            .collect()
    }
    
    fn generate_struct_item(
        &self,
        attributes: Vec<Attribute>,
        visibility: Visibility,
        generics: Generics,
        ident: Ident,
        fields: &Vec<Field>
    ) -> ItemStruct {
        ItemStruct {
            attrs: attributes,
            vis: visibility,
            struct_token: Default::default(),
            ident: Self::ident(&ident),
            generics,
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: fields.iter()
                    .map(|field| field.decompose())
                    .collect::<Punctuated<syn::Field, Token![,]>>(),
            }),
            semi_token: Default::default(),
        }
    }
    
    fn generate_struct_impl_item(
        &self,
        generics: Generics,
        ident: Ident,
        fields: &Vec<Field>
    ) -> ItemImpl {
        let mut methods = vec![
            Field::generate_new_method(fields)
        ];
        let mut set_methods = fields.iter()
            .map(|field| field.generate_set_method())
            .sorted_by(|a, b| {
                let a = match a {
                    ImplItem::Fn(value) => value,
                    _ => panic!("Unexpected item")
                };
                let b = match b {
                    ImplItem::Fn(value) => value,
                    _ => panic!("Unexpected item")
                };
                a.sig.ident.to_string().cmp(&b.sig.ident.to_string())
            })
            .collect();
        methods.append(&mut set_methods);
        methods.push(Self::generate_build_method(&ident, fields));
        ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics,
            trait_: None,
            self_ty: Box::new(Type::Path(TypePath {
                qself: None,
                path: Path::new(&Self::ident(&ident).to_string()).to_syn_path(),
            })),
            brace_token: Default::default(),
            items: methods,
        }
    }

    fn generate_inner_builders(
        &self,
        attributes: Vec<Attribute>,
        visibility: Visibility,
        generics: Generics,
        fields: &Vec<Field>
    ) -> Vec<StructItem> {
        fn generate_builder(
            generator: &Generator,
            attributes: Vec<Attribute>,
            visibility: Visibility,
            generics: Generics,
            ident: Ident,
            fields: &Vec<Field>
        ) -> Vec<StructItem> {
            let struct_item = generator.generate_struct_item(
                attributes.clone(),
                visibility.clone(),
                generics.clone(),
                ident.clone(),
                fields
            );
            let struct_impl_item = generator.generate_struct_impl_item(
                generics.clone(),
                ident.clone(),
                fields
            );
            let mut builders = vec![
                StructItem::new(struct_item, vec![struct_impl_item])
            ];
            let mut inner_builders = fields.iter()
                .filter_map(|inner_field| {
                    match &inner_field.ty.inner_fields {
                        Some(value) => {
                            Some(generate_builder(
                                generator,
                                attributes.clone(),
                                visibility.clone(),
                                generics.clone(),
                                inner_field.ty.ty.last().unwrap().ident.clone(),
                                &value))
                        }
                        None => None
                    }
                })
                .flatten()
                .collect::<Vec<_>>();
            builders.append(&mut inner_builders);
            builders
        }
        let builders = fields.iter()
            .filter_map(|field| {
                match field.ty.is_complex {
                    true => {}
                    false => return None
                }
                match &field.ty.inner_fields {
                    Some(value) => {
                        let builders = generate_builder(
                            self,
                            attributes.clone(),
                            visibility.clone(),
                            generics.clone(),
                            field.ty.ty.last().unwrap().ident.clone(),
                            value
                        );
                        Some(builders)
                    },
                    None => None
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        builders
    }
    
    fn generate_build_method(
        return_type: &Ident, 
        fields: &Vec<Field>
    ) -> ImplItem {
        let return_type = Path::from(return_type.clone());
        let mut statements = Vec::new();
        let fields_init = fields.iter()
            .map(|field| {
                (field.ident.clone(), Expr::Stmt(field.generate_build_method_statement()))
            })
            .collect::<HashMap<_, _>>();
        statements.push(Statement::implicit_return(
            Expr::Stmt(Statement::struct_literal(Path::new(return_type.to_string()), fields_init))
        ));
        let item = ImplItem::Fn(ImplItemFn {
            attrs: vec![],
            vis: Visibility::Public(Default::default()),
            defaultness: None,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: create_ident("build"),
                generics: Default::default(),
                paren_token: Default::default(),
                inputs: Punctuated::single(FnArg::Receiver(Receiver {
                    attrs: vec![],
                    reference: Some((Default::default(), None)),
                    mutability: None,
                    self_token: Default::default(),
                    colon_token: None,
                    ty: Box::new(Type::Reference(TypeReference {
                        and_token: Default::default(),
                        lifetime: None,
                        mutability: None,
                        elem: Box::new(Type::Path(TypePath {
                            qself: None,
                            path: Path::new("Self").to_syn_path(),
                        })),
                    })),
                })),
                variadic: None,
                output: ReturnType::Type(Default::default(), Box::new(Type::Path(TypePath {
                    qself: None,
                    path: Path::from(return_type.to_string()).to_syn_path(),
                }))),
            },
            block: Block {
                brace_token: Default::default(),
                stmts: statements,
            },
        });
        item
    }
}

impl Clone for Generator {
    fn clone(&self) -> Self {
        Self {
            modules: self.modules.clone(),
            field_rules: self.field_rules.clone(),
        }
    }
}