use collections::string::String;
use collections::vec::Vec;
use collections::btree_map::BTreeMap;

use shared::Shared;

use attribute::Attribute;
use bone::Bone;


#[derive(Debug)]
pub struct GeometryData<'a> {
    attributes: BTreeMap<String, Attribute<'a>>,
    bones: Vec<Bone>,
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
                bones: Vec::new(),
            })
        }
    }

    pub fn add_attribute(&mut self, attribute: Attribute<'a>) -> &mut Self {
        self.data.attributes.insert(attribute.get_name(), attribute);
        self
    }
    pub fn add_bone(&mut self, bone: Bone) -> &mut Self {
        self.data.bones.push(bone);
        self
    }

    pub fn get_attribute(&self, name: &str) -> Option<&Attribute<'a>> {self.data.attributes.get(name)}
    pub fn get_attribute_mut(&mut self, name: &str) -> Option<&mut Attribute<'a>> {self.data.attributes.get_mut(name)}

    pub fn get_attributes(&self) -> &BTreeMap<String, Attribute<'a>> {&self.data.attributes}
    pub fn get_attributes_mut(&mut self) -> &mut BTreeMap<String, Attribute<'a>> {&mut self.data.attributes}

    pub fn get_bone(&self, index: usize) -> Option<&Bone> {self.data.bones.get(index)}
    pub fn get_bone_mut(&mut self, index: usize) -> Option<&mut Bone> {self.data.bones.get_mut(index)}

    pub fn get_bones(&self) -> &Vec<Bone> {&self.data.bones}
    pub fn get_bones_mut(&mut self) -> &mut Vec<Bone> {&mut self.data.bones}
}
