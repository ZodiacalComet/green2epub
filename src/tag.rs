use std::fmt;

use html_escape::encode_text;

#[derive(Clone)]
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

// TODO: Is there a better name for `WithValue` variant?
#[derive(Clone)]
pub enum Attribute {
    WithValue(String, String),
    Boolean(String),
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Attribute::WithValue(attr, value) => write!(f, "{}=\"{}\"", attr, value),
            Attribute::Boolean(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Clone)]
pub struct Tag {
    name: String,
    attributes: Vec<Attribute>,
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
        self.attributes
            .push(Attribute::WithValue(name.to_string(), value.to_string()));
        self
    }

    pub fn boolean_attribute<N>(&mut self, name: N) -> &mut Self
    where
        N: ToString,
    {
        self.attributes.push(Attribute::Boolean(name.to_string()));
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

        for attribute in &self.attributes {
            write!(f, " {}", attribute)?;
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
