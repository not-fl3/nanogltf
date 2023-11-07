/// Raw json structs.
pub mod gltf;

mod base64;

pub use gltf::*;

impl gltf::Gltf {
    pub fn from_json(json: &str) -> Result<gltf::Gltf, nanoserde::DeJsonErr> {
        nanoserde::DeJson::deserialize_json(json)
    }
}

/// A few optional helpers to extract the data out of the parsed gltf.
pub mod utils {
    use crate::{base64, gltf as ngltf, Gltf};

    /// Data encoded in the URI
    pub enum UriData {
        Bytes(Vec<u8>),
        RelativePath(String),
    }

    /// Parse gltf's base64 string into a byte array or an external link.
    pub fn parse_uri(uri: &str) -> UriData {
        if !uri.starts_with("data:") {
            return UriData::RelativePath(uri.to_string());
        }

        if uri.starts_with("data:application/octet-stream;base64") {
            let uri = uri
                .strip_prefix("data:application/octet-stream;base64,")
                .unwrap();
            let bytes = base64::decode(uri);
            return UriData::Bytes(bytes);
        }

        if uri.starts_with("data:image/jpeg;base64") {
            let uri = uri.strip_prefix("data:image/jpeg;base64,").unwrap();
            let bytes = base64::decode(uri);
            return UriData::Bytes(bytes);
        }

        if uri.starts_with("data:image/png;base64") {
            let uri = uri.strip_prefix("data:image/png;base64,").unwrap();
            let bytes = base64::decode(uri);
            return UriData::Bytes(bytes);
        }

        unimplemented!()
    }

    pub enum ImageSource {
        Bytes(Vec<u8>),
        Slice {
            buffer: usize,
            offset: usize,
            length: usize,
        },
        RelativePath(String),
    }

    /// Get (buffer index, offset in the buffer, amount of bytes) from the view/accessor for the given attribute.
    ///
    /// Could be used as:
    /// `utils::attribute_bytes(&model, primitive.attributes["TEXCOORD_0"])`
    /// or
    /// `utils::attribute_bytes(&model, primitive.indices)`
    ///
    /// Common attribute names are: TEXCOORD_*, POSITION, NORMAL
    ///
    /// Will panic if gltf have sparse accessors present.
    /// (they are described here [glTF-Tutorials](https://github.com/KhronosGroup/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_005_BuffersBufferViewsAccessors.md), but not yet implemented by nanogltf)
    pub fn attribute_bytes(gltf: &Gltf, attribute: usize) -> (usize, usize, usize) {
        let accessor = &gltf.accessors[attribute];
        let buffer_view = accessor.buffer_view.unwrap();
        let view = &gltf.buffer_views[buffer_view];

        let k = match accessor.type_.as_ref().unwrap().as_str() {
            "VEC3" => 3,
            "VEC2" => 2,
            "SCALAR" => 1,
            _ => panic!(),
        };

        (
            view.buffer,
            accessor.byte_offset + view.byte_offset,
            accessor.count * accessor.component_type.byte_size() * k,
        )
    }

    /// If uri is present - will parse the uri into a byte array. If not - will return the (buffer index, byte_offset, byte_length).
    pub fn image_source(gltf: &Gltf, image: &ngltf::Image) -> ImageSource {
        if image.uri.is_some() {
            let uri = parse_uri(image.uri.as_ref().unwrap());
            match uri {
                UriData::Bytes(view) => ImageSource::Bytes(view),
                UriData::RelativePath(uri) => ImageSource::RelativePath(uri),
            }
        } else {
            let view = image.buffer_view.unwrap();
            let view = &gltf.buffer_views[view];
            ImageSource::Slice {
                buffer: view.buffer,
                offset: view.byte_offset,
                length: view.byte_length,
            }
        }
    }
}
