use image::DynamicImage;
use resvg::{
    self, tiny_skia,
    usvg::{self, fontdb, TreeParsing, TreeTextToPath},
};
use silicon::{assets::HighlightingAssets, formatter::ImageFormatterBuilder, utils::ShadowAdder};
use syntect::{easy::HighlightLines, util::LinesWithEndings};

use crate::routes::ImgQuery;

pub(crate) struct TextImageGenerator {
    ha: HighlightingAssets,
}

impl Default for TextImageGenerator {
    fn default() -> Self {
        Self { 
            ha: HighlightingAssets::new()
        }
    }
}

impl TextImageGenerator {
    pub(crate) fn generate(
        &self,
        code: &str,
        starting_line: u32,
        theme: &str,
        font: &str,
        font_size: f32,
    ) -> DynamicImage {
        let (ps, ts) = (&self.ha.syntax_set, &self.ha.theme_set);
    
        // Change this later to first choose syntax by file extension
        let syntax = ps
            .find_syntax_by_first_line(code)
            .unwrap_or(ps.find_syntax_by_token("rs").unwrap());
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
    pub(crate) fn generate_from_query(&self, code: &str, query: &ImgQuery) -> DynamicImage {
        self.generate(
            code,
            query.lines.map(|lines| lines.from).unwrap_or(1),
            &query.theme.clone().unwrap_or("Dracula".to_owned()),
            &query.font.clone().unwrap_or("Hack".to_owned()),
            query.font_size.unwrap_or(26.0),
        )
    }
}

pub(crate) struct SvgImageGenerator {
    db: fontdb::Database,
}

impl Default for SvgImageGenerator {
    fn default() -> Self {
        let mut db = fontdb::Database::new();
        db.load_font_data(include_bytes!("../assets/fonts/OpenSans-Regular.ttf").to_vec());
        db.set_serif_family("Open Sans".to_string());
        Self { db: Default::default() }
    }
}

impl SvgImageGenerator {
    pub(crate) fn generate(&self, buffer: &[u8]) -> Option<Vec<u8>> {
        let options = usvg::Options {
            font_family: "sans-serif".to_string(),
            dpi: 96.0,
            ..Default::default()
        };
        let mut tree = usvg::Tree::from_data(buffer, &options).ok()?;
        if tree.has_text_nodes() {
            tree.convert_text(&self.db);
        }
        let rtree = resvg::Tree::from_usvg(&tree);
        let size = rtree.size.to_int_size().scale_to_width(960)?;
        let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height())?;
        let render_ts = tiny_skia::Transform::from_scale(
            size.width() as f32 / tree.size.width(),
            size.height() as f32 / tree.size.height(),
        );
        rtree.render(render_ts, &mut pixmap.as_mut());
        pixmap.encode_png().ok()
    }
}
