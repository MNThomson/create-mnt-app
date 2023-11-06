use rust_embed::RustEmbed;
use serde::Serialize;
use std::path::Path;
use tera::{Context, Tera};

use std::fs;

fn should_ignore_file(file: &str) -> bool {
    // Ignore GH workflow files
    if file.ends_with(".yml") && file.contains(".github/workflows/") {
        return true;
    }

    return false;
}

pub fn templates_files<T: RustEmbed, S: Serialize>(project_name: String, context: &S) {
    for file in T::iter() {
        let filename = file.as_ref().to_string();

        let filepath = format!("{}/{}", project_name, filename);

        let file = T::get(&filename).unwrap().data;
        let filecontents = std::str::from_utf8(file.as_ref()).unwrap();

        fs::create_dir_all(Path::new(&filepath).parent().unwrap()).ok();

        if should_ignore_file(&filepath) {
            fs::write(filepath, filecontents).expect("Couldn't write file");
        } else {
            let rendered_contents = Tera::one_off(
                filecontents,
                &Context::from_serialize(context).expect("Cannot serialize struct"),
                true,
            )
            .unwrap();
            if rendered_contents.trim().is_empty() {
                continue;
            }
            fs::write(filepath, rendered_contents).expect("Couldn't write file");
        }
    }
}
