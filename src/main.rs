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
