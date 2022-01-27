# green2epub

Create an EPUB from text files in greentext format.

## Why?

Because:
- I don't like sitting in front of computer to read stories and prefer using a portable device to do so.
- I want to keep the little highlighting/formatting that a greentext has while doing so.

It first started with a messy [zero dependency python script][python-script] using the generated EPUBs of [fimfic2epub] as a base, which inspired the name. Then I decided to port it to Rust and here we are.

## Usage

You can use `green2epub --help` to get a list of all the available flags:

```
green2epub 0.1.0
Create an EPUB from text files in greentext format

USAGE:
    green2epub [OPTIONS] --title <TITLE> --author <AUTHOR> --output <PATH> [FILE]...

ARGS:
    <FILE>...
            Text files in greentext format to convert

OPTIONS:
    -t, --title <TITLE>
            Title of the greentext

    -a, --author <AUTHOR>
            Name of the author

    -c, --cover <FILE>
            Cover image to use

    -s, --subject <SUBJECT>
            Greentext subjects/tags.

            Can be used multiple times to set more than one.

            [aliases: tag]

    -o, --output <PATH>
            Path for the generated epub file

        --green-color <COLOR>
            Color of the green highlight

            [default: #2CAF26]

        --spoiler-color <COLOR>
            Color of the spoiler highlight

            [default: #000]

    -v, --verbose
            Shows verbose output, can be used multiple times to set level of verbosity

    -q, --quiet
            Supress all output

        --color <WHEN>
            When to use colors

            [default: auto]
            [possible values: auto, always, never]

    -h, --help
            Print help information

    -V, --version
            Print version information
```

### Examples

Let's suppose that `001-paste-author.txt`, `002-paste-author.txt` and `003-paste-author.txt` are the text files that we want as an EPUB. We need to provide minimum the `--title`, `--author` and `--output` flags besides the files themselves in the order we want them to be like so:

```sh
  green2epub --title "Paste" --author "Author" --output "Author - Paste.epub" \
    001-paste-author.txt 002-paste-author.txt 003-paste-author.txt
```

Creating our EPUB file as `Author - Paste.epub` in the current directory.

Now let's imagine that we have `paste-author-cover.png` that we'd like to use as a cover for the EPUB, passing in the `--cover` flag allows us to do that:

```sh
  green2epub --title "Paste" --author "Author" --output "Author - Paste.epub" \
    --cover paste-author-cover.png 001-paste-author.txt 002-paste-author.txt 003-paste-author.txt
```

Maybe you ~~need~~ like having subjects on EPUBs, for that you can provide the `--subject`/`--tag` flag as many times as needed like so:

```sh
  green2epub --title "Paste" --author "Author" --output "Author - Paste.epub" \
    --cover paste-author-cover.png --subject "SFW" --subject "Comedy" --subject "Romance" \
    001-paste-author.txt 002-paste-author.txt 003-paste-author.txt
```

## Installation

<!--
### Release

TODO
-->

### Build from source

Firstly, Rust should be installed in your system. Instructions on how to do so can be found [on its website](https://www.rust-lang.org/tools/install).

Then you need to clone this repository and build it using `cargo` like so:

```sh
  git clone https://github.com/ZodiacalComet/green2epub.git
  cd green2epub
  cargo build --release
```

The resulting binary will be located in `target/releases`.

## Acknowledgments

To [fimfic2epub] by [@daniel-j] which, besides being used as a base, greatly inspired this little project. It's excellent if you want [Fimfiction stories][fimfiction] as a proper EPUB, so check it out!

And to these other awesome projects that made this one less of a pain to make!

- [clap], that powers the CLI with it excellent argument parser.
- [epub-builder], that handles the actual EPUB generation.
- [html-escape], ensures that the content is properly HTML escaped.
- [imagesize], that recognizes the cover image formats and its dimensions.
- [log] for its simple API to control the application output.
- [humantime], that formats the time for the verbose output.
- [console] for its easy-to-use cross-platform abstractions over terminal text formatting.
- [indicatif], that provides the progress indicator used on the application.

## License

The contents of the `static` directory are a part of [fimfic2epub's stylesheets][fimfic2epub-styles], which are under the MIT License.

The rest of the project is under the [Unlicense License](LICENSE).

[python-script]: https://gist.github.com/ZodiacalComet/aea3ef9f48ab710c202dec6bbe6b1ff4

[fimfic2epub]: https://github.com/daniel-j/fimfic2epub
[fimfic2epub-styles]: https://github.com/daniel-j/fimfic2epub/tree/master/src/style
[@daniel-j]: https://github.com/daniel-j

[fimfiction]: https://www.fimfiction.net/

[epub-builder]: https://github.com/lise-henry/epub-builder
[clap]: https://github.com/clap-rs/clap
[html-escape]: https://github.com/magiclen/html-escape
[imagesize]: https://github.com/Roughsketch/imagesize
[log]: https://github.com/rust-lang/log
[humantime]: https://github.com/tailhook/humantime
[console]: https://github.com/mitsuhiko/console
[indicatif]: https://github.com/console-rs/indicatif
