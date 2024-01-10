use image::imageops::FilterType::Triangle;
use prospect::{
    abstraction::{prospect_window::ProspectWindow, high_level_abstraction::HighLevelGraphicsContext, mesh::{Mesh, Meshable}, shader::{BasicShader, ProspectShader}, vertex::Vertex, graphics_context::GraphicsContext},
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent}, prospect_shape::ProspectShape, prospect_camera::ProspectCamera, prospect_framebuffer::ProspectFramebuffer, prospect_texture::BindableTexture, shaders::textured_shader::TexturedShader,
};
use vecto_rs::linear::{Vector, VectorTrait};
use wgpu::SurfaceError;
use winit::event::VirtualKeyCode;

const TRIANGLE : ProspectShape<&[Vertex], &[u32]> = ProspectShape
{
    vertices: &[
        Vertex { position : [  0.0,  0.5, 0.], uv : [0.5, 0.0], normal : [0.; 3] },
        Vertex { position : [  0.5, -0.5, 0.], uv : [1.0, 1.0], normal : [0.; 3] },
        Vertex { position : [ -0.5, -0.5, 0.], uv : [0.0, 1.0], normal : [0.; 3] },
    ],
    indices: None,
};

fn main() {
    let mut window = ProspectWindow::new("Hello World!", 480, 480);

    let a = Box::new(HelloWorld::new(&mut window));
    window.run_with_app(a);
}

pub struct HelloWorld
{
    clear_col : (f64, f64, f64),
    mesh : Mesh,
    mesh2 : Mesh,
    camera : ProspectCamera,
    framebuffer : ProspectFramebuffer,
    framebuffer_depth_buffer : ProspectFramebuffer,
}

impl HelloWorld
{
    pub fn new(window : &mut ProspectWindow) -> Self
    {
        let basic_shader = BasicShader::new(window);
        let mut camera = ProspectCamera::new(window.get_device());
        camera.eye = Vector::new3(0., 0., -1.);
        let basic_shader = basic_shader.build_render_pipeline(window.get_device(), vec![camera.get_layout()]).into();

        let mesh = Mesh::from_shape(&TRIANGLE, window.get_device(), &basic_shader);
        let framebuffer = ProspectFramebuffer::new(window.get_device(), 720, 720);
        let framebuffer_depth_buffer = ProspectFramebuffer::new_depth(window.get_device(), 720, 720);

        let textured_shader = TexturedShader::new(window);
        let texture_bind_group = textured_shader.bind_prospect_texture(&framebuffer, window);

        let textured_shader_rp = textured_shader.build_render_pipeline(window.get_device(), vec![camera.get_layout()]).into();
        let mut mesh2 = Mesh::from_shape(&TRIANGLE, window.get_device(), &textured_shader_rp);
        mesh2.set_bind_group(1, &texture_bind_group);

        Self
        {
            framebuffer,
            framebuffer_depth_buffer,
            mesh,
            mesh2,
            clear_col: (0., 0., 0.),
            camera
        }
    }
}

impl ProspectApp for HelloWorld {
    fn setup(&mut self, window : &mut ProspectWindow) {}

    fn draw(&mut self, window : &mut ProspectWindow) -> Result<(), SurfaceError>
    {
        /* Framebuffer 1 */
        let clear_colour = (1., 1., 1.);
        let mut command_encoder = GraphicsContext::create_command_encoder(window.get_device(), "Framebuffer Command Encoder");
        let mut render_pass = HighLevelGraphicsContext::start_render(clear_colour, self.framebuffer.get_texture_view(), self.framebuffer_depth_buffer.get_texture_view(), &mut command_encoder);

        self.camera.process_frame(720., 720., window.get_queue());
        self.mesh.draw(&mut render_pass, &self.camera);

        drop(render_pass);
        let buffer = command_encoder.finish();
        window.get_queue().submit(std::iter::once(buffer));

        /* Main Draw */
        let clear_colour = (self.clear_col.0 / window.size.0 as f64, self.clear_col.1 / window.size.1 as f64, 0.5);
        let (output, view, mut command_encoder) = HighLevelGraphicsContext::init_view(window);
        let mut render_pass = HighLevelGraphicsContext::start_render(clear_colour, &view, window.get_depth_buffer(), &mut command_encoder);

        self.camera.process_frame(window.size.0 as f32, window.size.1 as f32, window.get_queue());
        self.mesh2.draw(&mut render_pass, &self.camera);

        drop(render_pass);

        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
        Ok(())
    }

    fn process(&mut self, event: ProspectEvent, window : &mut ProspectWindow) -> ProcessResponse {
        match event {
            ProspectEvent::KeyboardInput(key, _) => {
                if key == Some(VirtualKeyCode::Escape) {
                    ProcessResponse::CloseApp
                } else
                {
                    ProcessResponse::DontProcess
                }
            },
            ProspectEvent::CursorMoveEvent(cursor_pos) =>
            {
                self.clear_col.0 = cursor_pos.x as f64;
                self.clear_col.1 = cursor_pos.y as f64;
                ProcessResponse::DontProcess
            },
            _ => {ProcessResponse::DontProcess}
        }
    }
}
