use crate::field_type_segment::{FieldTypeSegment, InnerFieldTypeSegment};
use crate::generator::Generator;
use ast_shaper::utils::path::Path;
use ast_shaper::utils::statement::{Expr, ExprMethodChainCall, Statement};
use ast_shaper::utils::{create_generic_type, create_ident};
use std::collections::HashMap;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Block, FnArg, Ident, ImplItem, ImplItemFn, Pat, PatIdent, PatType, Receiver, ReturnType, Signature, Stmt, Type, TypePath, TypeReference, Visibility};

#[derive(Clone)]
pub(crate) struct Field {
    generator: Generator,
    pub item: syn::Field,
    pub ident: String,
    pub ty: FieldTypeSegment,
    pub is_required: bool,
}

impl Field {
    pub(crate) fn new(generator: Generator, item: syn::Field) -> Self {
        let mut field = item.clone();
        field.vis = Visibility::Inherited;
        let field_ident = item.ident.clone().unwrap().to_string();
        let field_type = item.ty;
        let (field_type, is_required) = match field_type {
            Type::Path(value) => {
                let segment = value.path.segments.last().unwrap();
                let segment_ident = segment.ident.to_string();
                let is_required = segment_ident != "Option";
                let segment = Path::from(segment.clone());
                (segment, is_required)
            }
            _ => panic!("Unexpected path type")
        };
        let field_type = FieldTypeSegment::new(&generator.clone(), field_type);
        Self {
            generator,
            item: field,
            ident: field_ident,
            ty: field_type,
            is_required,
        }
    }

    pub fn attributes_mut(&mut self) -> &mut Vec<Attribute> {
        &mut self.item.attrs
    }

    pub fn rename(&mut self, ident: String) {
        self.ident = ident.into();
    }

    pub fn map(&mut self, ty: Path) {
        self.ty = FieldTypeSegment::new(&self.generator.clone(), ty);
    }

    pub(self) fn sanitized_ident(&self) -> String {
        match self.ident.as_str() {
            "r#type" => "type".to_string(),
            _ => self.ident.to_string()
        }
    }

    pub(crate) fn unwrap(&self) -> syn::Field {
        let field_type = create_generic_type(
            "Option",
            vec![self.ty.unwrap()]
        );
        let mut field = self.item.clone();
        field.ident = Some(create_ident(&self.ident));
        field.ty = Type::Path(TypePath {
            qself: None,
            path: field_type.to_syn_path(),
        });
        field
    }

