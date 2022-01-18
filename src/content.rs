use crate::parser::RESET_FOREGROUND_CLASS;
use crate::tag::{Child, Tag};

const NS_XHTML: &str = "http://www.w3.org/1999/xhtml";
const NS_OPS: &str = "http://www.idpf.org/2007/ops";
const NS_SVG: &str = "http://www.w3.org/2000/svg";
const NS_XLINK: &str = "http://www.w3.org/1999/xlink";

pub const COVER_STYLESHEET: &str = "style/coverstyle.css";

fn xhtml_content_from_html_tag(html: Tag) -> String {
    format!(
        "<?xml version='1.0' encoding='utf-8'?><!DOCTYPE html>{}",
        html
    )
}

// This cover page is fimfic2epub's cover page translated to the Tag's constructor with no
// remarkable differences.
//  https://github.com/daniel-j/fimfic2epub/blob/master/src/templates.js#L353
//  MIT License: https://github.com/daniel-j/fimfic2epub/blob/master/LICENSE
pub fn coverpage_content<S>(href: S, (width, height): (usize, usize)) -> String
where
    S: AsRef<str>,
{
    let mut html = Tag::new("html");
    html.attribute("xmlns", NS_XHTML)
        .attribute("xmlns:epub", NS_OPS)
        .attribute("lang", "en")
        .attribute("xml:lang", "en")
        .child(
            Tag::new("head")
                .child(Tag::new("meta").attribute("charset", "utf-8"))
                .child(
                    Tag::new("meta")
                        .attribute("name", "viewport")
                        .attribute("content", format!("width={}, height={}", width, height)),
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
                        .attribute("xmlns", NS_SVG)
                        .attribute("xmlns:xlink", NS_XLINK)
                        .attribute("version", "1.1")
                        .attribute("viewBox", format!("0 0 {} {}", width, height))
                        .attribute("id", "cover")
                        .child(
                            Tag::new("image")
                                .attribute("width", width)
                                .attribute("height", height)
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
        html.attribute("xmlns", NS_XHTML)
            .attribute("xmlns:epub", NS_OPS)
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
