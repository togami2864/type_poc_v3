#[cfg(test)]
mod tests {
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
        let mut analyzer = TypeAnalyzer::new();
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
                TypeInfo::Literal(TsLiteralTypeKind::String("hello".to_string()))
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
}
