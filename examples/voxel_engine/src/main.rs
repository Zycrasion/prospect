use noise::Perlin;
use prospect::{prospect_app::{ProspectApp, ProspectEvent, ProcessResponse}, abstraction::{prospect_window::ProspectWindow, high_level_abstraction::HighLevelGraphicsContext, mesh::Meshable}, shaders::{default_3d::Default3D, textured_shader::TexturedShader}, prospect_light::ProspectPointLight};
use voxel_engine::{chunk::{Chunk, CHUNK_SIZE}, player::Player, voxel_shader::VoxelShader};

fn main() {
    let mut window = ProspectWindow::new("Voxel Engine", 720, 720);
    let engine = VoxelEngine::new(&mut window);
    window.run_with_app(Box::new(engine));
}

pub struct VoxelEngine
{
    chunks : Vec<Chunk>,
    player : Player,
    noise : Perlin
}

impl VoxelEngine
{
    pub fn new(window : &mut ProspectWindow) -> Self
    {
        let player = Player::new(window);

        let light = ProspectPointLight::new(window);
        light.process_frame(window);

        let shader = VoxelShader::new(&window);
        let shader_key = window.add_shader(&shader, player.get_camera(), vec![light.get_layout()]);

        let noise = Perlin::new(55);
        let mut chunks = vec![];

        for x in -5..=5
        {
            for y in -5..=5
            {
                for z in -5..=5
                {
                    chunks.push(Chunk::new(x as f32 * CHUNK_SIZE, y as f32 * CHUNK_SIZE, z as f32 * CHUNK_SIZE, noise, window, &shader_key, &shader, &light))
                }
            }
        }

        Self
        {
            player,
            chunks,
            noise,
        }
    }
}

impl ProspectApp for VoxelEngine
{
    fn setup(&mut self, window : &mut ProspectWindow) {
    }

    fn draw(&mut self, window : &mut ProspectWindow) -> Result<(), prospect::wgpu::SurfaceError> {
        self.player.update(window);

        let clear_colour = (0.5, 0.0, 0.5);
        let (output, view, mut command_encoder) = HighLevelGraphicsContext::init_view(window);
        let mut render_pass = HighLevelGraphicsContext::start_render(
            clear_colour,
            &view,
            window.get_depth_buffer(),
            &mut command_encoder,
        );

        for chunk in &self.chunks
        {
            chunk.draw(&mut render_pass, window, self.player.get_camera());
        }

        drop(render_pass);
        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
        Ok(())
    }

    fn process(&mut self, event : ProspectEvent, window : &mut ProspectWindow) -> ProcessResponse {
        self.player.process(event, window);
        ProcessResponse::ProspectProcess
    }
}