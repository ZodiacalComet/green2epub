use crate::tag::Tag;

pub const RESET_FOREGROUND_CLASS: &str = "icolor";
const SPOILER_START: &str = "[spoiler]";
const SPOILER_END: &str = "[/spoiler]";

macro_rules! close_spoiler {
    ($idx:expr, $spoiler_tag:ident, $paragraph_tag:ident, $line:ident) => {
        // Avoid appending empty spoilers
        if $idx != 0 {
            $spoiler_tag.child(&$line[..$idx]);
            $paragraph_tag.child($spoiler_tag);
        }

        // The spoiler could be closing at the end of the line, avoid appending it to the paragraph
        // on that case
        $line = &$line[$idx + SPOILER_END.len()..];
        if !$line.is_empty() {
            $paragraph_tag.child($line);
        }
    };
}

#[derive(Default)]
pub struct LineParser {
    open_spoiler: bool,
}

impl LineParser {
    pub fn is_spoiler_open(&self) -> bool {
        self.open_spoiler
    }

    pub fn parse<S: ?Sized>(&mut self, line: &S) -> Tag
    where
        S: AsRef<str>,
    {
        let line = line.as_ref();

        let mut paragraph_tag = Tag::new("p");
        let mut spoiler_tag = Tag::new("span");

        // Remove highlight if it doesn't apply to the given line
        let trimmed_line = line.trim_start();
        if !trimmed_line.starts_with('>')
            && !trimmed_line.starts_with(&format!("{}>", SPOILER_START))
            && !trimmed_line.starts_with(&format!("{}>", SPOILER_END))
        {
            paragraph_tag.attribute("class", RESET_FOREGROUND_CLASS);
        }

        // Parse spoilers
        if self.open_spoiler {
            let mut line = line;

            if let Some(idx) = line.find(SPOILER_END) {
                close_spoiler!(idx, spoiler_tag, paragraph_tag, line);
                self.open_spoiler = false;
            } else {
                // Here we are in a line that doesn't close the spoiler coming from lines back
                spoiler_tag.child(line);
                paragraph_tag.child(spoiler_tag);
            }
        } else if let Some(idx) = line.find(SPOILER_START) {
            let mut line = line;

            // Avoiding appending an empty text child at the start of the paragraph for spoilers at
            // the start of the line
            if idx != 0 {
                paragraph_tag.child(&line[..idx]);
            }

            line = &line[idx + SPOILER_START.len()..];
            if let Some(idx) = line.find(SPOILER_END) {
                close_spoiler!(idx, spoiler_tag, paragraph_tag, line);
            } else {
                // Here we are in a line with an unclosed spoiler, which could be starting at the
                // end of the line. Avoid appending on that case for this line.
                if !line.is_empty() {
                    spoiler_tag.child(line);
                    paragraph_tag.child(spoiler_tag);
                }

                self.open_spoiler = true;
            }
        } else {
            paragraph_tag.child(line);
        }

        paragraph_tag
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! tag {
        (hi, $($tag:expr),+) => {
            Tag::new("p")
                $( .child($tag) )+
        };
        ($($tag:expr),+) => {
            Tag::new("p")
                .attribute("class", RESET_FOREGROUND_CLASS)
                $( .child($tag) )+
        }
    }

    macro_rules! spoiler {
        ($content: expr) => {
            Tag::new("span").child($content)
        };
    }

    macro_rules! assert_parse {
        (spoiler, $parser:ident, $expected:expr, $raw:expr) => {
            assert_eq!(&$parser.parse($raw), $expected);
            assert_eq!(
                $parser.open_spoiler, true,
                "expected `open_spoiler` attribute to be `true`: {:?}",
                $raw
            );
        };
        ($parser:ident, $expected:expr, $raw:expr) => {
            assert_eq!(&$parser.parse($raw), $expected);
            assert_eq!(
                $parser.open_spoiler, false,
                "expected `open_spoiler` attribute to be `false`: {:?}",
                $raw
            );
        };
    }

    #[test]
    fn highlight_lines() {
        let mut parser = LineParser::default();

        assert_parse!(
            parser,
            tag!("A paragraph without highlight"),
            "A paragraph without highlight"
        );
        assert_parse!(
            parser,
            tag!(hi, ">Line with highlight"),
            ">Line with highlight"
        );
    }

    #[test]
    fn single_line_spoiler() {
        let mut parser = LineParser::default();

        assert_parse!(
            parser,
            tag!(spoiler!("This is a spoiler paragraph")),
            "[spoiler]This is a spoiler paragraph[/spoiler]"
        );
        assert_parse!(
            parser,
            tag!("Starts normal ", spoiler!("and then there is a spoiler")),
            "Starts normal [spoiler]and then there is a spoiler[/spoiler]"
        );
        assert_parse!(
            parser,
            tag!(spoiler!("Starts with a spoiler"), " and ends normal"),
            "[spoiler]Starts with a spoiler[/spoiler] and ends normal"
        );
        assert_parse!(
            parser,
            tag!(
                "Starts normal ",
                spoiler!(", then a spoiler in the middle"),
                " and ends normal"
            ),
            "Starts normal [spoiler], then a spoiler in the middle[/spoiler] and ends normal"
        );
    }

    #[test]
    fn single_line_spoiler_with_highlight() {
        let mut parser = LineParser::default();

        assert_parse!(
            parser,
            tag!(hi, spoiler!(">This is a spoiler paragraph")),
            "[spoiler]>This is a spoiler paragraph[/spoiler]"
        );
        assert_parse!(
            parser,
            tag!(
                hi,
                ">Starts normal with '>' ",
                spoiler!("and then there is a spoiler")
            ),
            ">Starts normal with '>' [spoiler]and then there is a spoiler[/spoiler]"
        );
        assert_parse!(
            parser,
            tag!(
                hi,
                spoiler!(">Starts with a spoiler and '>' inside of it"),
                " and ends normal"
            ),
            "[spoiler]>Starts with a spoiler and '>' inside of it[/spoiler] and ends normal"
        );
        assert_parse!(
            parser,
            tag!(
                hi,
                ">Starts normal and with '>' ",
                spoiler!(", then a spoiler in the middle"),
                " and ends normal"
            ),
            ">Starts normal and with '>' [spoiler], then a spoiler in the middle[/spoiler] and ends normal"
        );

        assert_parse!(
            parser,
            tag!(
                hi,
                ">",
                spoiler!("Starts with a spoiler and '>' outside of it"),
                " and ends normal"
            ),
            ">[spoiler]Starts with a spoiler and '>' outside of it[/spoiler] and ends normal"
        );
    }

    #[test]
    fn multiple_line_spoiler() {
        let mut parser = LineParser::default();

        assert_parse!(
            spoiler,
            parser,
            tag!("Starts normal ", spoiler!("and then an spoiler opens")),
            "Starts normal [spoiler]and then an spoiler opens"
        );
        assert_parse!(
            spoiler,
            parser,
            tag!(spoiler!("The unclosed spoiler continues on this line")),
            "The unclosed spoiler continues on this line"
        );
        assert_parse!(
            parser,
            tag!(
                spoiler!("The spoiler ends here"),
                " and continues as normal"
            ),
            "The spoiler ends here[/spoiler] and continues as normal"
        );
    }

    #[test]
    fn multiple_line_spoiler_with_highlight() {
        let mut parser = LineParser::default();

        assert_parse!(
            spoiler,
            parser,
            tag!(
                hi,
                ">Starts normal and with '>' ",
                spoiler!("and then an spoiler opens")
            ),
            ">Starts normal and with '>' [spoiler]and then an spoiler opens"
        );
        assert_parse!(
            spoiler,
            parser,
            tag!(hi, spoiler!(">The unclosed spoiler continues on this line")),
            ">The unclosed spoiler continues on this line"
        );
        assert_parse!(
            parser,
            tag!(
                hi,
                spoiler!(">The spoiler ends here"),
                " and continues as normal"
            ),
            ">The spoiler ends here[/spoiler] and continues as normal"
        );

        // A closing tag at the start
        let _ = &parser.parse("[spoiler]");
        assert_parse!(
            parser,
            tag!(hi, ">Nothing here either"),
            "[/spoiler]>Nothing here either"
        );
    }

    #[test]
    fn unclosed_spoiler() {
        let mut parser = LineParser::default();

        assert_parse!(
            spoiler,
            parser,
            tag!(spoiler!("Oh no I lost my spoiler closing tag")),
            "[spoiler]Oh no I lost my spoiler closing tag"
        );
        assert_parse!(
            spoiler,
            parser,
            tag!(spoiler!(
                "Now I can't stop the [spoiler] from consuming everything"
            )),
            "Now I can't stop the [spoiler] from consuming everything"
        );
        assert_parse!(spoiler, parser, tag!(spoiler!("Help!")), "Help!");
    }

    #[test]
    fn empty_spoiler() {
        let mut parser = LineParser::default();

        // Single line
        assert_parse!(
            parser,
            tag!("Nothing here"),
            "[spoiler][/spoiler]Nothing here"
        );
        assert_parse!(
            parser,
            tag!("Nothing here ", ", nothing there"),
            "Nothing here [spoiler][/spoiler], nothing there"
        );
        assert_parse!(
            parser,
            tag!("Nothing there"),
            "Nothing there[spoiler][/spoiler]"
        );

        // In two lines
        assert_parse!(
            spoiler,
            parser,
            tag!("Nothing here"),
            "Nothing here[spoiler]"
        );
        assert_parse!(
            parser,
            tag!("Nothing here either"),
            "[/spoiler]Nothing here either"
        );
    }
}
