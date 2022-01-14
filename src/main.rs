use std::{
    fs::{read_to_string, File, OpenOptions},
    path::PathBuf,
};

use clap::Parser;
use epub_builder::{EpubBuilder, EpubContent, EpubVersion, ReferenceType, ZipLibrary};

mod args;
mod content;
mod parser;
mod tag;

use args::Args;
use content::{coverpage_content, stylesheet_content, PasteContent, COVER_STYLESHEET};
use parser::LineParser;
use tag::Tag;

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
