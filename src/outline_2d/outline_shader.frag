#version 450

layout(location = 0) in vec2 v_Uv;
layout(location = 1) in vec2 v_size;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

# ifdef COLORMATERIAL_TEXTURE 
layout(set = 1, binding = 1) uniform texture2D ColorMaterial_texture;
layout(set = 1, binding = 2) uniform sampler ColorMaterial_texture_sampler;
# endif

layout(set = 3, binding = 0) uniform OutlineMaterial_configuration {
    vec4 OutlineColor;
    int width;
    int inside;
    float OutlineAlphaThreshold;
};

void main() {
    bool outline = false;
    vec4 color = Color;
# ifdef COLORMATERIAL_TEXTURE
    color *= texture(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),v_Uv);
# endif
# ifdef OUTLINEMATERIAL_WITH_OUTLINE
    if (inside==1 && color.a>OutlineAlphaThreshold) {
        float dx = width/v_size.x;
        float dy = width/v_size.y;
        outline = (
            (v_Uv.x+dx>1.0)
        ||  (v_Uv.x-dx<0.0)
        ||  (v_Uv.y+dy>1.0)
        ||  (v_Uv.y-dy<0.0)
# ifdef COLORMATERIAL_TEXTURE
        ||  (texture(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),vec2(v_Uv.x+dx,v_Uv.y)).a<OutlineAlphaThreshold)
        ||  (texture(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),vec2(v_Uv.x-dx,v_Uv.y)).a<OutlineAlphaThreshold)
        ||  (texture(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),vec2(v_Uv.x,v_Uv.y+dy)).a<OutlineAlphaThreshold)
        ||  (texture(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),vec2(v_Uv.x,v_Uv.y-dy)).a<OutlineAlphaThreshold)
# endif
        );
    } else if (inside==0) {
        float dx = width/v_size.x;
        float dy = width/v_size.y;
        if (color.a<OutlineAlphaThreshold) {
# ifdef COLORMATERIAL_TEXTURE
            outline = (
                (texture(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),vec2(v_Uv.x+dx,v_Uv.y)).a>OutlineAlphaThreshold)
            ||  (texture(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),vec2(v_Uv.x-dx,v_Uv.y)).a>OutlineAlphaThreshold)
            ||  (texture(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),vec2(v_Uv.x,v_Uv.y+dy)).a>OutlineAlphaThreshold)
            ||  (texture(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),vec2(v_Uv.x,v_Uv.y-dy)).a>OutlineAlphaThreshold)
            );
# endif
        } else {
            outline = (
                (v_Uv.x+dx>1.0)
            ||  (v_Uv.x-dx<0.0)
            ||  (v_Uv.y+dy>1.0)
            ||  (v_Uv.y-dy<0.0)
            );
        }
    }
# endif
    o_Target = outline ? OutlineColor : vec4(0.0);
}