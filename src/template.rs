use std::fs;
use tera::{Context, Tera};

use dotkit::{Contract, CT};

/// Render templates
pub fn template(c: &Contract) {
    let c_name = &c.name; // Name of the contract/project
    let a_name = &c.a_name; // Name of the author
    let a_email = &c.a_email; // Email of the author
    let ct = &c.typ; // Contract Type

    // Init the render engine
    let mut tera = Tera::new("src/templates/**/*.tera").unwrap();
    tera.add_raw_template("counter.tera", include_str!("templates/counter.tera"))
        .unwrap();
    tera.add_raw_template("token.tera", include_str!("templates/token.tera"))
        .unwrap();
    tera.add_raw_template("nft.tera", include_str!("templates/nft.tera"))
        .unwrap();
    tera.add_raw_template("multisig.tera", include_str!("templates/multisig.tera"))
        .unwrap();
    tera.add_raw_template("cargo.tera", include_str!("templates/cargo.tera"))
        .unwrap();
    tera.add_raw_template("gitignore.tera", include_str!("templates/gitignore.tera"))
        .unwrap();

    let mut context = Context::new();
    context.insert("c_name", c_name);
    context.insert("a_name", a_name);
    context.insert("a_email", a_email);
    context.insert("ct", ct);

    // Get the path of project folder
    let c_folder = format!("./{}", c_name);

    // Create the project folder
    fs::create_dir_all(&c_folder).expect("Failed to create output folder.");

    // Render Files
    generate_file(&tera, &context, "cargo.tera", &c_folder, "Cargo.toml");
    generate_file(&tera, &context, "gitignore.tera", &c_folder, ".gitignore");

    match ct {
        CT::Counter => {
            generate_file(&tera, &context, "counter.tera", &c_folder, "lib.rs");
        }

        CT::Token => {
            generate_file(&tera, &context, "token.tera", &c_folder, "lib.rs");
        }
        CT::NFT => {
            generate_file(&tera, &context, "nft.tera", &c_folder, "lib.rs");
        }
        CT::Multisig => {
            generate_file(&tera, &context, "multisig.tera", &c_folder, "lib.rs");
        }
    }
}

fn generate_file(
    tera: &Tera,
    context: &Context,
    template_name: &str,
    output_folder: &str,
    output_file: &str,
) {
    let rendered_content = tera.render(template_name, context).unwrap();

    // Obtain the file path
    let file_path = format!("{}/{}", output_folder, output_file);

    // Write contents to the files
    fs::write(file_path, rendered_content).expect("Failed to write file.");
}
