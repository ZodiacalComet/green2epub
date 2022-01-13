use std::{
    fs::{read_to_string, File, OpenOptions},
    path::PathBuf,
};

use clap::Parser;
use epub_builder::{EpubBuilder, EpubContent, EpubVersion, ReferenceType, ZipLibrary};

mod args;
mod tag;

use args::Args;
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
                            .attribute("href", "../style.css")
                            .attribute("rel", "stylesheet")
                            .attribute("type", "text/css"),
                    ),
            )
            .child(self.body);

        xhtml_content_from_html_tag(html)
    }
}

const COVER_IMG: &str = "img/cover.png";
const COVER_STYLESHEET: &str = "style/coverstyle.css";

fn coverpage_content() -> String {
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
                                .attribute("xlink:href", format!("../{}", COVER_IMG)),
                        ),
                ),
        );

    xhtml_content_from_html_tag(html)
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args = Args::parse();

    let mut epub = EpubBuilder::new(ZipLibrary::new()?)?;
    epub.epub_version(EpubVersion::V30)
        .metadata("author", args.author)?
        .metadata("title", args.title)?
        .stylesheet("".as_bytes())?;

    if let Some(subjects) = args.subjects {
        for subject in subjects {
            epub.metadata("subject", subject)?;
        }
    }

    if let Some(path) = args.cover {
        let reader = File::open(path)?;
        epub.add_cover_image(COVER_IMG, reader, "image/png")?;

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
            EpubContent::new("content/cover.xhtml", coverpage_content().as_bytes())
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

        let content = read_to_string(&path)?;
        for line in content.lines() {
            if line.is_empty() {
                paste.add_line(Tag::new("br"));
                continue;
            }

            // TODO: Highlight lines that start with ">".
            // TODO: Parse spoilers.
            paste.add_line(Tag::new("p").child(line));
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
