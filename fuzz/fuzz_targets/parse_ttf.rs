#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|font_data: &[u8]| {
    let face = match ttf_parser::Face::from_slice(&font_data, 0) {
        Ok(f) => f,
        Err(_) => return,
    };

    let family_name = face
        .names()
        .into_iter()
        .find(|name| name.name_id == ttf_parser::name_id::FULL_NAME && name.is_unicode())
        .and_then(|name| name.to_string());

    let post_script_name = face
        .names()
        .into_iter()
        .find(|name| name.name_id == ttf_parser::name_id::POST_SCRIPT_NAME && name.is_unicode())
        .and_then(|name| name.to_string());

    let _ = family_name;
    let _ = post_script_name;
    let _ = face.units_per_em();
    let _ = face.ascender();
    let _ = face.descender();
    let _ = face.line_gap();
    let _ = face.global_bounding_box();
    let _ = face.number_of_glyphs();
    let _ = face.underline_metrics();
    let _ = face.x_height();
    let _ = face.weight();
    let _ = face.width();
    let _ = face.is_regular();
    let _ = face.is_italic();
    let _ = face.is_bold();
    let _ = face.is_oblique();
    let _ = face.strikeout_metrics();
    let _ = face.subscript_metrics();
    let _ = face.superscript_metrics();
    let _ = face.is_variable();

    if let Some(ref table) = face.tables().gpos {
        print_opentype_layout("positioning", table);
    }

    if let Some(ref table) = face.tables().gsub {
        print_opentype_layout("substitution", table);
    }

    if face.is_variable() {
        println!("Variation axes:");
        for axis in face.variation_axes() {
            println!(
                "  {} {}..{}, default {}",
                axis.tag, axis.min_value, axis.max_value, axis.def_value
            );
        }
    }
});

fn print_opentype_layout(name: &str, table: &ttf_parser::opentype_layout::LayoutTable) {
    println!("OpenType {}:", name);
    println!("  Scripts:");
    for script in table.scripts {
        println!("    {}", script.tag);

        if script.languages.is_empty() {
            println!("      No languages");
            continue;
        }

        println!("      Languages:");
        for lang in script.languages {
            println!("        {}", lang.tag);
        }
    }

    let mut features: Vec<_> = table.features.into_iter().map(|f| f.tag).collect();
    features.dedup();
    println!("  Features:");
    for feature in features {
        println!("    {}", feature);
    }
}
