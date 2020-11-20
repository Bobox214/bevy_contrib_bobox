use bevy::{
    prelude::*,
    render::shader::asset_shader_defs_system,
    render::{
        pipeline::{DynamicBinding, PipelineDescriptor, PipelineSpecialization, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderDefs, ShaderStage, ShaderStages},
    },
    type_registry::TypeUuid,
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
const SPRITE_OUTLINE_PIPELINE: Handle<PipelineDescriptor> =
    Handle::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0xdec90e721f4b9a25);

#[derive(Debug, RenderResources, ShaderDefs, TypeUuid)]
#[uuid = "9d8440bd-cb6c-4265-a00a-09cda3a271a7"]
pub struct OutlineMaterial {
    /// The color of used to do the outlining
    pub color: Color,
    /// When 'True' the outline will be drawn.
    /// The outline is always drawn 'inside' the sprite
    #[render_resources(ignore)]
    #[shader_def]
    pub with_outline: bool,
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
        PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("outline_shader.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("outline_shader.frag"),
            ))),
        }),
    );
}

fn outlinable_sprite_system(
    mut commands: Commands,
    mut query: Query<
        Without<WithOutlinePipeline, With<Handle<OutlineMaterial>, (Entity, Mut<RenderPipelines>)>>,
    >,
) {
    for (entity, mut render_pipelines) in query.iter_mut() {
        render_pipelines.pipelines.push(RenderPipeline::specialized(
            SPRITE_OUTLINE_PIPELINE,
            PipelineSpecialization {
                dynamic_bindings: vec![
                    // Transform
                    DynamicBinding {
                        bind_group: 2,
                        binding: 0,
                    },
                    // Sprite
                    DynamicBinding {
                        bind_group: 2,
                        binding: 1,
                    },
                    // Outline Color
                    DynamicBinding {
                        bind_group: 2,
                        binding: 2,
                    },
                    // Outline Color
                    DynamicBinding {
                        bind_group: 2,
                        binding: 3,
                    },
                ],
                ..Default::default()
            },
        ));
        commands.insert_one(entity, WithOutlinePipeline {});
    }
}
