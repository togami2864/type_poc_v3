use std::{fs, path::Path};

use biome_js_parser::parse;
use biome_js_syntax::JsFileSource;
use fake_linter::NoFloatingPromisesLinter;
use server::Server;

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let builtin = vec![current_dir.join("src/lib/es5.d.ts")];
    let mut server = Server::new(builtin);

    let tests_dir = current_dir.join("src/tests");
    let paths = get_ts_files(&tests_dir);
    server.analyze(paths.clone());
    server.print_symbol_table();

    let mut linter = NoFloatingPromisesLinter::new(server);

    for path in paths {
        let src = fs::read_to_string(&path).unwrap();
        let src_type = JsFileSource::ts();
        let root = parse(&src, src_type, Default::default()).tree();

        linter.set_current_path(path.clone());
        linter.visit(&root);
    }

    for diagnostic in linter.diagnostics() {
        println!("{}", diagnostic);
    }
    println!();
}

fn get_ts_files(dir: &Path) -> Vec<std::path::PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("ts") {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::parse;
    use biome_js_syntax::JsFileSource;
    use server::Server;
    use std::path::PathBuf;

    fn setup_server(src: &str) -> Server {
        let builtin = vec![PathBuf::from("src/lib/es5.d.ts")];
        let mut server = Server::new(builtin);
        server.test_analyze(src);
        server.print_symbol_table();
        server
    }

    #[test]
    fn test_no_floating_promises_linter() {
        let src = r#"
        async function test(): Promise<void> {
            Promise.resolve("value");
            Promise.resolve("value").then(() => {});
            Promise.resolve("value").catch();
            Promise.resolve("value").finally();
        }
        "#;

        let server = setup_server(src);
        let mut linter = NoFloatingPromisesLinter::new(server);

        let src_type = JsFileSource::ts();
        let root = parse(src, src_type, Default::default()).tree();

        linter.set_current_path(PathBuf::from("test.ts"));
        linter.visit(&root);

        let diagnostics = linter.diagnostics();
        for d in diagnostics {
            println!("{}", d);
        }
    }

    #[test]
    fn test_1() {
        let src = r#"
        async function returnPromise(): Promise<string> {
            return "value";
        }
        // invalid
        returnPromise();
        returnPromise().then(() => {});
        returnPromise().catch();
        returnPromise().finally();

        //valid
        await returnPromise();
        (async () => {
            await returnPromise();
            await returnPromise().then(() => {});
            await returnPromise().catch();
            await returnPromise().finally();
        })();
        "#;

        let server = setup_server(src);
        let mut linter = NoFloatingPromisesLinter::new(server);

        let src_type = JsFileSource::ts();
        let root = parse(src, src_type, Default::default()).tree();

        linter.set_current_path(PathBuf::from("test.ts"));
        linter.visit(&root);

        let diagnostics = linter.diagnostics();
        for d in diagnostics {
            println!("{}", d);
        }
    }

    #[test]
    fn test_2() {
        let src = r#"
        declare const promiseValue: Promise<number>;
        async function test4(): Promise<void> {
            promiseValue;
            promiseValue.then(() => {});
            promiseValue.catch();
            await promiseValue.finally();
        }
        test4();
        "#;

        let server = setup_server(src);
        let mut linter = NoFloatingPromisesLinter::new(server);

        let src_type = JsFileSource::ts();
        let root = parse(src, src_type, Default::default()).tree();

        linter.set_current_path(PathBuf::from("test.ts"));
        linter.visit(&root);

        let diagnostics = linter.diagnostics();
        for d in diagnostics {
            println!("{}", d);
        }
    }

    #[test]
    fn test_3() {
        let src = r#"
        declare const promiseOrNumber: Promise<number> | number;

async function test() {
  promiseOrNumber;
}
        "#;

        let server = setup_server(src);
        let mut linter = NoFloatingPromisesLinter::new(server);

        let src_type = JsFileSource::ts();
        let root = parse(src, src_type, Default::default()).tree();

        linter.set_current_path(PathBuf::from("test.ts"));
        linter.visit(&root);

        let diagnostics = linter.diagnostics();
        for d in diagnostics {
            println!("{}", d);
        }
    }

    #[test]
    fn test_4() {
        let src = r#"
        const foo = async (): Promise<void> => {
            Promise.resolve("value");
        }
        foo();
        "#;

        let server = setup_server(src);
        let mut linter = NoFloatingPromisesLinter::new(server);

        let src_type = JsFileSource::ts();
        let root = parse(src, src_type, Default::default()).tree();

        linter.set_current_path(PathBuf::from("test.ts"));
        linter.visit(&root);

        let diagnostics = linter.diagnostics();
        for d in diagnostics {
            println!("{}", d);
        }
    }
}
