shader_type spatial;

global uniform vec2 position;
global uniform vec2 map_size;
global uniform vec2 display_size;
global uniform float verticality;
global uniform float smoothness;
global uniform sampler2D heightmap;

uniform vec2 mouse_position = vec2(0.5, 0.5);
uniform bool draw_mouse = false;

uniform vec3 upper_grass_color : source_color = vec3(0.0, 0.0, 0.0);
uniform vec3 lower_grass_color : source_color = vec3(0.36, 0.52, 0.30);
uniform vec3 cliff_color : source_color = vec3(0.4, 0.4, 0.10);

varying float height_val;
varying float height_variance;

const vec2 vec_array[] = {
	vec2(1, 0),
	vec2(-1, 0),
	vec2(0, 1),
	vec2(0, -1),
	vec2(1, 1),
	vec2(1, -1),
	vec2(-1, 1),
	vec2(-1, -1)
};

void vertex() {
	// GRID SETUP
	vec2 offset = vec2(mod(position.x, 600.0/64.0), mod(position.y, 600.0/64.0));
	vec2 new_position = position + offset;
	
	// HEIGHT CALC
	height_val = texture(heightmap, (UV * display_size + new_position)/map_size).r;
	height_variance = 0.0;
	
	// AVERAGE NEIGHBORS
	float sum = 0.0;
	for (int i = 0; i < 8; i++) {
		float nei_val = texture(heightmap, (UV * display_size + new_position + vec_array[i] * smoothness)/map_size).r;
		height_variance += pow(height_val - nei_val, 2); 
		sum += nei_val;
	}
	
	height_val = (height_val + sum)/9.0;
	height_variance = height_variance/8.0;
	
	VERTEX.y += height_val * verticality;
	VERTEX.xz += offset;
}

void fragment() {
	// TERRAIN PAINT
	if (distance(mouse_position, UV) < 0.01 && draw_mouse) {
		ALBEDO = vec3(1.0, 0.0, 0.0);
	} else {
		vec3 grass_color = mix(lower_grass_color, upper_grass_color, height_val * 2.0);
		ALBEDO = mix(grass_color, cliff_color, height_variance * 50.0);
	}
}
