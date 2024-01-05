use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
    thread, time::{SystemTime, Duration, Instant}, collections::HashMap,
};

use noise::Perlin;
use prospect::{
    abstraction::{
        high_level_abstraction::HighLevelGraphicsContext, mesh::Meshable,
        prospect_window::ProspectWindow,
    },
    linear::{Vector, VectorTrait, vector3},
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent},
    prospect_light::ProspectPointLight,
    prospect_texture::ProspectTexture,
    shaders::{default_3d::Default3D, textured_shader::TexturedShader}, prospect_shader_manager::{ProspectShaderIndex, ProspectBindGroupIndex}, winit::event::{VirtualKeyCode, ElementState},
};
use voxel_engine::{
    chunk::{Chunk, ChunkData, CHUNK_LWH, CHUNK_SIZE, BLOCKS_PER_CHUNK, ChunkEntry, from_vector, to_vector},
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
    player_pos: Arc<Mutex<Vector>>,
    noise: Perlin,
    chunks: HashMap<ChunkEntry, Chunk>,
    chunk_data: Arc<Mutex<Vec<ChunkData>>>,
    chunk_remove: Arc<Mutex<Vec<ChunkEntry>>>,
    shader : VoxelShader,
    shader_key : ProspectShaderIndex,
    light : ProspectPointLight,
    texture_index : ProspectBindGroupIndex,
    running : Arc<Mutex<bool>>,
    thread_has_stopped : Arc<Mutex<bool>>,
    lock_player_pos : bool,
}

impl VoxelEngine {
    pub fn new(window: &mut ProspectWindow) -> Self {
        let mut block_atlas = ProspectTexture::image_file_from_bytes(
            "BlockAtlas",
            include_bytes!("textures/block_atlas.png"),
            window,
        );

        let player = Player::new(window);

        let mut light = ProspectPointLight::new(window);
        light.position = Vector::new3(0., 0., 0.);
        light.process_frame(window);

        let shader = VoxelShader::new(&window);
        let shader_key = window.add_shader(&shader, player.get_camera(), vec![light.get_layout()]);
        let texture_index = shader.bind_prospect_texture(&block_atlas, window);

        let noise = Perlin::new(55);

        let chunk_data = Arc::new(Mutex::new(vec![]));
        let chunk_remove = Arc::new(Mutex::new(vec![]));
        let chunks = HashMap::new();

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
            player_pos : Arc::new(Mutex::new(vector3(0., 0., 0.))),
            noise,
            chunk_data,
            chunks,
            shader,
            shader_key,
            light,
            texture_index,
            running : Arc::new(Mutex::new(true)),
            thread_has_stopped : Arc::new(Mutex::new(false)),
            chunk_remove,
            lock_player_pos : false
        }
    }

    pub fn start_chunk_thread(&self)
    {
        let noise = self.noise.clone();
        let chunk_data = self.chunk_data.clone();
        let running = self.running.clone(); // clone the reference (not the data)
        let thread_has_stopped = self.thread_has_stopped.clone(); // clone the reference (not the data)
        let player_pos = self.player_pos.clone();
        let chunk_remove = self.chunk_remove.clone();

        thread::spawn(move || {
            let mut built_chunks : Vec<ChunkEntry> = vec![];
            let mut explored_chunks : Vec<ChunkEntry> = vec![];
            
            while *running.lock().unwrap()
            {
                let binding = player_pos.lock().unwrap();
                let mut xyz = binding.clone();
                drop(binding);
                xyz.x = (xyz.x / CHUNK_SIZE).floor();
                xyz.y = (xyz.y / CHUNK_SIZE).floor();
                xyz.z = (xyz.z / CHUNK_SIZE).floor();
                let xyz = xyz;

                if !explored_chunks.contains(&from_vector(xyz))
                {
                    for i in -2..=2i32
                    {
                        for j in -2..=2i32
                        {
                            for k in -2..=2i32
                            {
                                if !built_chunks.contains(&from_vector(xyz + vector3(i as f32, j as f32, k as f32)))
                                {
                                    let data = ChunkData::new(xyz.x + i as f32, xyz.y + j as f32, xyz.z + k as f32, noise);
                                    chunk_data.lock().unwrap().push(data);
                                    built_chunks.push(from_vector(xyz + vector3(i as f32, j as f32, k as f32)));       
                                }
                            }
                        }
                    }
                    explored_chunks.push(from_vector(xyz));
                }

                let mut removed_chunks = vec![];
                let mut i : i32 = 0;
                for chunk in &built_chunks
                {
                    if to_vector(*chunk).dist(&xyz) > 4.
                    {
                        println!("Marked Chunk for deletion {:#?}", chunk);
                        chunk_remove.lock().unwrap().push(*chunk);
                        removed_chunks.push(i);
                        if explored_chunks.contains(chunk)
                        {
                            let e = explored_chunks.iter().position(|p| *p == *chunk);
                            if let Some(i) = e
                            {
                                explored_chunks.remove(i);
                            }
                        }
                        i -= 1;
                    }
                    i += 1;
                }

                for i in removed_chunks
                {
                    if i.is_negative()
                    {
                        continue;
                    }
                    built_chunks.remove(i as usize);
                }

                thread::sleep(Duration::from_secs_f32(1. / 30.)) // Only run 30 times a second
            }
            println!("Stopped Running!");
            *thread_has_stopped.lock().unwrap() = true;
        });
    }
}

