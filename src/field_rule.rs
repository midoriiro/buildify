use crate::field::Field;
use ast_shaper::utils::create_generic_type;
use ast_shaper::utils::path::Path;
use std::cell::RefCell;
use std::rc::Rc;
use syn::Meta;

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
        FieldRuleThenSelectorBuilder::new(self.rules.clone(), self.item_ident.clone(), None)
    }

    pub fn with_field(&mut self, ident: impl Into<String>) -> FieldRuleThenSelectorBuilder {
        FieldRuleThenSelectorBuilder::new(self.rules.clone(), self.item_ident.clone(), Some(ident.into()))
    }
}

pub struct FieldRuleThenSelectorBuilder {
    rules: Rc<RefCell<Vec<FieldRule>>>,
    item_ident: Option<String>,
    field_ident: Option<String>
}

impl FieldRuleThenSelectorBuilder {
    pub(self) fn new(
        rules: Rc<RefCell<Vec<FieldRule>>>,
        item_ident: Option<String>,
        field_ident: Option<String>
    ) -> Self {
        Self {
            rules,
            item_ident,
            field_ident
        }
    }

    pub(self) fn then<F>(&mut self, rule: F) -> &mut Self
    where
        F: Fn(&mut Field) + 'static
    {
        let rule = FieldRule::new(
            self.item_ident.clone(),
            self.field_ident.clone(),
            rule,
        );
        self.rules.borrow_mut().push(rule);
        self
    }

    pub fn then_map(&mut self, ty: Path) -> &mut Self {
        self.then(move |field| field.map(ty.clone()))
    }

    pub fn then_remap_to_vec(&mut self, ty: Path) -> &mut Self {
        self.then(move |field| field.map(create_generic_type("Vec", vec![ty.clone()])))
    }

    pub fn then_rename(&mut self, ident: impl Into<String>) -> &mut Self {
        let ident = ident.into();
        self.then(move |field| field.rename(ident.clone()))
    }

    pub fn then_discard_attribute(&mut self, ident: impl Into<String>) -> &mut Self {
        let ident = ident.into();
        self.then(move |field| {
            fn predicate(path: &Path, ident: &String) -> bool {
                let segment = path.last().unwrap().ident.to_string();
                match *ident == segment {
                    true => false,
                    false => true
                }
            }
            field.attributes_mut().retain_mut(|attribute| {
                match &attribute.meta {
                    Meta::List(value) => {
                        let path = Path::from(&value.path);
                        predicate(&path, &ident)
                    }
                    Meta::Path(value) => {
                        let path = Path::from(value);
                        predicate(&path, &ident)
                    },
                    Meta::NameValue(value) => {
                        let path = Path::from(&value.path);
                        predicate(&path, &ident)
                    },
                }
            });
        })
    }
}

pub struct FieldRule {
    item_ident: Option<String>,
    field_ident: Option<String>,
    rule: Box<dyn Fn(&mut Field) + 'static>
}

impl FieldRule {
    pub(self) fn new(
        item_ident: Option<String>,
        field_ident: Option<String>,
        rule: impl Fn(&mut Field) + 'static

    ) -> Self {
        Self {
            item_ident,
            field_ident,
            rule: Box::new(rule),
        }
    }

    pub(crate) fn apply(&self, item_ident: &String, field_ident: &String, field: &mut Field) {
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
                if value != field_ident {
                    return
                }
            }
            None => {}
        }
        (self.rule)(field)
    }
}