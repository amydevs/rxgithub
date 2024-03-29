use maud::{html, PreEscaped};

use crate::{
    routes::{GistPath, SrcPath},
    utils::Lines,
};

pub(crate) trait Content {
    fn get_html(&self) -> PreEscaped<String>;
}

pub(crate) struct TextContent<'a> {
    pub(crate) path: &'a SrcPath,
    pub(crate) query_string: String,
    pub(crate) lines: Lines,
    pub(crate) origin: String,
}

impl<'a> Content for TextContent<'a> {
    fn get_html(&self) -> PreEscaped<String> {
        let file_name = self.path.path.split('/').last().unwrap_or("<undefined>");
        let og_title = format!(
            "{} · {}/{}@{}",
            file_name, self.path.repository, self.path.author, self.path.branch
        );
        let og_image = format!(
            "{}/image/{}/{}/{}/{}?{}",
            self.origin,
            self.path.author,
            self.path.repository,
            self.path.branch,
            self.path.path,
            self.query_string
        );
        let og_description = format!(
            "Lines {}-{} of {} from {}/{}@{}",
            self.lines.from,
            self.lines.to,
            file_name,
            self.path.author,
            self.path.repository,
            self.path.branch
        );
        html! {
            meta name="description" content=(og_description);
            meta property="og:image" content=(og_image);
            meta property="og:image:type" content="image/png";
            meta property="og:title" content=(og_title);
            meta property="og:description" content=(og_description);

            meta name="twitter:title" content=(og_title);
            meta name="twitter:card" content="summary_large_image";
            meta name="twitter:description" content=(og_description);
            meta name="twitter:image" content=(og_image);
        }
    }
}

pub(crate) struct ImageContent<'a> {
    pub(crate) path: &'a SrcPath,
    pub(crate) image_url: String,
    pub(crate) mime: String,
}

impl<'a> Content for ImageContent<'a> {
    fn get_html(&self) -> PreEscaped<String> {
        let file_name = self.path.path.split('/').last().unwrap_or("<undefined>");
        let og_title = format!(
            "{} · {}/{}@{}",
            file_name, self.path.repository, self.path.author, self.path.branch
        );
        let og_description = format!(
            "{} from {}/{}@{}",
            file_name, self.path.author, self.path.repository, self.path.branch
        );
        html! {
            meta name="description" content=(og_description);
            meta property="og:image" content=(self.image_url);
            meta property="og:image:type" content=(self.mime);
            meta property="og:title" content=(og_title);
            meta property="og:description" content=(og_description);

            meta name="twitter:title" content=(og_title);
            meta name="twitter:card" content="summary_large_image";
            meta name="twitter:description" content=(og_description);
            meta name="twitter:image" content=(self.image_url);
        }
    }
}

pub(crate) struct SVGContent<'a> {
    pub(crate) path: &'a SrcPath,
    pub(crate) origin: String,
}

impl<'a> Content for SVGContent<'a> {
    fn get_html(&self) -> PreEscaped<String> {
        let file_name = self.path.path.split('/').last().unwrap_or("<undefined>");
        let og_title = format!(
            "{} · {}/{}@{}",
            file_name, self.path.repository, self.path.author, self.path.branch
        );
        let og_image = format!(
            "{}/image/{}/{}/{}/{}",
            self.origin, self.path.author, self.path.repository, self.path.branch, self.path.path
        );
        let og_description = format!(
            "{} from {}/{}@{}",
            file_name, self.path.author, self.path.repository, self.path.branch
        );
        html! {
            meta name="description" content=(og_description);
            meta property="og:image" content=(og_image);
            meta property="og:image:type" content="image/png";
            meta property="og:title" content=(og_title);
            meta property="og:description" content=(og_description);

            meta name="twitter:title" content=(og_title);
            meta name="twitter:card" content="summary_large_image";
            meta name="twitter:description" content=(og_description);
            meta name="twitter:image" content=(og_image);
        }
    }
}

pub(crate) struct VideoContent<'a> {
    pub(crate) path: &'a SrcPath,
    pub(crate) video_url: String,
    pub(crate) mime: String,
    pub(crate) origin: String,
}

impl<'a> Content for VideoContent<'a> {
    fn get_html(&self) -> PreEscaped<String> {
        let file_name = self.path.path.split('/').last().unwrap_or("<undefined>");
        let video_embed_url = format!(
            "{}/video-embed/{}/{}/{}/{}",
            self.origin, self.path.author, self.path.repository, self.path.branch, self.path.path
        );
        let og_title = format!(
            "{} · {}/{}@{}",
            file_name, self.path.repository, self.path.author, self.path.branch
        );
        let og_description = format!(
            "{} from {}/{}@{}",
            file_name, self.path.author, self.path.repository, self.path.branch
        );
        html! {
            meta name="description" content=(og_description);
            meta property="og:video" content=(self.video_url);
            meta property="og:video:type" content=(self.mime);
            meta property="og:title" content=(og_title);
            meta property="og:description" content=(og_description);

            meta name="twitter:title" content=(og_title);
            meta name="twitter:card" content="player";
            meta name="twitter:description" content=(og_description);
            meta name="twitter:player" content=(video_embed_url);
            meta name="twitter:player:width" content="1280";
            meta name="twitter:player:height" content="720";
        }
    }
}

pub(crate) struct GistContent<'a> {
    pub(crate) path: &'a GistPath,
    pub(crate) query_string: String,
    pub(crate) lines: Lines,
    pub(crate) origin: String,
}

impl<'a> Content for GistContent<'a> {
    fn get_html(&self) -> PreEscaped<String> {
        let og_title = format!("{}/{}", self.path.author, self.path.id);
        let og_image = format!(
            "{}/gist-image/{}/{}?{}",
            self.origin, self.path.author, self.path.id, self.query_string
        );
        let og_description = format!(
            "Lines {}-{} of {}/{}",
            self.lines.from, self.lines.to, self.path.author, self.path.id
        );
        html! {
            meta name="description" content=(og_description);
            meta property="og:image" content=(og_image);
            meta property="og:image:type" content="image/png";
            meta property="og:title" content=(og_title);
            meta property="og:description" content=(og_description);

            meta name="twitter:title" content=(og_title);
            meta name="twitter:card" content="summary_large_image";
            meta name="twitter:description" content=(og_description);
            meta name="twitter:image" content=(og_image);
        }
    }
}
