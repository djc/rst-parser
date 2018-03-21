use nom;

use std::str;

use ast::{BlockLevel, BodyElement, Fragment};

named!(pub document<Fragment>, do_parse!(
    elements: many0!(body_elem) >>
    (Fragment { elements })
));

named!(body_elem<BlockLevel>, do_parse!(
    para: map_res!(nom::not_line_ending, str::from_utf8) >>
    many0!(nom::line_ending) >>
    (BlockLevel::BodyElement(BodyElement::Paragraph(para)))
));

#[cfg(test)]
mod tests {
    use super::document;
    use ast::{BlockLevel, BodyElement, Fragment};
    use nom::IResult;
    #[test]
    fn test_para() {
        match document(b"foo\n\nbar") {
            IResult::Done(_, res) => assert_eq!(
                res,
                Fragment { elements: vec![
                    BlockLevel::BodyElement(BodyElement::Paragraph("foo")),
                    BlockLevel::BodyElement(BodyElement::Paragraph("bar")),
                ] }
            ),
            err => panic!("parser failed: {:?}", err),
        }
    }
}
