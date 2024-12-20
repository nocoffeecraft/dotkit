use std::fs;
use tera::{Tera, Context};

use dotkit::{Contract};

pub fn template(c: &Contract) {
    let c_name = &c.name;
    let ct = &c.typ;

    let tera = Tera::new("templates/**/*.tera").unwrap();

    let mut context = Context::new();
    context.insert("c_name", c_name);
    context.insert("ct", ct);

    let c_folder = format!("./{}", c_name);
    
    fs::create_dir_all(&c_folder).expect("Failed to create output folder.");

    generate_file(&tera, &context, "lib.tera", &c_folder, "lib.rs");
    generate_file(&tera, &context, "cargo.tera", &c_folder, "Cargo.toml");

    println!("Contract scaffolding generated at: {}", c_folder);
}

fn generate_file(tera: &Tera, context: &Context, template_name: &str, output_folder: &str, output_file: &str) {
    let rendered_content = tera.render(template_name, context).unwrap();

    let file_path = format!("{}/{}", output_folder, output_file);

    fs::write(file_path, rendered_content).expect("Failed to write file.");
}
