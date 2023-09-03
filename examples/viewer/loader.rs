use miniquad::*;

use glam::{vec3, Quat, Vec3};

pub mod shader {
    use miniquad::*;

    pub const FRAGMENT: &str = include_str!("fragment.glsl");
    pub const VERTEX: &str = include_str!("vertex.glsl");
    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![
                "Albedo".to_string(),
                "Emissive".to_string(),
                "Occlusion".to_string(),
                "Normal".to_string(),
                "MetallicRoughness".to_string(),
                "Environment".to_string(),
            ],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("Projection", UniformType::Mat4),
                    UniformDesc::new("Model", UniformType::Mat4),
                    UniformDesc::new("ModelInverse", UniformType::Mat4),
                    UniformDesc::new("Color", UniformType::Float4),
                    UniformDesc::new("Material", UniformType::Float4),
                    UniformDesc::new("CameraPosition", UniformType::Float3),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub projection: glam::Mat4,
        pub model: glam::Mat4,
        pub model_inverse: glam::Mat4,
        pub color: [f32; 4],
        pub material: [f32; 4], // metallic, roughness, 0, 0,
        pub camera_pos: glam::Vec3,
    }
}

pub struct NodeData {
    pub pipeline: Pipeline,
    pub color: [f32; 4],
    pub material: [f32; 4],
    pub vertex_buffers: Vec<miniquad::BufferId>,
    pub index_buffer: miniquad::BufferId,
    pub base_color_texture: Option<miniquad::TextureId>,
    pub emissive_texture: Option<miniquad::TextureId>,
    pub occlusion_texture: Option<miniquad::TextureId>,
    pub normal_texture: Option<miniquad::TextureId>,
    pub metallic_roughness_texture: Option<miniquad::TextureId>,
}

