use collections::string::String;


#[derive(Debug)]
pub struct Bone {
    pub parent_index: usize,
    pub name: String,
    pub skinned: bool,
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
    pub bind_pose: [f32; 16],
}

impl Bone {
    pub fn new(
        parent_index: usize,
        name: &str,
        skinned: bool,
        position: [f32; 3],
        rotation: [f32; 4],
        scale: [f32; 3],
        bind_pose: [f32; 16],
    ) -> Self {
        Bone {
            parent_index: parent_index,
            name: String::from(name),
            skinned: skinned,
            position: position,
            rotation: rotation,
            scale: scale,
            bind_pose: bind_pose,
        }
    }
}
