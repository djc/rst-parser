use ast::{BlockLevel, BodyElement, Fragment, Inline};

pub struct Html5Writer {
    buf: String,
}

impl Html5Writer {
    fn visit(&mut self, fragment: Fragment) {
        for block in fragment.elements.iter() {
            self.visit_block(block);
        }
    }

    fn visit_block(&mut self, block: &BlockLevel) {
        use ast::BlockLevel::*;
        match *block {
            BodyElement(ref body) => self.visit_body_elem(body),
            Section => {},
            Transition => {},
        }
    }

    fn visit_body_elem(&mut self, body: &BodyElement) {
        use ast::BodyElement::*;
        match *body {
            Paragraph(ref items) => self.visit_paragraph(items),
        }
    }

    fn visit_paragraph(&mut self, items: &[Inline]) {
        self.buf.push_str("<p>");
        for item in items.iter() {
            match *item {
                Inline::Text(s) => self.buf.push_str(s),
            }  
        }
        self.buf.push_str("</p>\n");
    }
}

impl Writer for Html5Writer {
    fn translate(fragment: Fragment) -> String {
        let mut writer = Self { buf: String::new() };
        writer.visit(fragment);
        let Self { buf } = writer;
        buf
    }
}

pub trait Writer {
    fn translate(fragment: Fragment) -> String;
}