pub struct Node {
    pub name: String,
    pub data: Vec<NodeData>,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

pub struct Model {
    pub nodes: Vec<Node>,
}

pub fn load_gltf(ctx: &mut miniquad::Context, json: &str) -> Model {
    use nanogltf::{utils, Gltf};

    let gltf = Gltf::from_json(json).unwrap();
    //println!("{:#?}", gltf);
    let buffers = gltf
        .buffers
        .iter()
        .map(|buffer| {
            let bytes = match utils::parse_uri(&buffer.uri) {
                utils::UriData::Bytes(bytes) => bytes,
                _ => unimplemented!(),
            };
            bytes
        })
        .collect::<Vec<_>>();

    assert!(gltf.scenes.len() == 1);

    let mut textures = vec![];
    for image in &gltf.images {
        let source = utils::image_source(&gltf, image);
        let bytes: &[u8] = match source {
            utils::ImageSource::Bytes(ref bytes) => bytes,
            utils::ImageSource::Slice {
                buffer,
                offset,
                length,
            } => &buffers[buffer][offset..offset + length],
        };
        let image = crate::image::decode(&bytes).unwrap();
        //let image = image::load_from_memory(bytes).unwrap().to_rgba8();

        let texture =
            ctx.new_texture_from_rgba8(image.width as u16, image.height as u16, &image.data);
        ctx.texture_set_wrap(texture, TextureWrap::Repeat, TextureWrap::Repeat);
        textures.push(texture);
    }

    let mut nodes = vec![];
    assert!(gltf.scenes.len() == 1);
    let scene = &gltf.scenes[0];
    for node in &scene.nodes {
        let node = &gltf.nodes[*node];
        if node.children.len() != 0 {
            continue;
        }
        let translation = node
            .translation
            .map_or(Vec3::ZERO, |t| vec3(t[0] as f32, t[1] as f32, t[2] as f32));
        let rotation = node.rotation.map_or(Quat::IDENTITY, |t| {
            Quat::from_xyzw(t[0] as f32, t[1] as f32, t[2] as f32, t[3] as f32)
        });
        let scale = node.scale.map_or(vec3(1., 1., 1.), |t| {
            vec3(t[0] as f32, t[1] as f32, t[2] as f32)
        });
        let mesh = node.mesh.unwrap();
        let mesh = &gltf.meshes[mesh];
        let mut bindings = Vec::new();

        for primitive in &mesh.primitives {
            let material = &gltf.materials[primitive.material.unwrap()];
            let color = material.pbr_metallic_roughness.base_color_factor;
            let base_color_texture = &material.pbr_metallic_roughness.base_color_texture;
            let base_color_texture = base_color_texture.as_ref().map(|t| textures[t.index]);
            let metallic_roughness_texture = material
                .pbr_metallic_roughness
                .metallic_roughness_texture
                .as_ref()
                .map(|t| textures[t.index]);
            let emissive_texture = material
                .emissive_texture
                .as_ref()
                .map(|t| textures[t.index]);
            let occlusion_texture = material
                .occlusion_texture
                .as_ref()
                .map(|t| textures[t.index]);
            let normal_texture = material.normal_texture.as_ref().map(|t| textures[t.index]);
            let color = [
                color[0] as f32,
                color[1] as f32,
                color[2] as f32,
                color[3] as f32,
            ];

            let indices = utils::attribute_bytes(&gltf, primitive.indices.unwrap());
            let indices = &buffers[indices.0][indices.1..indices.1 + indices.2];
            let vertices = utils::attribute_bytes(&gltf, primitive.attributes["POSITION"]);
            let vertices = &buffers[vertices.0][vertices.1..vertices.1 + vertices.2];
            let uvs = utils::attribute_bytes(&gltf, primitive.attributes["TEXCOORD_0"]);
            let uvs = &buffers[uvs.0][uvs.1..uvs.1 + uvs.2];
            let normals = utils::attribute_bytes(&gltf, primitive.attributes["NORMAL"]);
            let normals = &buffers[normals.0][normals.1..normals.1 + normals.2];

            let vertex_buffer =
                ctx.new_buffer(BufferType::VertexBuffer, BufferUsage::Immutable, unsafe {
                    BufferSource::pointer(vertices.as_ptr(), vertices.len(), 4 * 3)
                });
            let normals_buffer =
                ctx.new_buffer(BufferType::VertexBuffer, BufferUsage::Immutable, unsafe {
                    BufferSource::pointer(normals.as_ptr(), normals.len(), 4 * 3)
                });
            let uvs_buffer =
                ctx.new_buffer(BufferType::VertexBuffer, BufferUsage::Immutable, unsafe {
                    BufferSource::pointer(uvs.as_ptr(), uvs.len(), 4 * 2)
                });
            let index_buffer =
                ctx.new_buffer(BufferType::IndexBuffer, BufferUsage::Immutable, unsafe {
                    BufferSource::pointer(indices.as_ptr(), indices.len(), 2)
                });

            // Note on shaders post processing.
            // Usually, glsl version 100 and metal would cover everything, and no post-processing would be required.
            // But if you are looking at this code - there is a good chance you are intrested in PBR shaders
            // and those shaders are really easier to implelement with dfDx and glTextureCube.
            // On linux/glx and webgl1 it is possible to get those functions in version 100 shader through extensions and keep everything simple.
            // But on mobile EGL and Mac OpenGL shader should be at least #version 300/330es.
            //
            // shadermagic is a VERY questionable shader compiler. But it works for this very shader.
            // and through studying its output you can choose more appropriate cross-compiler
            // or decide wich platforms to sacrifice and hand-write just some of those shaders.
            let shader = shadermagic::transform(
                shader::FRAGMENT,
                shader::VERTEX,
                &shader::meta(),
                &shadermagic::Options {
                    defines: vec![
                        "HAS_NORMAL_MAP".to_string(),
                        "HAS_METALLIC_ROUGHNESS_MAP".to_string(),
                    ],

                    ..Default::default()
                },
            )
            .unwrap();
            let source = shadermagic::choose_appropriate_shader(&shader, &ctx.info());
            let shader = ctx
                .new_shader(source, shader::meta())
                .unwrap_or_else(|e| panic!("Failed to load shader: {}", e));

            let pipeline = ctx.new_pipeline_with_params(
                &[
                    BufferLayout::default(),
                    BufferLayout::default(),
                    BufferLayout::default(),
                ],
                &[
                    VertexAttribute::with_buffer("in_position", VertexFormat::Float3, 0),
                    VertexAttribute::with_buffer("in_uv", VertexFormat::Float2, 1),
                    VertexAttribute::with_buffer("in_normal", VertexFormat::Float3, 2),
                ],
                shader,
                PipelineParams {
                    depth_test: Comparison::LessOrEqual,
                    depth_write: true,
                    ..Default::default()
                },
            );

            bindings.push(NodeData {
                pipeline,
                color,
                material: [
                    material.pbr_metallic_roughness.metallic_factor as f32,
                    material.pbr_metallic_roughness.roughness_factor as f32,
                    0.,
                    0.,
                ],
                vertex_buffers: vec![vertex_buffer, uvs_buffer, normals_buffer],
                index_buffer,
                base_color_texture,
                emissive_texture,
                occlusion_texture,
                normal_texture,
                metallic_roughness_texture,
            });
        }

        nodes.push(Node {
            name: node
                .name
                .clone()
                .unwrap_or("unnamed".to_string())
                .to_owned(),
            data: bindings,
            translation,
            rotation,
            scale,
        });
    }

    Model { nodes }
}
