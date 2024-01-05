# Prospect
Rendering Engine for rust

Check Examples
- examples/obj_realtime_updater - a project that watches an object file and automatically updates when it gets changed
- examples/simple_terrain_gen - generates a large chunk  of terrain
- examples/voxel_engine - a voxel engine with mutli-threading and 3 dimensional chunks and auto chunk loading
- examples/barebones.rs - barebones example
- examples/hello_world.rs - most basic
- examples/test.rs - what i use for developing the engine

# TODO
Redo the entire system of BindGroups & Shaders.

Why?

Because Currently if shader becomes unused, the shader manager still maintains ownership of the shader,
if you create enough shaders and drop enough Indexes, you will get completely unused shaders that will take up
valuable space in the HashMap

Current System:
Pros:
- Simple
- Easy To Use
- You can avoid using it by creating custom Meshables

Cons:
- Not Extensible
- Easy to Accidentally Create Memory Leaks

New System Ideas:

It should reduce the load on the end user, making it easier to read.

## Reference Counting
Interior Reference Counting

Main Source
```rust
let shader = ExampleShader::new();

let mesh = Mesh::new(..., shader.clone())
```

Shader
```rust
pub struct ExampleShader
{
    render_pipeline : Rc<RenderPipeline>
}

impl Clone for ExampeShader
{
    fn clone(&self) -> Self
    {
        Self
        {
            render_pipeline : self.render_pipeline.clone()
        }
    }
}
```

What about BindGroups?
for example shaders could now do something like

```rust
pub struct TexturedShader
{
    render_pipeline : Rc<RenderPipeline>,
    texture : Rc<BindGroup>
}
```

Pros:
- No Abstractions in the middle of Hashing that hurt peformance
- Lighter Weight
- Easily Clonable
- Should decrease startup times
- Super Readable
- Cleaner Code

Cons:
- Textures are not mutable, however they could be with mutexes (```Rc<Mutex<BindGroup>>```)

To address cons it could also be changed to 

Maybe something like

User code
```rust
let shader = ShaderExample::new();

// In Theory you dont even need to have a mutable reference to shader to change the texture due to Mutexes Interior Mutabality
shader.change_texture_to(another_image);

let mesh = Mesh::new(shader.clone());
```

Abstractions
```rust

pub struct ShaderExample
{
    render_pipeline : SmartRenderPipeline,
    texture : SmartBindGroup
}

#[derive(Clone, Debug)]
pub struct SmartBindGroup
{
    inner : Rc<Mutex<BindGroup>>
}

impl SmartBindGroup
{
    /// Haven't Checked the validity that this has, but looks good in concept
    pub fn change(&mut self, to : BindGroup)
    {
        let handle = self.inner.lock();
        if let Some(mut handle) = handle
        {
            *handle = to;
        }
    }
}

#[derive(Clone, Debug)]
pub struct SmartRenderPipeline
{
    inner : Rc<RenderPipeline> // Would probably be wise to not make this Mutable
}
```

I haven't tested any of this within rust so it may throw the borrow checker off (maybe wgpu-rs wont like it?)

However it looks good.