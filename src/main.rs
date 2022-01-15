#[macro_use]
extern crate log;

use std::{
    fs::{read_to_string, File, OpenOptions},
    path::PathBuf,
};

use clap::Parser;
use console::style;
use epub_builder::{EpubBuilder, EpubContent, EpubVersion, ReferenceType, ZipLibrary};
use indicatif::{ProgressBar, ProgressFinish, ProgressIterator, ProgressStyle};

mod args;
mod content;
mod logger;
mod parser;
mod tag;

use args::Args;
use content::{coverpage_content, stylesheet_content, PasteContent, COVER_STYLESHEET};
use parser::LineParser;
use tag::Tag;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args = Args::parse();
    logger::init(args.verbose).expect("failed to initialize logger");

    debug!("Parsed arguments: {:?}", args);

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
        info!("Setting cover to {:?}", style(path.display()).bold());

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

        debug!("Opening cover file");
        let reader = File::open(&path)?;

        debug!("Adding cover resources to EPUB");
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

        debug!("Opening file {:?}", path.display());
        let content = read_to_string(&path)?;

        let mut line_parser = LineParser::default();
        let progress = ProgressBar::new(content.lines().count() as u64)
            .with_message(format!("Parsing {:?}", style(path.display()).bold()))
            .with_style(
                ProgressStyle::default_spinner()
                    .template("  {spinner}  {msg} {percent:>3}%")
                    .on_finish(ProgressFinish::AndClear),
            );
        for line in content.lines().progress_with(progress) {
            if line.is_empty() {
                paste.add_line(Tag::new("br"));
                continue;
            }

            paste.add_line(line_parser.parse(line));
        }

        info!("Parsed {:?}", style(path.display()).bold());

        debug!(
            "Adding parsed content of {:?} to EPUB with title {:?}",
            path.display(),
            &title
        );
        epub.add_content(
            EpubContent::new(
                format!("content/paste-{:03}.xhtml", count),
                paste.build().as_bytes(),
            )
            .title(title),
        )?;
    }

    debug!("Creating output file");
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&args.output)?;
    epub.generate(&mut output_file)?;
    info!(
        "{}",
        style(format_args!(
            "Successfully generated {:?}",
            style(args.output).bold()
        ))
        .green()
    );

    Ok(())
}
