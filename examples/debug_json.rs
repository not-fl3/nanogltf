use nanogltf;
use nanoserde;

use nanogltf::utils;

pub fn main() {
    let testfile = include_str!("DamagedHelmet.gltf");

    let gltf: nanogltf::Gltf = nanoserde::DeJson::deserialize_json(testfile).unwrap();
    println!("{:?}", &gltf);

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
    assert!(buffers.len() != 0);

    for image in &gltf.images {
        let view = utils::parse_uri(image.uri.as_ref().unwrap());
        let view = match view {
            utils::UriData::Bytes(view) => view,
            _ => unimplemented!(),
        };
        let image = nanoimage::decode(&view).unwrap();

        println!("valid image, size: ({} {})", image.width, image.height);
    }

    assert!(gltf.scenes.len() == 1);
    let scene = &gltf.scenes[0];
    for node in &scene.nodes {
        let node = &gltf.nodes[*node];

        println!("node: {:?}", node);
        let mesh = node.mesh.unwrap();
        let mesh = &gltf.meshes[mesh];
        println!("mesh: {:?}", mesh);
        for primitive in &mesh.primitives {
            let material = primitive.material.unwrap();
            let material = &gltf.materials[material];
            let color = material.pbr_metallic_roughness.base_color_factor;

            println!("primitive: {:?}, color: {:?}", primitive, color);

            let data = utils::attribute_bytes(&gltf, primitive.attributes["POSITION"]);
            let bytes = &buffers[data.0][data.1..data.1 + data.2];
            println!("positions byte buffer len: {}", bytes.len());
        }
    }
}
