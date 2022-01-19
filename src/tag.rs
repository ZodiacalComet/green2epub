use std::fmt;

use html_escape::{encode_double_quoted_attribute, encode_text};

#[derive(Debug, Clone, PartialEq)]
pub enum Child {
    Tag(Tag),
    Text(String),
}

impl From<Tag> for Child {
    fn from(tag: Tag) -> Child {
        Child::Tag(tag)
    }
}

impl From<&Tag> for Child {
    fn from(tag: &Tag) -> Child {
        Child::Tag(tag.clone())
    }
}

impl From<&mut Tag> for Child {
    fn from(tag: &mut Tag) -> Child {
        Child::Tag(tag.clone())
    }
}

impl From<String> for Child {
    fn from(text: String) -> Child {
        Child::Text(text)
    }
}

impl From<&str> for Child {
    fn from(text: &str) -> Child {
        Child::Text(text.into())
    }
}

impl fmt::Display for Child {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Child::Tag(tag) => write!(f, "{}", tag),
            Child::Text(text) => write!(f, "{}", encode_text(text)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Child>,
}

impl Tag {
    pub fn new<S>(name: S) -> Self
    where
        S: ToString,
    {
        Self {
            name: name.to_string(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn attribute<N, V>(&mut self, name: N, value: V) -> &mut Self
    where
        N: ToString,
        V: ToString,
    {
        self.attributes.push((name.to_string(), value.to_string()));
        self
    }

    pub fn child<C>(&mut self, child: C) -> &mut Self
    where
        C: Into<Child>,
    {
        self.children.push(child.into());
        self
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}", self.name)?;

        for (attr, value) in &self.attributes {
            write!(f, " {}=\"{}\"", attr, encode_double_quoted_attribute(value))?;
        }

        if self.children.is_empty() {
            return write!(f, "/>");
        }

        write!(f, ">")?;

        for child in &self.children {
            write!(f, "{}", child)?;
        }

        write!(f, "</{}>", self.name)
    }
}
