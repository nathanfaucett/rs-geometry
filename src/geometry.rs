use collections::string::String;

use hash_map::HashMap;
use insert::Insert;
use shared::Shared;

use attribute::Attribute;


#[derive(Debug)]
pub struct GeometryData {
    attributes: HashMap<String, Attribute>,
    dirty: bool,
}

#[derive(Clone)]
pub struct Geometry {
    data: Shared<GeometryData>,
}

impl Geometry {

    pub fn new() -> Self {
        Geometry {
            data: Shared::new(GeometryData {
                attributes: HashMap::new(),
                dirty: false,
            })
        }
    }

    pub fn add_attribute(&mut self, attribute: Attribute) -> &mut Self {
        self.data.attributes.insert(attribute.name.clone(), attribute);
        self
    }

    pub fn get_attribute(&self, name: &str) -> Option<&Attribute> {self.data.attributes.get(name)}
    pub fn get_attribute_mut(&mut self, name: &str) -> Option<&mut Attribute> {self.data.attributes.get_mut(name)}

    pub fn get_attributes(&self) -> &HashMap<String, Attribute> {&self.data.attributes}
    pub fn get_attributes_mut(&mut self) -> &mut HashMap<String, Attribute> {&mut self.data.attributes}

    pub fn get_dirty(&self) -> bool {self.data.dirty}
    pub fn set_dirty(&mut self, dirty: bool) -> &mut Self {
        self.data.dirty = dirty;
        self
    }
}
