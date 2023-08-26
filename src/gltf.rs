use std::fmt;

use nanoserde::DeJson;

use std::collections::HashMap;

// a helper for Debug impls
fn debug_trim_string(uri: &str) -> String {
    if uri.len() > 30 {
        format!("{}.., total length: {}", &uri[0..30], uri.len())
    } else {
        uri.to_string()
    }
}

#[derive(DeJson, PartialEq, Debug)]
#[nserde(transparent)]
pub struct GlU32(u32);

#[derive(DeJson, PartialEq, Debug)]
pub struct Gltf {
    #[nserde(default)]
    pub accessors: Vec<Accessor>,
    #[nserde(default)]
    pub assets: Vec<Asset>,
    #[nserde(default)]
    pub buffers: Vec<Buffer>,
    #[nserde(rename = "bufferViews")]
    #[nserde(default)]
    pub buffer_views: Vec<BufferView>,
    #[nserde(default)]
    pub images: Vec<Image>,
    #[nserde(default)]
    pub scenes: Vec<Scene>,
    #[nserde(default)]
    pub materials: Vec<Material>,
    #[nserde(default)]
    pub meshes: Vec<Mesh>,
    #[nserde(default)]
    pub nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum ComponentType {
    Byte = 5120,
    UnsignedByte = 5121,
    Short = 5122,
    UnsignedShort = 5123,
    UnsignedInt = 5125,
    Float = 5126,
}
impl ComponentType {
    pub fn byte_size(&self) -> usize {
        use ComponentType::*;

        match self {
            Byte | UnsignedByte => 1,
            Short | UnsignedShort => 2,
            UnsignedInt => 4,
            Float => 4,
        }
    }
}

impl From<&GlU32> for ComponentType {
    fn from(n: &GlU32) -> ComponentType {
        match n.0 as u32 {
            x if x == ComponentType::Byte as u32 => ComponentType::Byte,
            x if x == ComponentType::UnsignedByte as u32 => ComponentType::UnsignedByte,
            x if x == ComponentType::Short as u32 => ComponentType::Short,
            x if x == ComponentType::UnsignedShort as u32 => ComponentType::UnsignedShort,
            x if x == ComponentType::UnsignedInt as u32 => ComponentType::UnsignedInt,
            x if x == ComponentType::Float as u32 => ComponentType::Float,
            x => panic!("Not a ComponentType u32!: {x}"),
        }
    }
}

#[derive(DeJson, PartialEq, Debug)]
pub struct Accessor {
    #[nserde(rename = "bufferView")]
    pub buffer_view: Option<usize>,
    #[nserde(rename = "byteOffset")]
    #[nserde(default = 0)]
    pub byte_offset: usize,
    #[nserde(rename = "componentType")]
    #[nserde(proxy = "GlU32")]
    pub component_type: ComponentType,
    #[nserde(default = "false")]
    pub normalized: bool,
    pub count: usize,
    pub max: Option<Vec<f64>>,
    pub min: Option<Vec<f64>>,
    pub sparse: Option<Sparse>,
    pub name: Option<String>,
    #[nserde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct SparseIndices {
    #[nserde(rename = "bufferView")]
    pub buffer_view: usize,
    #[nserde(rename = "byteOffset")]
    pub byte_fffset: usize,
    #[nserde(rename = "componentType")]
    pub component_type: u32,
}

#[derive(DeJson, PartialEq, Debug)]
struct SparseValues {
    #[nserde(rename = "bufferView")]
    pub buffer_view: usize,
    #[nserde(rename = "byteOffset")]
    pub byte_offset: usize,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct Sparse {
    count: usize,
    indices: SparseIndices,
    values: SparseValues,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct Asset {
    pub copyright: Option<String>,
    pub generator: Option<String>,
    pub version: String,
    #[nserde(rename = "minVersion")]
    pub min_version: Option<String>,
}

#[derive(DeJson, PartialEq)]
pub struct Buffer {
    pub uri: String,
    #[nserde(rename = "byteLength")]
    pub byte_length: usize,
    pub name: Option<String>,
}
impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Buffer")
            .field("uri", &debug_trim_string(&self.uri))
            .field("byte_length", &self.byte_length)
            .field("name", &self.name)
            .finish()
    }
}

pub enum BufferViewTarget {
    ArrayBuffer = 34962,
    ElementArrayBuffer = 34963,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct BufferView {
    pub buffer: usize,
    #[nserde(default)]
    #[nserde(rename = "byteOffset")]
    pub byte_offset: usize,
    #[nserde(rename = "byteLength")]
    pub byte_length: usize,
    pub stride: Option<usize>,
    pub target: Option<u32>,
    pub name: Option<String>,
}

#[derive(DeJson, PartialEq)]
pub struct Image {
    pub uri: Option<String>,
    #[nserde(rename = "mimeType")]
    pub mime_type: Option<String>,
    #[nserde(rename = "bufferView")]
    pub buffer_view: Option<usize>,
    pub name: Option<String>,
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Image")
            .field("uri", &self.uri.as_ref().map(|s| debug_trim_string(s)))
            .field("mime_type", &self.mime_type)
            .field("name", &self.name)
            .finish()
    }
}

#[derive(DeJson, PartialEq, Debug)]
pub struct PBRMetallicRoughness {
    #[nserde(default = "[1.0, 1.0, 1.0, 1.0]")]
    #[nserde(rename = "baseColorFactor")]
    pub base_color_factor: [f64; 4],
    #[nserde(rename = "baseColorTexture")]
    pub base_color_texture: Option<BaseColorTexture>,
    #[nserde(default = 1.0)]
    #[nserde(rename = "metallicFactor")]
    pub metallic_factor: f64,
    #[nserde(default = 1.0)]
    #[nserde(rename = "roughnessFactor")]
    pub roughness_factor: f64,
    #[nserde(rename = "metallicRoughnessTexture")]
    pub metallic_roughness_texture: Option<MetallicRoughnessTexture>,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct BaseColorTexture {
    pub index: usize,
    #[nserde(rename = "texCoord")]
    #[nserde(default = 0)]
    pub tex_coord: usize,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct NormalTexture {
    pub index: usize,
    #[nserde(rename = "texCoord")]
    #[nserde(default = 0)]
    pub tex_coord: usize,
    #[nserde(default = 1.0)]
    pub scale: f64,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct OcclusionTexture {
    pub index: usize,
    #[nserde(rename = "texCoord")]
    #[nserde(default = 0)]
    pub tex_coord: usize,
    #[nserde(default = 1.0)]
    pub strength: f64,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct EmissiveTexture {
    pub index: usize,
    #[nserde(rename = "texCoord")]
    #[nserde(default = 0)]
    pub tex_coord: usize,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct MetallicRoughnessTexture {
    pub index: usize,
    #[nserde(rename = "texCoord")]
    #[nserde(default = 0)]
    pub tex_coord: usize,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct Material {
    pub name: Option<String>,
    #[nserde(rename = "pbrMetallicRoughness")]
    pub pbr_metallic_roughness: PBRMetallicRoughness,
    #[nserde(rename = "normalTexture")]
    pub normal_texture: Option<NormalTexture>,
    #[nserde(rename = "occlusionTexture")]
    pub occlusion_texture: Option<OcclusionTexture>,
    #[nserde(rename = "emissiveTexture")]
    pub emissive_texture: Option<EmissiveTexture>,
    #[nserde(rename = "emissiveFactor")]
    #[nserde(default = "[0.0, 0.0, 0.0]")]
    pub emissive_factor: [f64; 3],
    #[nserde(rename = "alphaMode")]
    #[nserde(default)]
    pub alpha_mode: u32,
    #[nserde(rename = "alphaCutoff")]
    #[nserde(default = "0.5")]
    pub alpha_cutoff: f64,
    #[nserde(rename = "doubleSided")]
    #[nserde(default = "false")]
    pub double_sided: bool,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct Mesh {
    #[nserde(default)]
    pub primitives: Vec<Primitive>,
    pub weights: Option<Vec<f64>>,
    pub name: Option<String>,
}

#[derive(Debug, PartialEq)]
#[repr(u32)]
pub enum PrimitiveMode {
    Points = 0,
    Lines = 1,
    LineLoop = 2,
    LineStrip = 3,
    Triangles = 4,
    TriangleStrip = 5,
    TriangleFan = 6,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct Primitive {
    #[nserde(default)]
    pub attributes: HashMap<String, usize>,
    pub indices: Option<usize>,
    pub material: Option<usize>,
    pub mode: Option<u32>,
    pub targets: Option<HashMap<String, usize>>,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct Node {
    pub camera: Option<usize>,
    #[nserde(default)]
    pub children: Vec<usize>,
    pub skin: Option<usize>,
    pub matrix: Option<[f64; 16]>,
    pub mesh: Option<usize>,
    pub rotation: Option<[f64; 4]>,
    pub scale: Option<[f64; 3]>,
    pub translation: Option<[f64; 3]>,
    pub weights: Option<Vec<f64>>,
    pub name: Option<String>,
}

#[repr(u32)]
pub enum Filter {
    Nearest = 9728,
    Linear = 9729,
    NearestMimpapNearest = 9984,
    LinearMipmapNearest = 9985,
    NearestMipmapLinear = 9986,
    LinearMipmapLinear = 9987,
}

#[repr(u32)]
pub enum WrappingMode {
    ClampToEdge = 33071,
    MirroredRepeat = 33648,
    Repeat = 10497,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct Sampler {
    #[nserde(rename = "magFilter")]
    pub mag_filter: Option<u32>,
    #[nserde(rename = "minFilter")]
    pub min_filter: Option<u32>,
    #[nserde(rename = "wrapS")]
    pub wrap_s: Option<u32>,
    #[nserde(rename = "wrapT")]
    pub wrap_t: Option<u32>,
    pub name: Option<String>,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct Scene {
    pub nodes: Vec<usize>,
    pub name: Option<String>,
}

#[derive(DeJson, PartialEq, Debug)]
pub struct Texture {
    pub sampler: Option<usize>,
    pub source: Option<usize>,
    pub name: Option<String>,
}
