#[macro_use]
extern crate log;

use std::{
    fs::{read_to_string, File, OpenOptions},
    io::Read,
    path::PathBuf,
};

use clap::Parser;
use console::style;
use epub_builder::{EpubBuilder, EpubContent, EpubVersion, ReferenceType, ZipLibrary};
use indicatif::{ProgressBar, ProgressFinish, ProgressIterator, ProgressStyle};

mod args;
mod content;
mod errors;
mod logger;
mod parser;
mod tag;

use args::Args;
use content::{coverpage_content, stylesheet_content, PasteContent, COVER_STYLESHEET};
use errors::{CliError, CliResult};
use parser::LineParser;
use tag::Tag;

fn run(args: Args) -> CliResult<()> {
    logger::init(args.verbose)?;

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

        debug!("Opening cover file");
        let mut image_bytes: Vec<u8> = Vec::new();
        File::open(&path)
            .and_then(|mut file| file.read_to_end(&mut image_bytes))
            .map_err(|err| {
                CliError::from(err)
                    .context(format!("failed to open cover image: {:?}", path.display()))
            })?;

        let (extension, mime_type, dimensions) = match (
            imagesize::image_type(&image_bytes),
            imagesize::blob_size(&image_bytes),
        ) {
            (Ok(img_type), Ok(img_size)) => {
                use imagesize::ImageType;

                let dimensions = (img_size.width, img_size.height);
                let (extension, mime_type) = match img_type {
                    ImageType::Bmp => ("bmp", "image/bmp"),
                    ImageType::Gif => ("gif", "image/gif"),
                    ImageType::Jpeg => ("jpg", "image/jpeg"),
                    ImageType::Png => ("png", "image/png"),
                    ImageType::Webp => ("webp", "image/webp"),
                    _ => Err(CliError::from(format!(
                        "invalid format for cover image: {:?}",
                        img_type
                    )))?,
                };

                debug!("Cover image format: {:?}", extension);
                debug!("Cover image size: {:?}", dimensions);

                (extension, mime_type, dimensions)
            }
            (Err(err), _) => Err(CliError::from(err).context(format!(
                "failed to recognize cover image format: {:?}",
                path.display()
            )))?,
            (_, Err(err)) => Err(CliError::from(err).context(format!(
                "failed to get cover image dimensions: {:?}",
                path.display()
            )))?,
        };

        let href = format!("img/cover.{}", extension);

        debug!("Adding cover resources to EPUB");
        epub.add_cover_image(&href, image_bytes.as_slice(), mime_type)?;
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
            EpubContent::new(
                "content/cover.xhtml",
                coverpage_content(&href, dimensions).as_bytes(),
            )
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
        let content = read_to_string(&path).map_err(|err| {
            CliError::from(err).context(format!("failed to read input file: {:?}", path.display()))
        })?;

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

        if line_parser.is_spoiler_open() {
            warn!(
                "Input file has a spoiler that hasn't been closed and extended to the end of the file: {:?}",
                style(path.display()).bold(),
            );
        }

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
        .open(&args.output)
        .map_err(|err| {
            CliError::from(err).context(format!("failed to create output file: {:?}", &args.output))
        })?;

    epub.generate(&mut output_file)
        .map_err(|err| CliError::from(err).context("failed to generate EPUB"))?;

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

fn main() {
    let args = Args::parse();

    if let Err(err) = run(args) {
        error!("{}", err);
        std::process::exit(1)
    }
}
