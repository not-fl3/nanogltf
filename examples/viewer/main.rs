use miniquad::*;

use dolly::prelude::*;
use glam::{vec2, Mat4, Vec3};

mod cubemap;
mod image;
mod loader;

struct Stage {
    white_texture: miniquad::TextureId,
    black_texture: miniquad::TextureId,
    model: loader::Model,
    cubemap: cubemap::Cubemap,
    dolly_rig: CameraRig,
    ctx: Box<dyn RenderingBackend>,
    last_frame: f64,
    mouse_down: bool,
    zoom: f32,
    last_mouse: Option<(f32, f32)>,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let skybox = &[
            &include_bytes!("skybox/skybox_px.png")[..],
            &include_bytes!("skybox/skybox_nx.png")[..],
            &include_bytes!("skybox/skybox_py.png")[..],
            &include_bytes!("skybox/skybox_ny.png")[..],
            &include_bytes!("skybox/skybox_pz.png")[..],
            &include_bytes!("skybox/skybox_nz.png")[..],
        ];

        // unsafe {
        //     let version = gl::glGetString(gl::GL_SHADING_LANGUAGE_VERSION);
        //     let version = std::ffi::CStr::from_ptr(version as _).to_str().unwrap();
        //     miniquad::info!("version: {}", version);

        //     pub const GL_NUM_SHADING_LANGUAGE_VERSIONS: u32 = 0x82E9;
        //     let mut num = 0;
        //     gl::glGetIntegerv(GL_NUM_SHADING_LANGUAGE_VERSIONS, &mut num);
        //     println!("hello {num}");
        //     for i in 0..num {
        //         let version = gl::glGetStringi(gl::GL_SHADING_LANGUAGE_VERSION, i as _);
        //         let version = std::ffi::CStr::from_ptr(version as _).to_str().unwrap();
        //         miniquad::info!("version: {}", version);
        //     }

        //     let mut num = 0;
        //     gl::glGetIntegerv(gl::GL_NUM_EXTENSIONS, &mut num);
        //     println!("hello {num}");
        //     for i in 0..num {
        //         let version = gl::glGetStringi(gl::GL_EXTENSIONS, i as _);
        //         let version = std::ffi::CStr::from_ptr(version as _).to_str().unwrap();
        //         miniquad::info!("version: {}", version);
        //     }
        // }

        let cubemap = cubemap::Cubemap::new(ctx.as_mut(), skybox);
        ctx.texture_set_min_filter(
            cubemap.texture,
            FilterMode::Linear,
            MipmapFilterMode::Linear,
        );
        ctx.texture_generate_mipmaps(cubemap.texture);

        let model = loader::load_gltf(ctx.as_mut(), include_str!("../DamagedHelmet.gltf"));

        let dolly_rig: CameraRig = CameraRig::builder()
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-10.0))
            .with(Smooth::new_rotation(0.7))
            .with(Arm::new(Vec3::Z * 4.0))
            .build();

        Stage {
            dolly_rig,
            white_texture: ctx.new_texture_from_rgba8(1, 1, &[255, 255, 255, 255]),
            black_texture: ctx.new_texture_from_rgba8(1, 1, &[0, 0, 0, 0]),
            model,
            ctx,
            cubemap,
            zoom: 4.0,
            mouse_down: false,
            last_frame: miniquad::date::now(),
            last_mouse: None,
        }
    }
}

impl EventHandler for Stage {
    fn mouse_button_down_event(&mut self, _button: MouseButton, x: f32, y: f32) {
        self.mouse_down = true;
        self.last_mouse = Some((x, y));
    }
    fn mouse_button_up_event(&mut self, _button: MouseButton, _: f32, _e: f32) {
        self.mouse_down = false;
    }
    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        let (w, h) = window::screen_size();
        if self.mouse_down {
            if let Some((last_x, last_y)) = self.last_mouse {
                let mouse_delta = vec2(last_x - x, last_y - y) / vec2(w, h);

                self.dolly_rig
                    .driver_mut::<YawPitch>()
                    .rotate_yaw_pitch(mouse_delta.x * 200., mouse_delta.y * 200.);
            }
        }
        self.last_mouse = Some((x, y));
    }
    fn mouse_wheel_event(&mut self, _: f32, y: f32) {
        if y != 0.0 {
            self.zoom -= y * 0.01;
            self.zoom = self.zoom.clamp(1.8, 10.0);
            self.dolly_rig.driver_mut::<Arm>().offset = Vec3::Z * self.zoom;
        }
    }

    fn update(&mut self) {}

    fn draw(&mut self) {
        let time = miniquad::date::now();
        let delta = (time - self.last_frame) as f32;
        self.last_frame = miniquad::date::now();

        let (w, h) = window::screen_size();
        let t = self.dolly_rig.update(delta);
        let proj = Mat4::perspective_rh_gl(45., w / h, 0.01, 100.);
        let view = Mat4::look_at_rh(t.position, t.position + t.forward(), t.up());

        self.cubemap.draw(self.ctx.as_mut(), &proj, &view);

        self.ctx.begin_default_pass(PassAction::Nothing);

        for node in &self.model.nodes {
            for primitive in &node.data {
                let cubemap = self.cubemap.texture;
                let images = [
                    primitive.base_color_texture.unwrap_or(self.white_texture),
                    primitive.emissive_texture.unwrap_or(self.black_texture),
                    primitive.occlusion_texture.unwrap_or(self.white_texture),
                    primitive.normal_texture.unwrap_or(self.white_texture),
                    primitive
                        .metallic_roughness_texture
                        .unwrap_or(self.white_texture),
                    cubemap,
                ];
                self.ctx.apply_pipeline(&primitive.pipeline);
                self.ctx.apply_bindings_from_slice(
                    &primitive.vertex_buffers,
                    primitive.index_buffer,
                    &images,
                );

                let projection = proj * view;

                let model = Mat4::from_translation(node.translation)
                    * Mat4::from_quat(node.rotation)
                    * Mat4::from_scale(node.scale);
                let model_inverse = model.inverse();
                self.ctx
                    .apply_uniforms(UniformsSource::table(&loader::shader::Uniforms {
                        projection,
                        model,
                        model_inverse,
                        color: primitive.color,
                        material: primitive.material,
                        camera_pos: t.position,
                    }));

                let buffer_size = self.ctx.buffer_size(primitive.index_buffer) as i32 / 2;
                self.ctx.draw(0, buffer_size, 1);
            }
        }
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}

fn main() {
    let mut conf = conf::Conf::default();
    let metal = std::env::args().nth(1).as_deref() == Some("metal");
    conf.platform.apple_gfx_api = if metal {
        conf::AppleGfxApi::Metal
    } else {
        conf::AppleGfxApi::OpenGl
    };

    let egl = std::env::args().nth(1).as_deref() == Some("egl");
    if egl {
        conf.platform.linux_x11_gl = conf::LinuxX11Gl::EGLOnly;
    }

    miniquad::start(conf, move || Box::new(Stage::new()));
}
