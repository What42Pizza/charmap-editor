#version 450

layout(location = 0) in vec2 texcoord;

layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform texture2D higher_mip_tex;
layout(set = 0, binding = 1) uniform sampler filtering_sampler;
#define higher_mip sampler2D(higher_mip_tex, filtering_sampler)

void main() {
	vec2 pixelSize = 1.0 / textureSize(higher_mip, 0);
	frag_color = vec4(0.0);
	frag_color += texture(higher_mip, texcoord + vec2( 2.0,  0.0) * pixelSize);
	frag_color += texture(higher_mip, texcoord + vec2(-2.0,  0.0) * pixelSize);
	frag_color += texture(higher_mip, texcoord + vec2( 0.0,  2.0) * pixelSize);
	frag_color += texture(higher_mip, texcoord + vec2( 0.0, -2.0) * pixelSize);
	frag_color += texture(higher_mip, texcoord + vec2( 1.0,  1.0) * pixelSize);
	frag_color += texture(higher_mip, texcoord + vec2(-1.0,  1.0) * pixelSize);
	frag_color += texture(higher_mip, texcoord + vec2( 1.0, -1.0) * pixelSize);
	frag_color += texture(higher_mip, texcoord + vec2(-1.0, -1.0) * pixelSize);
	frag_color += texture(higher_mip, texcoord) * 4.0;
	frag_color /= 12.0;
}
