use rust_embed::RustEmbed;
use serde::Serialize;
use std::path::Path;
use tera::{Context, Tera};

use std::fs;

pub fn templates_files<T: RustEmbed, S: Serialize>(project_name: String, context: &S) {
    for file in T::iter() {
        let filename = file.as_ref().to_string();

        let filepath = format!("{}/{}", project_name, filename);

        let file = T::get(&filename).unwrap().data;
        let filecontents = std::str::from_utf8(file.as_ref()).unwrap();

        let rendered_contents = Tera::one_off(
            filecontents,
            &Context::from_serialize(&context).expect("Cannot serialize struct"),
            true,
        )
        .unwrap();
        if rendered_contents.trim().len() == 0 {
            continue;
        }

        fs::create_dir_all(&Path::new(&filepath).parent().unwrap()).ok();
        fs::write(filepath, rendered_contents).expect("Couldn't write file");
    }
}
