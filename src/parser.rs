use nom;

use std::mem;
use std::str;

use ast::{BlockLevel, BodyElement, Fragment, Inline};

/*

named!(pub document<Fragment>, do_parse!(
    elements: many0!(body_elem) >>
    (Fragment { elements })
));

named!(body_elem<BlockLevel>, do_parse!(
    para: map_res!(take_until!("\n\n"), str::from_utf8) >>
    many1!(tag_s!("\n")) >>
    ({ println!("PARA {:?}", para); BlockLevel::BodyElement(BodyElement::Paragraph(para)) })
));


*/


pub fn document(s: &str) -> Fragment {
    let mut bytes = s.as_bytes();
    let mut blocks = vec![];
    let mut block = vec![];
    let mut state = State::Next;
    loop {

        let line = match bytes.iter().position(|b| *b == b'\n') {
            Some(i) => &bytes[..i + 1],
            None => &bytes[..],
        };

        if line[0] != b'\n' {
            state = State::Para;
            block.push(Inline::Text(str::from_utf8(line).unwrap()));
        }

        if line[0] == b'\n' || bytes.len() == line.len() {
            match state {
                State::Para => {
                    let mut new = vec![];
                    mem::swap(&mut new, &mut block);
                    if let Some(last) = new.last_mut() {
                        let mut trimmed = last.trim_right();
                        mem::swap(last, trimmed);
                    }
                    let elem = BodyElement::Paragraph(new);
                    println!("para-exit {:?}", elem);
                    blocks.push(BlockLevel::BodyElement(elem));
                },
                State::Next => {},
            }
        }
        bytes = &bytes[line.len()..];

        // If `bytes` is empty, we're done.
        if bytes.is_empty() {
            break;
        }

    }

    

    Fragment { elements: blocks }
}

#[derive(Debug)]
enum State {
    Next,
    Para,
}

#[cfg(test)]
mod tests {
    use super::document;
    use ast::{BlockLevel, BodyElement, Fragment, Inline};
    use nom::IResult;
    #[test]
    fn test_para() {
        assert_eq!(
            document("foo\n\nbar"),
            Fragment {
                elements: vec![
                    BlockLevel::BodyElement(BodyElement::Paragraph(vec![Inline::Text("foo")])),
                    BlockLevel::BodyElement(BodyElement::Paragraph(vec![Inline::Text("bar")])),
                ]
            },
        );
    }
}