    pub(crate) fn generate_new_method(fields: &Vec<Field>) -> ImplItem {
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
                ident: create_ident("new"),
                generics: Default::default(),
                paren_token: Default::default(),
                inputs: Punctuated::default(),
                variadic: None,
                output: ReturnType::Type(Default::default(), Box::new(Type::Path(TypePath {
                    qself: None,
                    path: Path::new("Self").to_syn_path(),
                })))
            },
            block: Block {
                brace_token: Default::default(),
                stmts: vec![
                    Statement::implicit_return(
                        Expr::Stmt(Statement::struct_literal(
                            Path::new("Self"),
                            fields.iter()
                                .map(|field| {
                                    (
                                        field.ident.clone(),
                                        Expr::Path(Path::new("None"))
                                    )
                                })
                                .collect::<HashMap<String, Expr>>()
                        ))
                    )
                ],
            },
        });
        item
    }

    pub(crate) fn generate_set_method(&self) -> ImplItem {
        let ident = self.sanitized_ident();
        let ident = format!("with_{}", ident);
        let ident = Ident::new(ident.as_str(), ident.span());
        let mut arguments = Punctuated::new();
        arguments.push(FnArg::Receiver(Receiver {
            attrs: vec![],
            reference: Some((Default::default(), None)),
            mutability: Some(Default::default()),
            self_token: Default::default(),
            colon_token: None,
            ty: Box::new(Type::Reference(TypeReference {
                and_token: Default::default(),
                lifetime: None,
                mutability: Some(Default::default()),
                elem: Box::new(Type::Path(TypePath {
                    qself: None,
                    path: Path::new("Self").to_syn_path(),
                })),
            }))
        }));
        self.generate_set_method_arguments().iter()
            .map(|(ident, ty)| {
                FnArg::Typed(PatType {
                    attrs: vec![],
                    pat: Box::new(Pat::Ident(PatIdent {
                        attrs: vec![],
                        by_ref: None,
                        mutability: None,
                        ident: ident.clone(),
                        subpat: None,
                    })),
                    colon_token: Default::default(),
                    ty: Box::new(Type::Path(TypePath {
                        qself: None,
                        path: ty.to_syn_path(),
                    })),
                })
            })
            .for_each(|argument| {
                arguments.push(argument);
            });
        let return_type = match &self.ty.is_complex() {
            true => {
                ReturnType::Type(Default::default(), Box::new(Type::Reference(TypeReference {
                    and_token: Default::default(),
                    lifetime: None,
                    mutability: Some(Default::default()),
                    elem: Box::new(Type::Path(TypePath {
                        qself: None,
                        path: self.ty.unwrap().to_syn_path(),
                    })),
                })))
            }
            false => {
                ReturnType::Type(Default::default(), Box::new(Type::Reference(TypeReference {
                    and_token: Default::default(),
                    lifetime: None,
                    mutability: Some(Default::default()),
                    elem: Box::new(Type::Path(TypePath {
                        qself: None,
                        path: Path::new("Self").to_syn_path(),
                    })),
                })))
            }
        };
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
                ident,
                generics: Default::default(),
                paren_token: Default::default(),
                inputs: arguments,
                variadic: None,
                output: return_type,
            },
            block: Block {
                brace_token: Default::default(),
                stmts: self.generate_set_method_statements(),
            },
        });
        item
    }

    pub(self) fn generate_set_method_arguments(&self) -> Vec<(Ident, Path)> {
        match &self.ty.inner {
            InnerFieldTypeSegment::Complex(_) => vec![],
            InnerFieldTypeSegment::Vec(value) => {
                vec![
                    (
                        create_ident("value"),
                        value.item.unwrap()
                    )
                ]
            }
            InnerFieldTypeSegment::Map(value) => {
                vec![
                    (
                        create_ident("key"),
                        value.key.unwrap()
                    ),
                    (
                        create_ident("value"),
                        value.value.unwrap()
                    )
                ]
            }
            _ => {
                vec![
                    (
                        create_ident("value"),
                        self.ty.unwrap()
                    )
                ]
            }
        }
    }

    pub(self) fn generate_set_method_statements(&self) -> Vec<Stmt> {
        fn chain_call(field: String) -> Vec<ExprMethodChainCall> {
            vec![
                ExprMethodChainCall::Start {
                    receiver: Expr::Stmt(Statement::access_field(
                        Path::new("self"),
                        Path::new(field)
                    )),
                    method: Path::new("as_mut"),
                    arguments: vec![],
                },
                ExprMethodChainCall::Chained {
                    method: Path::new("unwrap"),
                    arguments: vec![],
                }
            ]
        }
        fn extend_chain_call(calls: &Vec<ExprMethodChainCall>, call: ExprMethodChainCall) -> Vec<ExprMethodChainCall> {
            let mut calls = calls.clone();
            calls.push(call);
            calls
        }
        match &self.ty.inner {
            InnerFieldTypeSegment::Complex(_) => {
                vec![
                    Statement::let_none_condition(
                        Expr::Stmt(Statement::access_field(
                            Path::new("self"),
                            Path::new(self.ident.clone())
                        )),
                        vec![
                            Statement::assign_field(
                                Path::new("self"),
                                Path::new(self.ident.clone()),
                                Expr::Stmt(Statement::call(
                                    Path::new("Some"),
                                    vec![
                                        Expr::Stmt(Statement::call(
                                            self.ty.unwrap().join("new").clone(),
                                            vec![]
                                        ))
                                    ]
                                ))
                            )
                        ]
                    ),
                    Statement::implicit_return(
                        Expr::Stmt(Statement::method_chain_call(chain_call(self.ident.clone())))
                    )
                ]
            }
            InnerFieldTypeSegment::Vec(_) => {
                vec![
                    Statement::let_none_condition(
                        Expr::Stmt(Statement::access_field(
                            Path::new("self"),
                            Path::new(self.ident.clone())
                        )),
                        vec![
                            Statement::assign_field(
                                Path::new("self"),
                                Path::new(self.ident.clone()),
                                Expr::Stmt(Statement::call(
                                    Path::new("Some"),
                                    vec![
                                        Expr::Stmt(Statement::call(
                                            Path::new("Vec").join("new").clone(),
                                            vec![]
                                        ))
                                    ]
                                ))
                            )
                        ]
                    ),
                    Statement::method_chain_call(extend_chain_call(
                        &chain_call(self.ident.clone()),
                        ExprMethodChainCall::Chained {
                            method: Path::new("push"),
                            arguments: vec![
                                Expr::Path(Path::new("value"))
                            ],
                        }
                    )),
                    Statement::implicit_return(Expr::Path(Path::new("self")))
                ]
            }
            InnerFieldTypeSegment::Map(_) => {
                vec![
                    Statement::let_none_condition(
                        Expr::Stmt(Statement::access_field(
                            Path::new("self"),
                            Path::new(self.ident.clone())
                        )),
                        vec![
                            Statement::assign_field(
                                Path::new("self"),
                                Path::new(self.ident.clone()),
                                Expr::Stmt(Statement::call(
                                    Path::new("Some"),
                                    vec![
                                        Expr::Stmt(Statement::call(
                                            Path::new("HashMap").join("new").clone(),
                                            vec![]
                                        ))
                                    ]
                                ))
                            )
                        ]
                    ),
                    Statement::method_chain_call(extend_chain_call(
                        &chain_call(self.ident.clone()),
                        ExprMethodChainCall::Chained {
                            method: Path::new("insert"),
                            arguments: vec![
                                Expr::Path(Path::new("key")),
                                Expr::Path(Path::new("value")),
                            ]
                        }
                    )),
                    Statement::implicit_return(Expr::Path(Path::new("self")))
                ]
            }
            _ => {
                vec![
                    Statement::assign_field(
                        Path::new("self"),
                        Path::new(self.ident.clone()),
                        Expr::Stmt(Statement::call(
                            Path::new("Some").clone(),
                            vec![
                                Expr::Path(Path::new("value"))
                            ]
                        ))
                    ),
                    Statement::implicit_return(Expr::Path(Path::new("self")))
                ]
            }
        }
    }

    pub(crate) fn generate_build_method_statement(&self) -> Stmt {
        let call = match self.ty.inner {
            InnerFieldTypeSegment::Complex(_) => {
                Expr::Stmt(Statement::method_call(
                    Expr::Path(Path::new("value")),
                    Path::new("build"),
                    vec![]
                ))
            }
            _ => {
                Expr::Path(Path::new("value"))
            }
        };
        Statement::let_some_condition(
            Expr::Stmt(Statement::method_call(
                Expr::Stmt(Statement::access_field(
                    Path::new("self"),
                    Path::new(self.ident.clone())
                )),
                Path::new("clone"),
                vec![]
            )),
            Path::new("value"),
            vec![Statement::implicit_return(
                match self.is_required {
                    true => {
                        call
                    }
                    false => {
                        Expr::Stmt(Statement::call(
                            Path::new("Some").clone(),
                            vec![
                                call
                            ]
                        ))
                    }
                }
            )],
            match self.is_required {
                true => {
                    Some(Expr::Stmt(Statement::panic(
                        format!("field '{}' is required", self.ident),
                        vec![]
                    )))
                }
                false => {
                    Some(Expr::Stmt(Statement::implicit_return(
                        Expr::Stmt(Statement::block(vec![
                            Statement::without_trailling_semi_colon(
                                Statement::path(Path::new("None"))
                            )
                        ]))
                    )))
                }
            }
        )
    }
}