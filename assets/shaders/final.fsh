#version 450



// these are given by the rust code to naga:
//#define CHAR_WIDTH 10
//#define CHAR_HEIGHT 14
//#define SCREEN_CHAR_WIDTH 112
//#define SCREEN_CHAR_HEIGHT 60
#define CHAR_SIZE uvec2(CHAR_WIDTH, CHAR_HEIGHT)
#define SCREEN_CHAR_SIZE uvec2(SCREEN_CHAR_WIDTH, SCREEN_CHAR_HEIGHT)
#define SCREEN_SIZE (CHAR_SIZE * SCREEN_CHAR_SIZE)



layout(set = 0, binding = 0) uniform all_uniforms {
	
	uvec2 targetSize;
	vec2 invTargetSize;
	uint backgroundColorU32;
	
	float fishEyeAmount;
	float scalingFactor;
	float scanlineAmount;
	float scanlineScale;
	float bloomAmount;
	
};
layout(set = 1, binding = 0) uniform texture2D intermediate_tex;
layout(set = 1, binding = 1) uniform sampler linear_sampler;
#define intermediate sampler2D(intermediate_tex, linear_sampler)



layout(location = 0) out vec4 frag_color;



//Catmull-Rom sampling from Complementary Shaders, which was taken from Filmic SMAA presentation (https://advances.realtimerendering.com/s2016/Filmic%20SMAA%20v7.pptx)
// note: position is 0..targetSize
vec3 textureCatmullRom(vec2 position) {
	vec2 centerPosition = floor(position - 0.5) + 0.5;
	vec2 f = position - centerPosition;
	vec2 f2 = f * f;
	vec2 f3 = f * f2;
	
	float c = 0.5;
	vec2 w0 =        -c  * f3 +  2.0 * c         * f2 - c * f;
	vec2 w1 =  (2.0 - c) * f3 - (3.0 - c)        * f2         + 1.0;
	vec2 w2 = -(2.0 - c) * f3 + (3.0 -  2.0 * c) * f2 + c * f;
	vec2 w3 =         c  * f3 -                c * f2;
	
	vec2 w12 = w1 + w2;
	vec2 tc12 = (centerPosition + w2 / w12) / SCREEN_SIZE;
	
	vec2 tc0 = (centerPosition - 1.0) / SCREEN_SIZE;
	vec2 tc3 = (centerPosition + 2.0) / SCREEN_SIZE;
	vec4 color = vec4(texture(intermediate, vec2(tc12.x, tc0.y ), 0).rgb, 1.0) * (w12.x * w0.y ) +
				vec4(texture(intermediate, vec2(tc0.x,  tc12.y), 0).rgb, 1.0) * (w0.x  * w12.y) +
				vec4(texture(intermediate, vec2(tc12.x, tc12.y), 0).rgb, 1.0) * (w12.x * w12.y) +
				vec4(texture(intermediate, vec2(tc3.x,  tc12.y), 0).rgb, 1.0) * (w3.x  * w12.y) +
				vec4(texture(intermediate, vec2(tc12.x, tc3.y ), 0).rgb, 1.0) * (w12.x * w3.y );
	return color.rgb / color.a;
}

float invMix(float v, float a, float b) {
	return (v - a) / (b - a);
}



vec3 getBloomLevel(vec2 texelCoord, int mip_level) {
	vec2 texcoord = texelCoord / SCREEN_SIZE;
	vec3 total = vec3(0.0);
	total += texture(intermediate, texcoord + 1.0 / vec2(      SCREEN_SIZE.x,       SCREEN_SIZE.y), mip_level).rgb;
	total += texture(intermediate, texcoord + 1.0 / vec2(0.0 - SCREEN_SIZE.x,       SCREEN_SIZE.y), mip_level).rgb;
	total += texture(intermediate, texcoord + 1.0 / vec2(      SCREEN_SIZE.x, 0.0 - SCREEN_SIZE.y), mip_level).rgb;
	total += texture(intermediate, texcoord + 1.0 / vec2(0.0 - SCREEN_SIZE.x, 0.0 - SCREEN_SIZE.y), mip_level).rgb;
	return total * 0.25;
}



void main() {
	vec2 texelCoord = gl_FragCoord.xy;
	texelCoord -= ceil(targetSize * 0.5);
	texelCoord *= 1.0 + length(texelCoord / targetSize) * 0.1 * fishEyeAmount;
	texelCoord *= scalingFactor;
	texelCoord += floor(SCREEN_SIZE * 0.5);
	
	vec3 color = textureCatmullRom(texelCoord);
	color *= sin(texelCoord.y * scanlineScale) * (0.5 * scanlineAmount) + (1.0 - 0.5 * scanlineAmount);
	
	color += getBloomLevel(texelCoord, 1) * bloomAmount * (1.0 / 4.0);
	color += getBloomLevel(texelCoord, 2) * bloomAmount * (1.0 / 8.0);
	color += getBloomLevel(texelCoord, 3) * bloomAmount * (1.0 / 16.0);
	color += getBloomLevel(texelCoord, 4) * bloomAmount * (1.0 / 32.0);
	
	float backgroundRed = ((backgroundColorU32 & 0x0000FF) >> 0) / 255.0;
	float backgroundGreen = ((backgroundColorU32 & 0x00FF00) >> 8) / 255.0;
	float backgroundBlue = ((backgroundColorU32 & 0xFF0000) >> 16) / 255.0;
	vec3 backgroundColor = vec3(backgroundRed, backgroundGreen, backgroundBlue);
	backgroundColor = pow(backgroundColor, vec3(2.2));
	
	float backgroundAmount = 0.0;
	backgroundAmount += clamp(invMix(texelCoord.x, 0.0, -1.0), 0.0, 1.0);
	backgroundAmount += clamp(invMix(texelCoord.x, SCREEN_SIZE.x - 1, SCREEN_SIZE.x), 0.0, 1.0);
	backgroundAmount += clamp(invMix(texelCoord.y, 0.0, -1.0), 0.0, 1.0);
	backgroundAmount += clamp(invMix(texelCoord.y, SCREEN_SIZE.y - 1, SCREEN_SIZE.y), 0.0, 1.0);
	backgroundAmount = min(backgroundAmount, 1.0);
	color = mix(color, backgroundColor, backgroundAmount);
	
	frag_color = vec4(color, 1.0);
}
