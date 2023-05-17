use image::DynamicImage;
use silicon::{assets::HighlightingAssets, formatter::ImageFormatterBuilder, utils::ShadowAdder};
use syntect::{easy::HighlightLines, util::LinesWithEndings};

pub fn generate_src_image(code: &str) -> DynamicImage {
    let ha = HighlightingAssets::new();
    let (ps, ts) = (ha.syntax_set, ha.theme_set);

    let syntax = ps.find_syntax_by_first_line(code).unwrap_or(ps.find_syntax_plain_text());
    let theme = &ts.themes["Dracula"];

    let mut h = HighlightLines::new(syntax, theme);
    let highlight = LinesWithEndings::from(code)
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