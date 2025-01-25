#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use analyzer::TypeAnalyzer;
    use biome_js_parser::parse;
    use biome_js_syntax::JsFileSource;
    use symbol::Symbol;
    use type_info::TypeInfo;
    use type_info::*;
    use visitor::Visitor;

    fn test_analyzer(src: &str, src_type: JsFileSource) -> TypeAnalyzer {
        let parsed = parse(src, src_type, Default::default());
        if parsed.has_errors() {
            panic!("Failed to parse source code: {:?}", parsed.diagnostics());
        }
        let root = parsed.tree();
        let mut analyzer = TypeAnalyzer::new(vec![]);
        analyzer.visit(&root);
        analyzer
    }

    #[test]
    fn test_keyword_types() {
        let src = r#"declare const a: number;
declare const b: string;
declare const c: boolean;
declare const d: bigint;
declare const e: symbol;
declare const f: null;
declare const g: undefined;
declare const h: never;
declare const i: void;
declare const j: any;
        "#;

        let analyzer = test_analyzer(src, JsFileSource::ts());
        assert_eq!(
            analyzer.get_symbol("a").unwrap(),
            &Symbol::new(
                "a".to_string(),
                TypeInfo::KeywordType(TsKeywordTypeKind::Number)
            )
        );

        assert_eq!(
            analyzer.get_symbol("b").unwrap(),
            &Symbol::new(
                "b".to_string(),
                TypeInfo::KeywordType(TsKeywordTypeKind::String)
            )
        );

        assert_eq!(
            analyzer.get_symbol("c").unwrap(),
            &Symbol::new(
                "c".to_string(),
                TypeInfo::KeywordType(TsKeywordTypeKind::Boolean)
            )
        );

        assert_eq!(
            analyzer.get_symbol("d").unwrap(),
            &Symbol::new(
                "d".to_string(),
                TypeInfo::KeywordType(TsKeywordTypeKind::BigInt)
            )
        );

        assert_eq!(
            analyzer.get_symbol("e").unwrap(),
            &Symbol::new(
                "e".to_string(),
                TypeInfo::KeywordType(TsKeywordTypeKind::Symbol)
            )
        );

        assert_eq!(
            analyzer.get_symbol("f").unwrap(),
            &Symbol::new(
                "f".to_string(),
                TypeInfo::KeywordType(TsKeywordTypeKind::Null)
            )
        );

        assert_eq!(
            analyzer.get_symbol("g").unwrap(),
            &Symbol::new(
                "g".to_string(),
                TypeInfo::KeywordType(TsKeywordTypeKind::Undefined)
            )
        );

        assert_eq!(
            analyzer.get_symbol("h").unwrap(),
            &Symbol::new(
                "h".to_string(),
                TypeInfo::KeywordType(TsKeywordTypeKind::Never)
            )
        );

        assert_eq!(
            analyzer.get_symbol("i").unwrap(),
            &Symbol::new(
                "i".to_string(),
                TypeInfo::KeywordType(TsKeywordTypeKind::Void)
            )
        );

        assert_eq!(
            analyzer.get_symbol("j").unwrap(),
            &Symbol::new(
                "j".to_string(),
                TypeInfo::KeywordType(TsKeywordTypeKind::Any)
            )
        );
    }

    #[test]
    fn test_object_literal() {
        let src = r#"
        const obj = {
            num: 42,
            str: 'hello',
            bool: true,
        };
    "#;

        let analyzer = test_analyzer(src, JsFileSource::ts());

        assert_eq!(
            analyzer.get_symbol("obj").unwrap(),
            &Symbol::new(
                "obj".to_string(),
                TypeInfo::Literal(TsLiteralTypeKind::Object(ObjectLiteral {
                    properties: vec![
                        ObjectPropertyType {
                            name: "num".to_string(),
                            type_info: TypeInfo::Literal(TsLiteralTypeKind::Number(42))
                        },
                        ObjectPropertyType {
                            name: "str".to_string(),
                            type_info: TypeInfo::Literal(TsLiteralTypeKind::String(
                                "hello".to_string()
                            ))
                        },
                        ObjectPropertyType {
                            name: "bool".to_string(),
                            type_info: TypeInfo::Literal(TsLiteralTypeKind::Boolean(
                                BoolLiteral::True
                            ))
                        }
                    ]
                }))
            )
        )
    }

    #[test]
    fn test_literal_types() {
        let src = r#"declare const a: 1;
declare const b: 'hello';
declare const c: true;
declare const d: false;
const e = 1;
const f = 'hello';
const g = true;
const h = false;
        "#;

        let analyzer = test_analyzer(src, JsFileSource::ts());
        assert_eq!(
            analyzer.get_symbol("a").unwrap(),
            &Symbol::new(
                "a".to_string(),
                TypeInfo::Literal(TsLiteralTypeKind::Number(1))
            )
        );

        assert_eq!(
            analyzer.get_symbol("b").unwrap(),
            &Symbol::new(
                "b".to_string(),
                TypeInfo::Literal(TsLiteralTypeKind::String("'hello'".to_string()))
            )
        );

        assert_eq!(
            analyzer.get_symbol("c").unwrap(),
            &Symbol::new(
                "c".to_string(),
                TypeInfo::Literal(TsLiteralTypeKind::Boolean(BoolLiteral::True))
            )
        );

        assert_eq!(
            analyzer.get_symbol("d").unwrap(),
            &Symbol::new(
                "d".to_string(),
                TypeInfo::Literal(TsLiteralTypeKind::Boolean(BoolLiteral::False))
            )
        );

        assert_eq!(
            analyzer.get_symbol("e").unwrap(),
            &Symbol::new(
                "e".to_string(),
                TypeInfo::Literal(TsLiteralTypeKind::Number(1))
            )
        );

        assert_eq!(
            analyzer.get_symbol("f").unwrap(),
            &Symbol::new(
                "f".to_string(),
                TypeInfo::Literal(TsLiteralTypeKind::String("hello".to_string()))
            )
        );

        assert_eq!(
            analyzer.get_symbol("g").unwrap(),
            &Symbol::new(
                "g".to_string(),
                TypeInfo::Literal(TsLiteralTypeKind::Boolean(BoolLiteral::True))
            )
        );

        assert_eq!(
            analyzer.get_symbol("h").unwrap(),
            &Symbol::new(
                "h".to_string(),
                TypeInfo::Literal(TsLiteralTypeKind::Boolean(BoolLiteral::False))
            )
        );
    }

    #[test]
    fn test_interface() {
        let src = r#"
        interface Person {
            name: string;
            age: number;
            foo?: string;
            readonly bar: boolean;
        }

        interface MethodSignature {
          basic(): void;
          withParams(x: number, y: string): boolean;
          optional?(): void;
          generic<T>(value: T): T;
          complex(): string | Promise<number>;
        }
        "#;

        let analyzer = test_analyzer(src, JsFileSource::ts());

        assert_eq!(
            analyzer.get_symbol("Person").unwrap(),
            &Symbol::new(
                "Person".to_string(),
                TypeInfo::Interface(TsInterface {
                    name: "Person".to_string(),
                    extends: vec![],
                    type_params: vec![],
                    properties: vec![
                        TsInterfaceProperty {
                            name: "name".to_string(),
                            type_info: TypeInfo::KeywordType(TsKeywordTypeKind::String),
                            is_optional: false,
                            is_readonly: false,
                        },
                        TsInterfaceProperty {
                            name: "age".to_string(),
                            type_info: TypeInfo::KeywordType(TsKeywordTypeKind::Number),
                            is_optional: false,
                            is_readonly: false,
                        },
                        TsInterfaceProperty {
                            name: "foo".to_string(),
                            type_info: TypeInfo::KeywordType(TsKeywordTypeKind::String),
                            is_optional: true,
                            is_readonly: false,
                        },
                        TsInterfaceProperty {
                            name: "bar".to_string(),
                            type_info: TypeInfo::KeywordType(TsKeywordTypeKind::Boolean),
                            is_optional: false,
                            is_readonly: true,
                        }
                    ]
                })
            )
        );

        assert_eq!(
            analyzer.get_symbol("MethodSignature").unwrap(),
            &Symbol::new(
                "MethodSignature".to_string(),
                TypeInfo::Interface(TsInterface {
                    name: "MethodSignature".to_string(),
                    extends: vec![],
                    type_params: vec![],
                    properties: vec![
                        TsInterfaceProperty {
                            name: "basic".to_string(),
                            type_info: TypeInfo::Function(TsFunctionSignature {
                                params: vec![],
                                return_type: Box::new(TypeInfo::KeywordType(
                                    TsKeywordTypeKind::Void
                                )),
                                type_params: vec![],
                                this_param: None,
                            }),
                            is_optional: false,
                            is_readonly: false,
                        },
                        TsInterfaceProperty {
                            name: "withParams".to_string(),
                            type_info: TypeInfo::Function(TsFunctionSignature {
                                params: vec![
                                    FunctionParam {
                                        name: "x".to_string(),
                                        param_type: TypeInfo::KeywordType(
                                            TsKeywordTypeKind::Number
                                        ),
                                        is_optional: false
                                    },
                                    FunctionParam {
                                        name: "y".to_string(),
                                        param_type: TypeInfo::KeywordType(
                                            TsKeywordTypeKind::String
                                        ),
                                        is_optional: false
                                    }
                                ],
                                return_type: Box::new(TypeInfo::KeywordType(
                                    TsKeywordTypeKind::Boolean
                                )),
                                type_params: vec![],
                                this_param: None,
                            }),
                            is_optional: false,
                            is_readonly: false,
                        },
                        TsInterfaceProperty {
                            name: "optional".to_string(),
                            type_info: TypeInfo::Function(TsFunctionSignature {
                                params: vec![],
                                return_type: Box::new(TypeInfo::KeywordType(
                                    TsKeywordTypeKind::Void
                                )),
                                type_params: vec![],
                                this_param: None,
                            }),
                            is_optional: true,
                            is_readonly: false,
                        },
                        TsInterfaceProperty {
                            name: "generic".to_string(),
                            type_info: TypeInfo::Function(TsFunctionSignature {
                                params: vec![FunctionParam {
                                    name: "value".to_string(),
                                    param_type: TypeInfo::TypeRef(TsTypeRef {
                                        name: "T".to_string(),
                                        type_params: vec![]
                                    }),
                                    is_optional: false
                                }],
                                return_type: Box::new(TypeInfo::TypeRef(TsTypeRef {
                                    name: "T".to_string(),
                                    type_params: vec![]
                                })),
                                type_params: vec![TypeParam {
                                    name: "T".to_string(),
                                    constraint: None,
                                    default: None
                                }],

                                this_param: None,
                            }),
                            is_optional: false,
                            is_readonly: false,
                        },
                        TsInterfaceProperty {
                            name: "complex".to_string(),
                            type_info: TypeInfo::Function(TsFunctionSignature {
                                params: vec![],
                                return_type: Box::new(TypeInfo::Union(vec![
                                    TypeInfo::KeywordType(TsKeywordTypeKind::String),
                                    TypeInfo::TypeRef(TsTypeRef {
                                        name: "Promise".to_string(),
                                        type_params: vec![TypeInfo::KeywordType(
                                            TsKeywordTypeKind::Number
                                        )]
                                    })
                                ])),
                                type_params: vec![],
                                this_param: None,
                            }),
                            is_optional: false,
                            is_readonly: false,
                        }
                    ]
                })
            )
        );
    }

    #[test]
    fn test_interface_with_generics() {
        let src = r#"
        interface Box<T> {
            value: T;
        }

        interface Pair<T = string, U extends number> {
          first: T;
          second: U;
        }
        "#;

        let analyzer = test_analyzer(src, JsFileSource::ts());

        assert_eq!(
            analyzer.get_symbol("Box").unwrap(),
            &Symbol::new(
                "Box".to_string(),
                TypeInfo::Interface(TsInterface {
                    name: "Box".to_string(),
                    extends: vec![],
                    type_params: vec![TypeParam {
                        name: "T".to_string(),
                        constraint: None,
                        default: None,
                    }],
                    properties: vec![TsInterfaceProperty {
                        name: "value".to_string(),
                        type_info: TypeInfo::TypeRef(TsTypeRef {
                            name: "T".to_string(),
                            type_params: vec![]
                        }),
                        is_optional: false,
                        is_readonly: false,
                    }]
                })
            )
        );

        assert_eq!(
            analyzer.get_symbol("Pair").unwrap(),
            &Symbol::new(
                "Pair".to_string(),
                TypeInfo::Interface(TsInterface {
                    name: "Pair".to_string(),
                    extends: vec![],
                    type_params: vec![
                        TypeParam {
                            name: "T".to_string(),
                            constraint: None,
                            default: Some(TypeInfo::KeywordType(TsKeywordTypeKind::String)),
                        },
                        TypeParam {
                            name: "U".to_string(),
                            constraint: Some(TypeInfo::KeywordType(TsKeywordTypeKind::Number)),
                            default: None,
                        }
                    ],
                    properties: vec![
                        TsInterfaceProperty {
                            name: "first".to_string(),
                            type_info: TypeInfo::TypeRef(TsTypeRef {
                                name: "T".to_string(),
                                type_params: vec![]
                            }),
                            is_optional: false,
                            is_readonly: false,
                        },
                        TsInterfaceProperty {
                            name: "second".to_string(),
                            type_info: TypeInfo::TypeRef(TsTypeRef {
                                name: "U".to_string(),
                                type_params: vec![]
                            }),
                            is_optional: false,
                            is_readonly: false,
                        }
                    ]
                })
            )
        );
    }

    #[test]
    fn test_simple_type_ref() {
        let src = r#"
        declare const ref: Array;
        declare const withTypeArg: Array<number>;
        declare const nested: Array<Array<string>>;

        interface Person {
            name: string;
            age: number;
            foo?: string;
            readonly bar: boolean;
        }
        declare const person: Person;
        "#;

        let analyzer = test_analyzer(src, JsFileSource::ts());

        assert_eq!(
            analyzer.get_symbol("ref").unwrap(),
            &Symbol::new(
                "ref".to_string(),
                TypeInfo::TypeRef(TsTypeRef {
                    name: "Array".to_string(),
                    type_params: vec![]
                })
            )
        );

        assert_eq!(
            analyzer.get_symbol("withTypeArg").unwrap(),
            &Symbol::new(
                "withTypeArg".to_string(),
                TypeInfo::TypeRef(TsTypeRef {
                    name: "Array".to_string(),
                    type_params: vec![TypeInfo::KeywordType(TsKeywordTypeKind::Number)]
                })
            )
        );

        assert_eq!(
            analyzer.get_symbol("nested").unwrap(),
            &Symbol::new(
                "nested".to_string(),
                TypeInfo::TypeRef(TsTypeRef {
                    name: "Array".to_string(),
                    type_params: vec![TypeInfo::TypeRef(TsTypeRef {
                        name: "Array".to_string(),
                        type_params: vec![TypeInfo::KeywordType(TsKeywordTypeKind::String)]
                    })]
                })
            )
        );

        assert_eq!(
            analyzer.get_symbol("person").unwrap(),
            &Symbol::new(
                "person".to_string(),
                TypeInfo::TypeRef(TsTypeRef {
                    name: "Person".to_string(),
                    type_params: vec![]
                })
            )
        );
    }

    #[test]
    fn test_union_test() {
        let src = r#"
        declare const basic: string | number;
        declare const withLiteral: "foo" | 42 | true;
        declare const withRef: Array<string> | Promise<number>;
        declare const nested: (string | number) | (boolean | null);
        "#;

        let analyzer = test_analyzer(src, JsFileSource::ts());

        assert_eq!(
            analyzer.get_symbol("basic").unwrap(),
            &Symbol::new(
                "basic".to_string(),
                TypeInfo::Union(vec![
                    TypeInfo::KeywordType(TsKeywordTypeKind::String),
                    TypeInfo::KeywordType(TsKeywordTypeKind::Number)
                ])
            )
        );

        assert_eq!(
            analyzer.get_symbol("withLiteral").unwrap(),
            &Symbol::new(
                "withLiteral".to_string(),
                TypeInfo::Union(vec![
                    TypeInfo::Literal(TsLiteralTypeKind::String("\"foo\"".to_string())),
                    TypeInfo::Literal(TsLiteralTypeKind::Number(42)),
                    TypeInfo::Literal(TsLiteralTypeKind::Boolean(BoolLiteral::True))
                ])
            )
        );

        assert_eq!(
            analyzer.get_symbol("withRef").unwrap(),
            &Symbol::new(
                "withRef".to_string(),
                TypeInfo::Union(vec![
                    TypeInfo::TypeRef(TsTypeRef {
                        name: "Array".to_string(),
                        type_params: vec![TypeInfo::KeywordType(TsKeywordTypeKind::String)]
                    }),
                    TypeInfo::TypeRef(TsTypeRef {
                        name: "Promise".to_string(),
                        type_params: vec![TypeInfo::KeywordType(TsKeywordTypeKind::Number)]
                    })
                ])
            )
        );

        assert_eq!(
            analyzer.get_symbol("nested").unwrap(),
            &Symbol::new(
                "nested".to_string(),
                TypeInfo::Union(vec![
                    TypeInfo::Union(vec![
                        TypeInfo::KeywordType(TsKeywordTypeKind::String),
                        TypeInfo::KeywordType(TsKeywordTypeKind::Number)
                    ]),
                    TypeInfo::Union(vec![
                        TypeInfo::KeywordType(TsKeywordTypeKind::Boolean),
                        TypeInfo::KeywordType(TsKeywordTypeKind::Null)
                    ])
                ])
            )
        );
    }

    #[test]
    fn test_func_type() {
        let src = r#"
        declare const basic: () => void;
        declare const withParams: (x: number, y: string) => boolean;
        declare const withOptional: (x?: number) => string;
        declare const generic: <T>(value: T) => T;
        "#;

        let analyzer = test_analyzer(src, JsFileSource::ts());

        assert_eq!(
            analyzer.get_symbol("basic").unwrap(),
            &Symbol::new(
                "basic".to_string(),
                TypeInfo::Function(TsFunctionSignature {
                    params: vec![],
                    return_type: Box::new(TypeInfo::KeywordType(TsKeywordTypeKind::Void)),
                    type_params: vec![],
                    this_param: None,
                })
            )
        );

        assert_eq!(
            analyzer.get_symbol("withParams").unwrap(),
            &Symbol::new(
                "withParams".to_string(),
                TypeInfo::Function(TsFunctionSignature {
                    params: vec![
                        FunctionParam {
                            name: "x".to_string(),
                            param_type: TypeInfo::KeywordType(TsKeywordTypeKind::Number),
                            is_optional: false
                        },
                        FunctionParam {
                            name: "y".to_string(),
                            param_type: TypeInfo::KeywordType(TsKeywordTypeKind::String),
                            is_optional: false
                        }
                    ],
                    return_type: Box::new(TypeInfo::KeywordType(TsKeywordTypeKind::Boolean)),
                    type_params: vec![],
                    this_param: None,
                })
            )
        );

        assert_eq!(
            analyzer.get_symbol("withOptional").unwrap(),
            &Symbol::new(
                "withOptional".to_string(),
                TypeInfo::Function(TsFunctionSignature {
                    params: vec![FunctionParam {
                        name: "x".to_string(),
                        param_type: TypeInfo::KeywordType(TsKeywordTypeKind::Number),
                        is_optional: true
                    }],
                    return_type: Box::new(TypeInfo::KeywordType(TsKeywordTypeKind::String)),
                    type_params: vec![],
                    this_param: None,
                })
            )
        );

        assert_eq!(
            analyzer.get_symbol("generic").unwrap(),
            &Symbol::new(
                "generic".to_string(),
                TypeInfo::Function(TsFunctionSignature {
                    params: vec![FunctionParam {
                        name: "value".to_string(),
                        param_type: TypeInfo::TypeRef(TsTypeRef {
                            name: "T".to_string(),
                            type_params: vec![]
                        }),
                        is_optional: false
                    }],
                    return_type: Box::new(TypeInfo::TypeRef(TsTypeRef {
                        name: "T".to_string(),
                        type_params: vec![]
                    })),
                    type_params: vec![TypeParam {
                        name: "T".to_string(),
                        constraint: None,
                        default: None
                    }],
                    this_param: None,
                })
            )
        );
    }

    #[test]
    fn test_arrow_function() {
        let src = r#"
        const foo = (): Promise<string> => {};"#;

        let analyzer = test_analyzer(src, JsFileSource::ts());
        assert_eq!(
            analyzer.get_symbol("foo").unwrap(),
            &Symbol::new(
                "foo".to_string(),
                TypeInfo::Function(TsFunctionSignature {
                    params: vec![],
                    return_type: Box::new(TypeInfo::TypeRef(TsTypeRef {
                        name: "Promise".to_string(),
                        type_params: vec![TypeInfo::KeywordType(TsKeywordTypeKind::String)]
                    })),
                    type_params: vec![],
                    this_param: None,
                })
            )
        );
    }

    #[test]
    fn test_resolve_generic_interface() {
        let src = r#"
        interface Promise<T> {
          then<TResult1 = T, TResult2 = never>(onfulfilled?: ((value: T) => TResult1 | PromiseLike<TResult1>) | undefined | null, onrejected?: ((reason: any) => TResult2 | PromiseLike<TResult2>) | undefined | null): Promise<TResult1 | TResult2>;
          catch<TResult = never>(onrejected?: ((reason: any) => TResult | PromiseLike<TResult>) | undefined | null): Promise<T | TResult>;
        }

        interface PromiseLike<T> {
          then<TResult1 = T, TResult2 = never>(onfulfilled?: ((value: T) => TResult1 | PromiseLike<TResult1>) | undefined | null, onrejected?: ((reason: any) => TResult2 | PromiseLike<TResult2>) | undefined | null): PromiseLike<TResult1 | TResult2>;
        }

        declare const foo: () => Promise<string>;
        "#;

        let analyzer = test_analyzer(src, JsFileSource::ts());
        analyzer.print_symbol_table();
        analyzer.print_global_symbol_table();

        let symbol = analyzer.get_symbol("foo").unwrap();
        let type_info = analyzer.resolve_type_info(symbol, &PathBuf::new());
        dbg!(&type_info);
    }

    #[test]
    #[ignore]
    fn quick_test() {
        let src = r#"
        interface Promise<T> {
          then<TResult1 = T, TResult2 = never>(onfulfilled?: ((value: T) => TResult1 | PromiseLike<TResult1>) | undefined | null, onrejected?: ((reason: any) => TResult2 | PromiseLike<TResult2>) | undefined | null): Promise<TResult1 | TResult2>;
          catch<TResult = never>(onrejected?: ((reason: any) => TResult | PromiseLike<TResult>) | undefined | null): Promise<T | TResult>;
        }

        interface PromiseLike<T> {
          then<TResult1 = T, TResult2 = never>(onfulfilled?: ((value: T) => TResult1 | PromiseLike<TResult1>) | undefined | null, onrejected?: ((reason: any) => TResult2 | PromiseLike<TResult2>) | undefined | null): PromiseLike<TResult1 | TResult2>;
        }

        declare const foo: () => Promise<string>;
        "#;

        let analyzer = test_analyzer(src, JsFileSource::ts());
        analyzer.print_symbol_table();
        analyzer.print_global_symbol_table();
    }
}
