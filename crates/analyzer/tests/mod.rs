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
}
