use collections::string::String;

use core::hash::{Hash, Hasher};

use uuid::Uuid;
use hash_map::HashMap;
use insert::Insert;
use shared::Shared;

use attribute::Attribute;


#[derive(Debug, PartialEq, Eq)]
pub struct GeometryData {
    uuid: Uuid,
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
                uuid: Uuid::new_v4(),
                attributes: HashMap::new(),
                dirty: false,
            })
        }
    }

    pub fn get_uuid(&self) -> &Uuid {&self.data.uuid}

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

impl Hash for Geometry {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
         (&*self.data as *const _).hash(state);
    }
}

impl PartialEq<Geometry> for Geometry {
    fn eq(&self, other: &Geometry) -> bool {
        self.data.eq(&*other.data)
    }
}
impl Eq for Geometry {}
