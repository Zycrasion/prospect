use std::path::Path;
use std::time::{Duration, SystemTime};

use futures::channel::mpsc::{channel, Receiver};

use futures::{executor, SinkExt};
use notify::{Event, RecommendedWatcher, Config, Watcher, ReadDirectoryChangesWatcher, PollWatcher, RecursiveMode};
use prospect::abstraction::shader::ProspectShader;
use prospect::parse_obj;
use prospect::smart::{SmartRenderPipeline, SmartBindGroup};
use prospect::{utils::prospect_fs::{path_with_respect_to_cwd_str, path_with_respect_to_cwd}};
use prospect::wgpu::{SurfaceError, Texture};
use prospect::winit::{
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::CursorGrabMode,
};
use prospect::{
    abstraction::{
        graphics_context::GraphicsContext,
        high_level_abstraction::HighLevelGraphicsContext,
        mesh::{Mesh, Meshable},
        prospect_window::ProspectWindow,
        shader::BasicShader,
        vertex::Vertex,
    },
    model::Model3D,
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent},
    prospect_camera::ProspectCamera,
    prospect_camera_controller::CameraController,
    prospect_light::ProspectPointLight,
    prospect_shape::ProspectShape,
    shaders::{default_3d::Default3D, textured_shader::TexturedShader},
    utils::prospect_fs::{
        read_file_panic, read_file_with_respect_to_cwd, read_file_with_respect_to_cwd_bytes,
    },
};
use prospect::{
    linear::{Vector, VectorTrait},
    trig::to_degrees,
};

fn main() {
    let mut window = ProspectWindow::new("Test Window", 480, 480);
    let app = ObjPreviewer::new(&mut window);
    window.run_with_app(Box::new(app));
}

fn to_shape(str: &str) -> ProspectShape<Vec<Vertex>, Vec<u32>> {
    let mut mesh = parse_obj(str);
    let verts = mesh.extract_vertices_and_uv_and_normals();
    let mut shape: ProspectShape<Vec<Vertex>, Vec<u32>> = ProspectShape {
        vertices: Vec::new(),
        indices: None,
    };

    for vert in verts {
        shape.vertices.push(Vertex {
            position: [vert.0.x, vert.0.y, vert.0.z],
            uv: [vert.1.x, 1. - vert.1.y],
            normal: [vert.2.x, vert.2.y, vert.2.z],
        })
    }

    shape
}

fn watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)>
{
    let (mut tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

fn watch<S: AsRef<str>>(path: S) -> notify::Result<(Receiver<Result<Event, notify::Error>>, RecommendedWatcher)>
{
    let (mut watcher, mut rx) = watcher()?;

    println!("{}", path.as_ref().to_string());
    watcher.watch(&Path::new(path.as_ref()), notify::RecursiveMode::Recursive)?;

    Ok((rx, watcher))
}

pub struct ObjPreviewer {
    shader : Default3D,
    shader_key : SmartRenderPipeline,
    main_model: Model3D,
    main_mesh: Mesh,
    texture : SmartBindGroup,
    model_path: String,
    texture_path: String,
    model_rx : Receiver<Result<Event, notify::Error>>,
    watcher: RecommendedWatcher,
    frame: f32,
    camera: ProspectCamera,
    cam_controller: CameraController,
    last_frame: SystemTime,
    light: ProspectPointLight,
}

impl ObjPreviewer {
    fn new(window: &mut ProspectWindow) -> Self {
        let camera = ProspectCamera::new(window.get_device());
        let mut light = ProspectPointLight::new(window);
        light.position = Vector::new3(4., 4., 4.);
        light.colour = Vector::new3(1., 1., 1.);

        let default_shader = Default3D::new(&window);
        let default_shader_key = default_shader.build_render_pipeline(window.get_device(), vec![camera.get_layout(), light.get_layout()]).into();

        let model_path = std::env::args()
            .nth(1)
            .unwrap_or("res/car01.obj".to_string());
        let texture_path = std::env::args()
            .nth(2)
            .unwrap_or("res/car01_Car_Pallete.png".to_string());

        let texture = default_shader.register_texture(
            "texture",
            &read_file_with_respect_to_cwd_bytes(&texture_path),
            window,
        );

        let mut main_mesh = Mesh::from_shape(
            &to_shape(&read_file_with_respect_to_cwd(&model_path)),
            window.get_device(),
            &default_shader_key,
        );
        main_mesh.set_bind_group(1, &texture);
        main_mesh.set_bind_group(2, &light.get_bind_group());
        let main_model = Model3D::new(&default_shader, window);

        // Dispatch watcher
        let (rx, mut watcher) = watch(&path_with_respect_to_cwd_str(&model_path)).unwrap();
        watcher.watch(&path_with_respect_to_cwd(&texture_path), RecursiveMode::Recursive).unwrap();

        Self {
            watcher,
            shader : default_shader,
            shader_key : default_shader_key,
            texture_path,
            texture,
            main_mesh,
            main_model,
            model_rx: rx,
            model_path,
            frame: 1.,
            camera,
            last_frame: SystemTime::now(),
            cam_controller: CameraController::new(),
            light,
        }
    }
}

impl ProspectApp for ObjPreviewer {
    fn setup(&mut self, window: &mut ProspectWindow) {}

    fn draw(&mut self, window: &mut ProspectWindow) -> Result<(), SurfaceError> {
        /* update */
        let this_time = SystemTime::now();
        let delta = this_time
            .duration_since(self.last_frame)
            .unwrap_or(Duration::from_secs_f32(1. / 60.))
            .as_secs_f32();
        self.last_frame = this_time;

        self.light.process_frame(window);
        self.cam_controller.process(delta, &mut self.camera, window);
        self.camera.process_frame(
            window.size.0 as f32,
            window.size.1 as f32,
            window.get_queue(),
        );

        let clear_colour = (0.5, 0.0, 0.5);

        if let Ok(Some(val)) = self.model_rx.try_next()
        {
            match val
            {
                Ok(_) => {
                    println!("Updating Mesh & Texture...");
                    let texture = self.shader.register_texture(
                        "texture",
                        &read_file_with_respect_to_cwd_bytes(&self.texture_path),
                        window,
                    );
                    self.texture = texture;

                    let mut main_mesh = Mesh::from_shape(
                        &to_shape(&read_file_with_respect_to_cwd(&self.model_path)),
                        window.get_device(),
                        &self.shader_key,
                    );
                    main_mesh.set_bind_group(1, &self.texture);
                    main_mesh.set_bind_group(2, &self.light.get_bind_group());
                    self.main_mesh = main_mesh;
                },
                Err(a) => {
                    println!("{:#?} twas error", a);
                }
            }
        }

        /* draw */
        let (output, view, mut command_encoder) = HighLevelGraphicsContext::init_view(window);
        let mut render_pass = HighLevelGraphicsContext::start_render(
            clear_colour,
            &view,
            window.get_depth_buffer(),
            &mut command_encoder,
        );

        self.main_model
            .draw(&mut render_pass, window, &self.camera, &self.main_mesh);

        drop(render_pass);

        self.frame += 1. / 60.;
        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
        Ok(())
    }

    fn process(&mut self, event: ProspectEvent, window: &mut ProspectWindow) -> ProcessResponse {
        self.cam_controller.input_event(event, window);
        ProcessResponse::ProspectProcess
    }
}
