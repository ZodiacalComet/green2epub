use crate::tag::Tag;

pub const RESET_FOREGROUND_CLASS: &str = "icolor";
const SPOILER_OPEN_TAG: &str = "[spoiler]";
const SPOILER_CLOSE_TAG: &str = "[/spoiler]";

#[derive(Debug)]
enum Token {
    SpoilerOpen,
    SpoilerClose,
    Text(String),
}

macro_rules! tokenize_spoiler {
    ($ref:expr, $idx:expr, $tokens:expr, $line:ident, $token_kind:ident) => {
        // Push the text before the tag if any.
        if $idx != $ref {
            $tokens.push(Token::Text($line[$ref..$idx].into()));
        }
        $tokens.push(Token::$token_kind);
    };
    ($idx:expr, $tokens:expr, $line:ident, $token_kind:ident) => {
        tokenize_spoiler!(0, $idx, $tokens, $line, $token_kind);
    };
}

macro_rules! push_line_buffer {
    ($line:ident, $idx:expr, $tag:expr) => {
        $line = &$line[$idx + $tag.len()..];
    };
}

fn tokenize<S: ?Sized>(line: &S) -> Vec<Token>
where
    S: AsRef<str>,
{
    let mut line = line.as_ref();
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        match (line.find(SPOILER_OPEN_TAG), line.find(SPOILER_CLOSE_TAG)) {
            (Some(start_idx), Some(end_idx)) => {
                // While both tags are in the same line, they could be in any order.
                if start_idx < end_idx {
                    tokenize_spoiler!(start_idx, tokens, line, SpoilerOpen);
                    tokenize_spoiler!(
                        start_idx + SPOILER_OPEN_TAG.len(),
                        end_idx,
                        tokens,
                        line,
                        SpoilerClose
                    );
                    push_line_buffer!(line, end_idx, SPOILER_CLOSE_TAG);
                } else {
                    tokenize_spoiler!(end_idx, tokens, line, SpoilerClose);
                    tokenize_spoiler!(
                        end_idx + SPOILER_CLOSE_TAG.len(),
                        start_idx,
                        tokens,
                        line,
                        SpoilerOpen
                    );
                    push_line_buffer!(line, start_idx, SPOILER_OPEN_TAG);
                }
            }
            (Some(idx), None) => {
                tokenize_spoiler!(idx, tokens, line, SpoilerOpen);
                push_line_buffer!(line, idx, SPOILER_OPEN_TAG);
            }
            (None, Some(idx)) => {
                tokenize_spoiler!(idx, tokens, line, SpoilerClose);
                push_line_buffer!(line, idx, SPOILER_CLOSE_TAG);
            }
            (None, None) => break,
        };
    }

    if !line.is_empty() {
        tokens.push(Token::Text(line.into()))
    }

    tokens
}

macro_rules! push_span_tag {
    ($paragraph:ident, $text:expr) => {
        let mut spoiler = Tag::new("span");
        spoiler.child($text);
        $paragraph.child(spoiler);
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

    // FIX: Merge sequential span tags together.
    pub fn parse<S: ?Sized>(&mut self, line: &S) -> Tag
    where
        S: AsRef<str>,
    {
        let mut paragraph = Tag::new("p");
        let mut is_first_text = true;

        for token in tokenize(line) {
            // Remove highlight if it doesn't apply to the given line
            if is_first_text {
                if let Token::Text(text) = &token {
                    if !text.starts_with('>') {
                        paragraph.attribute("class", RESET_FOREGROUND_CLASS);
                    }
                    is_first_text = false;
                }
            }

            match token {
                Token::Text(text) if self.open_spoiler => {
                    push_span_tag!(paragraph, text);
                }
                Token::Text(text) => {
                    paragraph.child(text);
                }
                // Place the tag as is if `open_spoiler` would stay the same.
                Token::SpoilerOpen if self.open_spoiler => {
                    push_span_tag!(paragraph, SPOILER_OPEN_TAG);
                }
                Token::SpoilerClose if !self.open_spoiler => {
                    push_span_tag!(paragraph, SPOILER_CLOSE_TAG);
                }
                Token::SpoilerOpen => self.open_spoiler = true,
                Token::SpoilerClose => self.open_spoiler = false,
            };
        }

        paragraph
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
    fn several_spoilers_in_a_single_line() {
        let mut parser = LineParser::default();

        assert_parse!(
            parser,
            tag!(
                spoiler!("This is a paragraph"),
                " with ",
                spoiler!("two spoilers")
            ),
            "[spoiler]This is a paragraph[/spoiler] with [spoiler]two spoilers[/spoiler]"
        );
        assert_parse!(
            parser,
            tag!(spoiler!("This is"), " a ", spoiler!("paragraph"), " with ", spoiler!("three spoilers")),
            "[spoiler]This is[/spoiler] a [spoiler]paragraph[/spoiler] with [spoiler]three spoilers[/spoiler]"
        );
        assert_parse!(
            parser,
            tag!(spoiler!("This is"), " a ", spoiler!("paragraph"), " with ", spoiler!("four"), " ", spoiler!("spoilers")),
            "[spoiler]This is[/spoiler] a [spoiler]paragraph[/spoiler] with [spoiler]four[/spoiler] [spoiler]spoilers[/spoiler]"
        );
    }

    #[test]
    fn several_spoilers_multiple_lines() {
        let mut parser = LineParser::default();

        assert_parse!(
            spoiler,
            parser,
            tag!(
                "The ",
                spoiler!("spoiler"),
                " starts on ",
                spoiler!("this line")
            ),
            "The [spoiler]spoiler[/spoiler] starts on [spoiler]this line"
        );
        assert_parse!(
            spoiler,
            parser,
            tag!(spoiler!("then ends on the following one"), ", next is ", spoiler!("a middle one"), " and then ", spoiler!("it starts again")),
            "then ends on the following one[/spoiler], next is [spoiler]a middle one[/spoiler] and then [spoiler]it starts again"
        );
        assert_parse!(
            parser,
            tag!(spoiler!("to end"), " and never come back again"),
            "to end[/spoiler] and never come back again"
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