impl ProspectApp for VoxelEngine {
    fn setup(&mut self, window: &mut ProspectWindow) 
    {
        self.start_chunk_thread();
    }

    fn draw(&mut self, window: &mut ProspectWindow) -> Result<(), prospect::wgpu::SurfaceError> {
        self.player.update(window);

        if !self.lock_player_pos
        {
            *self.player_pos.lock().unwrap() = self.player.get_camera().eye;
        }

        let mut len = self.chunk_data.lock().unwrap().len();

        if len > 0
        {
            while len > 0
            {
                let first_chunk = self.chunk_data.lock().unwrap().remove(0);
                self.chunks.insert(first_chunk.entry, Chunk::new(first_chunk, window, &self.shader_key, &self.shader, &self.light, &self.texture_index));
                len -= 1;
            }
        }

        let mut len = self.chunk_remove.lock().unwrap().len();

        if len > 0
        {
            while len > 0
            {
                let first_chunk = self.chunk_remove.lock().unwrap().remove(0);
                self.chunks.remove(&first_chunk);
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

        let plr_camera_pos = self.player.get_camera().eye;

        let mut chunks = self.chunks.iter().collect::<Vec<(&ChunkEntry, &Chunk)>>();
        chunks.sort_by(|a, b| {
            return a.1
                .dist_from(plr_camera_pos)
                .total_cmp(&b.1.dist_from(plr_camera_pos));
        });

        for chunk in &chunks
        {
            chunk.1.draw(&mut render_pass, window, self.player.get_camera());
        }

        drop(render_pass);
        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
        Ok(())
    }

    fn process(&mut self, event: ProspectEvent, window: &mut ProspectWindow) -> ProcessResponse {
        self.player.process(event, window);
        match event
        {
            ProspectEvent::KeyboardInput(Some(VirtualKeyCode::Escape), ElementState::Pressed) =>
            {
                *self.running.lock().unwrap() = false;

                let start_time = Instant::now();

                // Wait for Chunk Thread to stop
                while !*self.thread_has_stopped.lock().unwrap()
                {
                    if start_time.elapsed().as_secs_f32() > 2.00
                    {
                        println!("Unable to stop sub-thread, or it pre-maturely crashed!");
                        break;
                    }
                }

                // Chunk thread has stopped, can now Close the app
                return ProcessResponse::CloseApp;
            },
            ProspectEvent::KeyboardInput(Some(VirtualKeyCode::Q), ElementState::Pressed) =>
            {
                self.lock_player_pos = !self.lock_player_pos;
            },
            _ => {}
        }
        ProcessResponse::DontProcess
    }
}
