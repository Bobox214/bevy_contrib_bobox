use bevy::{
    core::Bytes,
    prelude::*,
    reflect::TypeUuid,
    render::shader::asset_shader_defs_system,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::{RenderResource, RenderResources},
        shader::{ShaderDefs, ShaderStage, ShaderStages},
    },
};

/// Allow to add an outline shader to a 'SpriteComponents'
/// Simply add an Handle<OutlineMaterial> to your entity , through Assets<OutlineMaterial>.add(OutlineMaterial{ .... })
pub struct Outline2dPlugin;

impl Plugin for Outline2dPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<OutlineMaterial>()
            .add_system(outlinable_sprite_system.system())
            .add_system_to_stage(
                stage::POST_UPDATE,
                asset_shader_defs_system::<OutlineMaterial>.system(),
            )
            .add_startup_system(setup_outline_pipeline.system());
    }
}
const SPRITE_OUTLINE_PIPELINE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0xdec90e721f4b9a25);

#[derive(Debug, RenderResources, ShaderDefs, TypeUuid)]
#[uuid = "9d8440bd-cb6c-4265-a00a-09cda3a271a7"]
pub struct OutlineMaterial {
    /// The configuration used to do the outlining
    pub configuration: OutlineConfiguration,
    /// When 'True' the outline will be drawn.
    /// The outline is always drawn in the 'inside' part of the sprite, for 3 pixels.
    #[render_resources(ignore)]
    #[shader_def]
    pub with_outline: bool,
}

impl Default for OutlineMaterial {
    fn default() -> Self {
        Self {
            configuration: OutlineConfiguration::default(),
            with_outline: false,
        }
    }
}

#[derive(Bytes, Debug, RenderResources, RenderResource, ShaderDefs, TypeUuid)]
#[uuid = "9d8440bd-cb6c-4265-a00a-09cda3a271a8"]
pub struct OutlineConfiguration {
    /// The color used to do the outlining
    pub color: Color,
    /// Width in pixel of the outlining
    pub width: u32,
    /// When 1, the outlining is done on the 'inside' part of the sprite.
    /// When 0, the outlining is done on the 'outside' part of the sprite.
    /// Note that doing the outlining on the 'outside' requires for best result a texture with at least 'width' pixels of alpha<alpha_threshold around the sprite itself.
    pub inside: u32,
    /// The alpha_threshold that defines which pixel in the sprite texture are 'inside' (higher than threshold) and 'outside' (lower than threshold).
    /// The outlining is done at the boundary between 'inside' and 'outside'
    pub alpha_threshold: f32,
}

impl Default for OutlineConfiguration {
    fn default() -> Self {
        Self {
            color: Color::RED,
            width: 3,
            inside: 1,
            alpha_threshold: 0.5,
        }
    }
}
/// Internal tag component to the outline_2d plugin.
/// It is added to each entity once, when the outline_pipeline has been added to the render_pipelines of this entity.
struct WithOutlinePipeline {}

fn setup_outline_pipeline(
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    render_graph.add_system_node(
        "outline_2d_pipeline",
        AssetRenderResourcesNode::<OutlineMaterial>::new(true),
    );

    render_graph
        .add_node_edge("outline_2d_pipeline", base::node::MAIN_PASS)
        .unwrap();

    pipelines.set_untracked(
        SPRITE_OUTLINE_PIPELINE,
        PipelineDescriptor {
            name: Some("outline_2d_pipeline".to_string()),
            ..PipelineDescriptor::default_config(ShaderStages {
                vertex: shaders.add(Shader::from_glsl(
                    ShaderStage::Vertex,
                    include_str!("outline_shader.vert"),
                )),
                fragment: Some(shaders.add(Shader::from_glsl(
                    ShaderStage::Fragment,
                    include_str!("outline_shader.frag"),
                ))),
            })
        },
    );
}

fn outlinable_sprite_system(
    commands: &mut Commands,
    mut query: Query<
        (Entity, Mut<RenderPipelines>),
        (Without<WithOutlinePipeline>, With<Handle<OutlineMaterial>>),
    >,
) {
    for (entity, mut render_pipelines) in query.iter_mut() {
        render_pipelines
            .pipelines
            .push(RenderPipeline::new(SPRITE_OUTLINE_PIPELINE.typed()));
        commands.insert_one(entity, WithOutlinePipeline {});
    }
}
