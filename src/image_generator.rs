use image::DynamicImage;
use silicon::{assets::HighlightingAssets, formatter::ImageFormatterBuilder, utils::ShadowAdder, font::ImageFont};
use syntect::{easy::HighlightLines, util::LinesWithEndings};
use resvg::{self, usvg::{self, TreeParsing, fontdb, TreeTextToPath}, tiny_skia};

use crate::{routes::ImgQuery};

pub(crate) fn generate_src_image(code: &str, starting_line: u32, theme: &str, font: &str, font_size: f32) -> DynamicImage {
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
        .line_offset(starting_line)
        .build()
        .unwrap();

    formatter.format(&highlight, theme)
}

pub(crate) fn generate_src_image_with_query(code: &str, query: &ImgQuery) -> DynamicImage {    
    generate_src_image(
        code,
        query.lines.map(|lines| lines.from).unwrap_or(1),
        &query.theme.clone().unwrap_or("Dracula".to_owned()),
        &query.font.clone().unwrap_or("Hack".to_owned()),
        query.font_size.unwrap_or(26.0)
    )
}

pub(crate) fn generate_svg_image(buffer: &[u8]) -> Option<Vec<u8>> {
    let mut db = fontdb::Database::new();
    db.load_font_data(include_bytes!("../assets/fonts/OpenSans-Regular.ttf").to_vec());
    db.set_serif_family("Open Sans".to_string());
    let options = usvg::Options {
        font_family: "sans-serif".to_string(),
        dpi: 96.0,
        ..Default::default()
    };
    let mut tree = usvg::Tree::from_data(buffer, &options).ok()?;
    if tree.has_text_nodes() {
        tree.convert_text(&db);
    }
    let rtree = resvg::Tree::from_usvg(&tree);
    let size = rtree.size.to_int_size().scale_to_width(960)?;
    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height())?;
    let render_ts = tiny_skia::Transform::from_scale(
        size.width() as f32 / tree.size.width() as f32,
        size.height() as f32 / tree.size.height() as f32,
    );
    rtree.render(render_ts, &mut pixmap.as_mut());
    pixmap.encode_png().ok()
}