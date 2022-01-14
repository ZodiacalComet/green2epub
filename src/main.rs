use std::{
    fs::{read_to_string, File, OpenOptions},
    path::PathBuf,
};

use clap::Parser;
use epub_builder::{EpubBuilder, EpubContent, EpubVersion, ReferenceType, ZipLibrary};

mod args;
mod parser;
mod tag;

use args::Args;
use parser::{LineParser, RESET_FOREGROUND_CLASS};
use tag::{Child, Tag};

fn xhtml_content_from_html_tag(html: Tag) -> String {
    format!(
        "<?xml version='1.0' encoding='utf-8'?>\
                <!DOCTYPE html>\
                {}",
        html
    )
}

struct PasteContent {
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

const COVER_STYLESHEET: &str = "style/coverstyle.css";

fn coverpage_content<S>(href: S) -> String
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

fn stylesheet_content<G, S>(green_color: G, spoiler_color: S) -> Vec<u8>
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

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args = Args::parse();

    let mut epub = EpubBuilder::new(ZipLibrary::new()?)?;
    epub.epub_version(EpubVersion::V30)
        .metadata("author", args.author)?
        .metadata("title", args.title)?
        .stylesheet(stylesheet_content(args.green_color, args.spoiler_color).as_slice())?;

    if let Some(subjects) = args.subjects {
        for subject in subjects {
            epub.metadata("subject", subject)?;
        }
    }

    if let Some(path) = args.cover.map(PathBuf::from) {
        let extension = path
            .extension()
            .map(|os_str| os_str.to_string_lossy().into_owned())
            .unwrap_or_else(|| "png".into());

        let href = format!("img/cover.{}", extension);
        let mime_type = match extension.as_ref() {
            "bmp" => "image/bmp",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            _ => "image/png",
        };

        let reader = File::open(&path)?;
        epub.add_cover_image(&href, reader, mime_type)?;

        epub.add_resource(
            COVER_STYLESHEET,
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static/coverstyle.css"
            ))
            .as_slice(),
            "text/css",
        )?;
        epub.add_content(
            EpubContent::new("content/cover.xhtml", coverpage_content(&href).as_bytes())
                .title("Cover")
                .reftype(ReferenceType::Cover),
        )?;
    }

    // NOTE: Keep TOC after the cover page.
    epub.inline_toc();

    for (count, path) in args
        .files
        .iter()
        .enumerate()
        .map(|(i, path)| (i + 1, PathBuf::from(path)))
    {
        let title = path
            .file_stem()
            .expect(&format!("failed to get stem of {}", path.display()))
            .to_string_lossy();
        let mut paste = PasteContent::new(&title);

        let mut line_parser = LineParser::default();
        let content = read_to_string(&path)?;
        for line in content.lines() {
            if line.is_empty() {
                paste.add_line(Tag::new("br"));
                continue;
            }

            paste.add_line(line_parser.parse(line));
        }

        epub.add_content(
            EpubContent::new(
                format!("content/paste-{:03}.xhtml", count),
                paste.build().as_bytes(),
            )
            .title(title),
        )?;
    }

    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(args.output)?;
    epub.generate(&mut output_file)?;

    Ok(())
}
