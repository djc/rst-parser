#[derive(Debug, PartialEq)]
pub struct Fragment<'a> {
    pub elements: Vec<BlockLevel<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum BlockLevel<'a> {
    BodyElement(BodyElement<'a>),
    Section,
    Transition,
}

#[derive(Debug, PartialEq)]
pub enum BodyElement<'a> {
    Paragraph(Vec<Inline<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum Inline<'a> {
    Text(&'a str),
}
