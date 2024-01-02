use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
    thread,
};

use noise::Perlin;
use prospect::{
    abstraction::{
        high_level_abstraction::HighLevelGraphicsContext, mesh::Meshable,
        prospect_window::ProspectWindow,
    },
    linear::{Vector, VectorTrait},
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent},
    prospect_light::ProspectPointLight,
    prospect_texture::ProspectTexture,
    shaders::{default_3d::Default3D, textured_shader::TexturedShader}, prospect_shader_manager::{ProspectShaderIndex, ProspectBindGroupIndex},
};
use voxel_engine::{
    chunk::{Chunk, ChunkData, CHUNK_LWH, CHUNK_SIZE},
    player::Player,
    voxel_shader::VoxelShader,
};

fn main() {
    let mut window = ProspectWindow::new("Voxel Engine", 720, 720);
    let engine = VoxelEngine::new(&mut window);
    window.run_with_app(Box::new(engine));
}

pub struct VoxelEngine {
    player: Player,
    noise: Perlin,
    chunks: Vec<Chunk>,
    chunk_data: Arc<Mutex<Vec<ChunkData>>>,
    shader : VoxelShader,
    shader_key : ProspectShaderIndex,
    light : ProspectPointLight,
    texture_index : ProspectBindGroupIndex
}

impl VoxelEngine {
    pub fn new(window: &mut ProspectWindow) -> Self {
        let mut block_atlas = ProspectTexture::from_bytes(
            "BlockAtlas",
            include_bytes!("textures/block_atlas.png"),
            window,
        );

        let player = Player::new(window);

        let mut light = ProspectPointLight::new(window);
        light.position = Vector::new3(0., 10., 0.);
        light.process_frame(window);

        let shader = VoxelShader::new(&window);
        let shader_key = window.add_shader(&shader, player.get_camera(), vec![light.get_layout()]);
        let texture_index = shader.bind_prospect_texture(&block_atlas, window);

        let noise = Perlin::new(55);

        let chunk_data = Arc::new(Mutex::new(vec![]));
        let chunks = vec![];

        // for x in -5..=5 {
        //     for y in -5..=5 {
        //         for z in -5..=5 {
        //             thread::spawn(|| {
        //                 let data = ChunkData::new(x as f32, y as f32, z as f32, noise);
        //                 chunk_data.lock().unwrap().push(data);
        //             });
        //             // CHUNKS.push(Chunk::new(data, window, &shader_key, &shader, &light, &index))
        //         }
        //     }
        // }

        Self {
            player,
            noise,
            chunk_data,
            chunks,
            shader,
            shader_key,
            light,
            texture_index
        }
    }

    pub fn generate(&self)
    {
        let noise = self.noise.clone();
        let chunk_data = self.chunk_data.clone();
        thread::spawn(move || {
            for x in -5..=5 {
                for y in -5..=5 {
                    for z in -5..=5 {
                        let data = ChunkData::new(x as f32, y as f32, z as f32, noise);
                        chunk_data.lock().unwrap().push(data);
                        // CHUNKS.push(Chunk::new(data, window, &shader_key, &shader, &light, &index))
                    }
                }
            }
        });
    }
}

impl ProspectApp for VoxelEngine {
    fn setup(&mut self, window: &mut ProspectWindow) 
    {
        self.generate();
    }

    fn draw(&mut self, window: &mut ProspectWindow) -> Result<(), prospect::wgpu::SurfaceError> {
        self.player.update(window);

        let mut len = self.chunk_data.lock().unwrap().len();

        if len > 0
        {
            while len > 0
            {
                let first_chunk = self.chunk_data.lock().unwrap().remove(0);
                self.chunks.push(Chunk::new(first_chunk, window, &self.shader_key, &self.shader, &self.light, &self.texture_index));
                len -= 1;
            }
        }

        let clear_colour = (0.5, 0.0, 0.5);
        let (output, view, mut command_encoder) = HighLevelGraphicsContext::init_view(window);
        let mut render_pass = HighLevelGraphicsContext::start_render(
            clear_colour,
            &view,
            window.get_depth_buffer(),
            &mut command_encoder,
        );

        self.chunks.sort_by(|a, b| {
            return a
                .dist_from(self.player.get_camera())
                .total_cmp(&b.dist_from(self.player.get_camera()));
        });

        for chunk in &self.chunks
        {
            chunk.draw(&mut render_pass, window, self.player.get_camera());
        }

        drop(render_pass);
        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
        Ok(())
    }

    fn process(&mut self, event: ProspectEvent, window: &mut ProspectWindow) -> ProcessResponse {
        self.player.process(event, window);
        ProcessResponse::ProspectProcess
    }
}
