use image::DynamicImage;
use silicon::{assets::HighlightingAssets, formatter::ImageFormatterBuilder, utils::ShadowAdder};
use syntect::{easy::HighlightLines, util::LinesWithEndings};

pub fn generate_src_image(code: &str) -> DynamicImage {
    let ha = HighlightingAssets::new();
    let (ps, ts) = (ha.syntax_set, ha.theme_set);

    // check if truncate code works and maybe remove clone if possible
    let mut cloned_code = code.to_owned();
    let truncate_index = code.match_indices("\n").nth(50).and_then(|e| {Some(e.0)}).unwrap_or(code.len());
    cloned_code.truncate(truncate_index);

    // Change this later to first choose syntax by file extension
    let syntax = ps.find_syntax_by_first_line(&cloned_code).unwrap_or(ps.find_syntax_by_token("rs").unwrap());
    let theme = &ts.themes["Dracula"];

    let mut h = HighlightLines::new(syntax, theme);
    let highlight = LinesWithEndings::from(&cloned_code)
        .map(|line| h.highlight_line(line, &ps))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    
    let mut formatter = ImageFormatterBuilder::new()
        .font(vec![("Hack", 26.0)])
         .shadow_adder(ShadowAdder::default())
         .build()
         .unwrap();

    formatter.format(&highlight, theme)
}