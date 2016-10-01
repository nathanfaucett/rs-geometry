use collections::string::String;
use collections::btree_map::BTreeMap;

use shared::Shared;

use attribute::Attribute;


#[derive(Debug)]
pub struct GeometryData<'a> {
    attributes: BTreeMap<String, Attribute<'a>>,
    dirty: bool,
}

#[derive(Clone)]
pub struct Geometry<'a> {
    data: Shared<GeometryData<'a>>,
}

impl<'a> Geometry<'a> {

    pub fn new() -> Self {
        Geometry {
            data: Shared::new(GeometryData {
                attributes: BTreeMap::new(),
                dirty: false,
            })
        }
    }

    pub fn add_attribute(&mut self, attribute: Attribute<'a>) -> &mut Self {
        self.data.attributes.insert(attribute.name.clone(), attribute);
        self
    }

    pub fn get_attribute(&self, name: &str) -> Option<&Attribute<'a>> {self.data.attributes.get(name)}
    pub fn get_attribute_mut(&mut self, name: &str) -> Option<&mut Attribute<'a>> {self.data.attributes.get_mut(name)}

    pub fn get_attributes(&self) -> &BTreeMap<String, Attribute<'a>> {&self.data.attributes}
    pub fn get_attributes_mut(&mut self) -> &mut BTreeMap<String, Attribute<'a>> {&mut self.data.attributes}

    pub fn get_dirty(&self) -> bool {self.data.dirty}
    pub fn set_dirty(&mut self, dirty: bool) -> &mut Self {
        self.data.dirty = dirty;
        self
    }
}
