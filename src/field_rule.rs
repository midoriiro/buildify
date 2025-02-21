use crate::field::Field;
use ast_shaper::utils::create_generic_type;
use ast_shaper::utils::path::Path;
use quote::ToTokens;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use syn::{parse_str, Meta};

pub struct FieldRuleItemSelectorBuilder {
    rules: Rc<RefCell<Vec<FieldRule>>>
}

impl FieldRuleItemSelectorBuilder {
    pub(crate) fn new(rules: Rc<RefCell<Vec<FieldRule>>>) -> Self {
        Self {
            rules,
        }
    }

    pub fn for_all(&self) -> FieldRuleFieldSelectorBuilder {
        FieldRuleFieldSelectorBuilder::new(self.rules.clone(), None)
    }

    pub fn for_item(&self, ident: impl Into<String>) -> FieldRuleFieldSelectorBuilder {
        FieldRuleFieldSelectorBuilder::new(self.rules.clone(), Some(ident.into()))
    }
}

pub struct FieldRuleFieldSelectorBuilder {
    rules: Rc<RefCell<Vec<FieldRule>>>,
    item_ident: Option<String>
}

impl FieldRuleFieldSelectorBuilder {
    pub(self) fn new(
        rules: Rc<RefCell<Vec<FieldRule>>>,
        item_ident: Option<String>
    ) -> Self {
        Self {
            rules,
            item_ident,
        }
    }

    pub fn and_all_fields(&mut self) -> FieldRuleThenSelectorBuilder {
        FieldRuleThenSelectorBuilder::new(
            self.rules.clone(), 
            self.item_ident.clone(), 
            None,
            None
        )
    }

    pub fn with_field_ident(&mut self, ident: impl Into<String>) -> FieldRuleThenSelectorBuilder {
        FieldRuleThenSelectorBuilder::new(
            self.rules.clone(), 
            self.item_ident.clone(), 
            Some(ident.into()),
            None
        )
    }

    pub fn with_field_type(&mut self, ty: impl Into<String>) -> FieldRuleThenSelectorBuilder {
        FieldRuleThenSelectorBuilder::new(
            self.rules.clone(), 
            self.item_ident.clone(), 
            None,
            Some(Path::new(ty.into()))
        )
    }
}

pub struct FieldRuleThenSelectorBuilder {
    rules: Rc<RefCell<Vec<FieldRule>>>,
    item_ident: Option<String>,
    field_ident: Option<String>,
    field_type: Option<Path>,
}

impl FieldRuleThenSelectorBuilder {
    pub(self) fn new(
        rules: Rc<RefCell<Vec<FieldRule>>>,
        item_ident: Option<String>,
        field_ident: Option<String>,
        field_type: Option<Path>
    ) -> Self {
        Self {
            rules,
            item_ident,
            field_ident,
            field_type,
        }
    }

    pub(self) fn then<F>(&mut self, rule: F) -> &mut Self
    where
        F: Fn(&mut Field) + 'static
    {
        let rule = FieldRule::new(
            self.item_ident.clone(),
            self.field_ident.clone(),
            self.field_type.clone(),
            rule,
        );
        self.rules.borrow_mut().push(rule);
        self
    }

    pub fn then_map(&mut self, ty: Path) -> &mut Self {
        if self.field_ident.is_none() && self.field_type.is_none() {
            panic!("Cannot remap field when field selector target all field");
        }
        self.then(move |field| field.map(ty.clone()))
    }

    pub fn then_map_to_vec(&mut self, ty: Path) -> &mut Self {
        if self.field_ident.is_none() && self.field_type.is_none() {
            panic!("Cannot remap field when field selector target all field");
        }
        self.then(move |field| {
            field.map(create_generic_type("Vec", vec![ty.clone()]))
        })
    }

    pub fn then_rename(&mut self, ident: impl Into<String>) -> &mut Self {
        if self.field_ident.is_none() && self.field_type.is_none() {
            panic!("Cannot rename field when field selector target all field");
        }
        let ident = ident.into();
        self.then(move |field| field.rename(ident.clone()))
    }

    pub fn then_discard_attribute(&mut self, attribute: impl Into<String>) -> &mut Self {
        let attribute_to_compare = attribute.into();
        self.then(move |field| {
            field.attributes_mut().retain_mut(|attribute| {
                match &attribute.meta {
                    Meta::Path(value) => {
                        let attribute_to_compare = parse_str(attribute_to_compare.as_str())
                            .map(Meta::Path);
                        if attribute_to_compare.is_err() {
                            return true;
                        }
                        let attribute_to_compare = attribute_to_compare
                            .as_ref()
                            .unwrap()
                            .require_path_only()
                            .unwrap();
                        value != attribute_to_compare
                    },
                    Meta::NameValue(value) => {
                        let attribute_to_compare = parse_str(attribute_to_compare.as_str())
                            .map(Meta::NameValue);
                        if attribute_to_compare.is_err() {
                            return true;
                        }
                        let attribute_to_compare = attribute_to_compare
                            .as_ref()
                            .unwrap()
                            .require_name_value()
                            .unwrap();
                        value != attribute_to_compare
                    },
                    Meta::List(value) => {
                        let attribute_to_compare = parse_str(attribute_to_compare.as_str())
                            .map(Meta::List);
                        if attribute_to_compare.is_err() {
                            return true;
                        }
                        let attribute_to_compare = attribute_to_compare
                            .as_ref()
                            .unwrap()
                            .require_list()
                            .unwrap();
                        let l = attribute_to_compare.to_token_stream().to_string();
                        let l = value.to_token_stream().to_string();
                        value != attribute_to_compare
                    }
                }
            });
        })
    }
}

pub struct FieldRule {
    item_ident: Option<String>,
    field_ident: Option<String>,
    field_type: Option<Path>,
    rule: Box<dyn Fn(&mut Field) + 'static>
}

impl FieldRule {
    pub(self) fn new(
        item_ident: Option<String>,
        field_ident: Option<String>,
        field_type: Option<Path>,
        rule: impl Fn(&mut Field) + 'static

    ) -> Self {
        Self {
            item_ident,
            field_ident,
            field_type,
            rule: Box::new(rule),
        }
    }

    pub(crate) fn apply(&self, item_ident: &String, field: &mut Field) {
        match self.item_ident.as_ref() {
            Some(value) => {
                if value != item_ident {
                    return
                }
            }
            None => {}
        }
        match self.field_ident.as_ref() {
            Some(value) => {
                if value != &field.ident {
                    return
                }
            }
            None => {}
        }
        match self.field_type.as_ref() {
            Some(value) => {
            if value != &field.ty.unwrap() {
                
            }
            }
            None => {}
        }
        (self.rule)(field)
    }
}

impl Debug for FieldRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FieldRule")
            .field("item_ident", &self.item_ident)
            .field("field_ident", &self.field_ident)
            .field("field_type", &self.field_type)
            .field("rule", &"Not debuggable")
            .finish()
    }
}