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
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    UnsignedInt,
    Float,
}
impl From<&u32> for ComponentType {
    fn from(n: &u32) -> ComponentType {
        match *n {
            5120 => ComponentType::Byte,
            5121 => ComponentType::UnsignedByte,
            5122 => ComponentType::Short,
            5123 => ComponentType::UnsignedShort,
            5125 => ComponentType::UnsignedInt,
            5126 => ComponentType::Float,
            x => panic!("Not a ComponentType u32!: {x}"),
        }
    }
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

#[derive(DeJson, PartialEq, Debug)]
pub struct Accessor {
    #[nserde(rename = "bufferView")]
    pub buffer_view: Option<usize>,
    #[nserde(rename = "byteOffset")]
    #[nserde(default = 0)]
    pub byte_offset: usize,
    #[nserde(rename = "componentType")]
    #[nserde(proxy = "u32")]
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

#[derive(PartialEq, Debug)]
pub enum BufferViewTarget {
    ArrayBuffer,
    ElementArrayBuffer,
}
impl From<&u32> for BufferViewTarget {
    fn from(n: &u32) -> BufferViewTarget {
        match *n {
            34962 => BufferViewTarget::ArrayBuffer,
            34963 => BufferViewTarget::ElementArrayBuffer,
            x => panic!("Not a BufferViewTarget u32! {x}"),
        }
    }
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
    #[nserde(proxy = "u32")]
    pub target: Option<BufferViewTarget>,
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
    pub alpha_mode: String,
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
    Points,
    Lines,
    LineLoop,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
}
impl From<&u32> for PrimitiveMode {
    fn from(n: &u32) -> PrimitiveMode {
        match *n {
            0 => PrimitiveMode::Points,
            1 => PrimitiveMode::Lines,
            2 => PrimitiveMode::LineLoop,
            3 => PrimitiveMode::LineStrip,
            4 => PrimitiveMode::Triangles,
            5 => PrimitiveMode::TriangleStrip,
            6 => PrimitiveMode::TriangleFan,
            x => panic!("Not a PrimitiveMode u32! {x}"),
        }
    }
}
#[derive(DeJson, PartialEq, Debug)]
pub struct Primitive {
    #[nserde(default)]
    pub attributes: HashMap<String, usize>,
    pub indices: Option<usize>,
    pub material: Option<usize>,
    #[nserde(proxy = "u32")]
    pub mode: Option<PrimitiveMode>,
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

#[derive(Debug, PartialEq)]
pub enum Filter {
    Nearest = 9728,
    Linear = 9729,
    NearestMimpapNearest = 9984,
    LinearMipmapNearest = 9985,
    NearestMipmapLinear = 9986,
    LinearMipmapLinear = 9987,
}
impl From<&u32> for Filter {
    fn from(n: &u32) -> Filter {
        match *n {
            9728 => Filter::Nearest,
            9729 => Filter::Linear,
            9984 => Filter::NearestMimpapNearest,
            9985 => Filter::LinearMipmapNearest,
            9986 => Filter::NearestMipmapLinear,
            9987 => Filter::LinearMipmapLinear,
            x => panic!("Not a Filter u32! {x}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum WrappingMode {
    ClampToEdge = 33071,
    MirroredRepeat = 33648,
    Repeat = 10497,
}
impl From<&u32> for WrappingMode {
    fn from(n: &u32) -> WrappingMode {
        match *n {
            33071 => WrappingMode::ClampToEdge,
            33648 => WrappingMode::MirroredRepeat,
            10497 => WrappingMode::Repeat,
            x => panic!("Not a WrappingMode u32! {x}"),
        }
    }
}

#[derive(DeJson, PartialEq, Debug)]
pub struct Sampler {
    #[nserde(rename = "magFilter")]
    #[nserde(proxy = "u32")]
    pub mag_filter: Option<Filter>,
    #[nserde(rename = "minFilter")]
    #[nserde(proxy = "u32")]
    pub min_filter: Option<Filter>,
    #[nserde(rename = "wrapS")]
    #[nserde(proxy = "u32")]
    pub wrap_s: Option<WrappingMode>,
    #[nserde(proxy = "u32")]
    #[nserde(rename = "wrapT")]
    pub wrap_t: Option<WrappingMode>,
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
