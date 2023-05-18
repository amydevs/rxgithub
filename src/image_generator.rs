use image::DynamicImage;
use silicon::{assets::HighlightingAssets, formatter::ImageFormatterBuilder, utils::ShadowAdder};
use syntect::{easy::HighlightLines, util::LinesWithEndings};

use crate::{routes::ImgQuery, utils::{substring_lines_with_max, QueryLines}};

pub(crate) fn generate_src_image(code: &str, theme: &str, font: &str, font_size: f32) -> DynamicImage {
    let ha = HighlightingAssets::new();
    let (ps, ts) = (ha.syntax_set, ha.theme_set);

    // Change this later to first choose syntax by file extension
    let syntax = ps.find_syntax_by_first_line(code).unwrap_or(ps.find_syntax_by_token("rs").unwrap());
    let theme = ts.themes.get(theme).unwrap_or(&ts.themes["Dracula"]);

    let mut h = HighlightLines::new(syntax, theme);
    let highlight = LinesWithEndings::from(code)
        .map(|line| h.highlight_line(line, &ps))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    
    let mut formatter = ImageFormatterBuilder::new()
        .font(vec![(font, font_size)])
         .shadow_adder(ShadowAdder::default())
         .build()
         .unwrap();

    formatter.format(&highlight, theme)
}

pub(crate) fn generate_src_image_with_query(code: &str, query: &ImgQuery) -> DynamicImage {
    let truncated_src_code = query.lines.as_ref().map(|query_lines| substring_lines_with_max(code, query_lines)).unwrap_or(substring_lines_with_max(code, &QueryLines::new()));
    
    generate_src_image(
        &truncated_src_code,
        &query.theme.clone().unwrap_or("Dracula".to_owned()),
        &query.font.clone().unwrap_or("Hack".to_owned()),
        query.font_size.unwrap_or(26.0)
    )
}