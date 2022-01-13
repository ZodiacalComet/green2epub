mod tag;

use tag::{Child, Tag};

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

        format!(
            "<?xml version='1.0' encoding='utf-8'?>\
                <!DOCTYPE html>\
                {}",
            html
        )
    }
}

fn main() {
    let mut paste = PasteContent::new("A Title Here");
    paste
        .add_line("This is a line")
        .add_line(Tag::new("p").child("This is a paragraph"))
        .add_line(
            Tag::new("div")
                .attribute("class", "some-container center")
                .boolean_attribute("disabled")
                .child("A div element"),
        );

    let xhtml_content = paste.build();

    println!("{}", xhtml_content);
}
