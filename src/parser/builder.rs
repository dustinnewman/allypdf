use std::collections::BTreeMap;
use crate::document::document::{ObjectMap, PDFDocument, Version};
use super::parser::{Dictionary, IndirectObject, IndirectReference, Object, Trailer, XrefSection};

pub struct Builder<'a> {
    objects: &'a Vec<Object>,
}

impl<'a> Builder<'a> {
    pub fn new(objects: &'a Vec<Object>) -> Self {
        Self {
            objects
        }
    }

    pub fn build(&self) -> Option<PDFDocument<'a>> {
        let version: Option<Version> = None;
        let trailer: Option<Trailer> = None;
        let xref_section: Option<XrefSection> = None;
        let start_xref = None;
        let object_map: ObjectMap = BTreeMap::new();
        for obj in self.objects.drain(..) {
            match obj {
                Object::Header(m, n) => version = Some((m, n)),
                Object::Trailer(t) => trailer = Some(t),
                Object::StartXref(s) => start_xref = Some(s),
                Object::IndirectObject(ind) => self.indirect_object(&mut object_map, ind),
                Object::Xref(x) => xref_section = Some(x),
                _ => continue,
            }
        }
        Some(PDFDocument {
            version: version?,
            trailer: trailer?,
            xref_table: xref_section?,
            start_xref: start_xref?,
            object_map,
            catalog: catalog?,
        })
    }

    fn indirect_object(&mut self, object_map: &mut ObjectMap, object: IndirectObject) {
        match *object.object {
            Object::Dictionary(dict) => self.indirect_dict(object_map, dict),
            Object::Stream(_) => todo!(),
            value => {
                let key = IndirectReference {
                    object_number: object.object_number,
                    generation_number: object.generation_number,
                };
                object_map.insert(key, value);
            }
        }
    }

    fn indirect_dict(&mut self, object_map: &mut ObjectMap, dict: Dictionary) {

    }
}