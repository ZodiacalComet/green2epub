use crate::parser::RESET_FOREGROUND_CLASS;
use crate::tag::{Child, Tag};

pub const COVER_STYLESHEET: &str = "style/coverstyle.css";

fn xhtml_content_from_html_tag(html: Tag) -> String {
    format!(
        "<?xml version='1.0' encoding='utf-8'?><!DOCTYPE html>{}",
        html
    )
}

pub fn coverpage_content<S>(href: S) -> String
where
    S: AsRef<str>,
{
    let mut html = Tag::new("html");
    html.attribute("xmlns", "http://www.w3.org/1999/xhtml")
        .attribute("xmlns:epub", "http://www.idpf.org/2007/ops")
        .attribute("lang", "en")
        .attribute("xml:lang", "en")
        .child(
            Tag::new("head")
                .child(Tag::new("meta").attribute("charset", "utf-8"))
                .child(
                    Tag::new("meta")
                        .attribute("name", "viewport")
                        .attribute("content", "width=588, height=512"),
                )
                .child(Tag::new("title").child("Cover"))
                .child(
                    Tag::new("link")
                        .attribute("rel", "stylesheet")
                        .attribute("type", "text/css")
                        .attribute("href", format!("../{}", COVER_STYLESHEET)),
                ),
        )
        .child(
            Tag::new("body")
                .attribute("epub:type", "frontmatter cover")
                .attribute("id", "coverpage")
                .child(
                    Tag::new("svg")
                        .attribute("xmlns", "http://www.w3.org/2000/svg")
                        .attribute("xmlns:xlink", "http://www.w3.org/1999/xlink")
                        .attribute("version", "1.1")
                        .attribute("viewBox", "0 0 588 512")
                        .attribute("id", "cover")
                        .child(
                            Tag::new("image")
                                .attribute("width", "588")
                                .attribute("height", "512")
                                .attribute("xlink:href", format!("../{}", href.as_ref())),
                        ),
                ),
        );

    xhtml_content_from_html_tag(html)
}

pub fn stylesheet_content<G, S>(green_color: G, spoiler_color: S) -> Vec<u8>
where
    G: AsRef<str>,
    S: AsRef<str>,
{
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend(
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/style.css")).as_slice(),
    );

    // By default, highlight all paragraphs with green color and use a class to remove it.
    // This is because most of the lines are going to be highlighted in the majority of greens
    // anyways.
    bytes.extend(
        format!(
            "p {{ color: {green_color}; }}\n\
            .{reset_foreground_class} {{ color: initial; }}\n\
            p > span {{ background-color: {spoiler_color}; color: transparent; }}",
            green_color = green_color.as_ref(),
            spoiler_color = spoiler_color.as_ref(),
            reset_foreground_class = RESET_FOREGROUND_CLASS
        )
        .as_bytes(),
    );

    bytes
}

pub struct PasteContent {
    title: String,
    body: Tag,
}

impl PasteContent {
    pub fn new<S>(title: S) -> Self
    where
        S: ToString,
    {
        Self {
            title: title.to_string(),
            body: Tag::new("body"),
        }
    }

    pub fn add_line<C>(&mut self, child: C) -> &mut Self
    where
        C: Into<Child>,
    {
        self.body.child(child);
        self
    }

    pub fn build(self) -> String {
        let mut html = Tag::new("html");
        html.attribute("xmlns", "http://www.w3.org/1999/xhtml")
            .attribute("xmlns:epub", "http://www.idpf.org/2007/ops")
            .attribute(
                "epub:prefix",
                "z3998: http://www.daisy.org/z3998/2012/vocab/structure/#",
            )
            .attribute("lang", "en")
            .attribute("xml:lang", "en")
            .child(
                Tag::new("head")
                    .child(Tag::new("title").child(self.title))
                    .child(
                        Tag::new("link")
                            .attribute("href", "../stylesheet.css")
                            .attribute("rel", "stylesheet")
                            .attribute("type", "text/css"),
                    ),
            )
            .child(self.body);

        xhtml_content_from_html_tag(html)
    }
}
