shader_type spatial;

uniform vec3 ocean_color : source_color = vec3(0.5, 0.5, 0.5); 

global uniform vec2 position;
global uniform vec2 map_size;
global uniform vec2 display_size;
global uniform float smoothness;

global uniform sampler2D heightmap;

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
	VERTEX.y = cos(TIME)/2.0;
}

void fragment() {
	float height_val = texture(heightmap, (UV * display_size + position)/map_size).r;
	
	// AVERAGE NEIGHBORS
	float sum = 0.0;
	for (int i = 0; i < 8; i++) {
		float nei_val = texture(heightmap, (UV * display_size + position + vec_array[i] * smoothness)/map_size).r;
		sum += nei_val;
	}
	
	height_val = (height_val + sum)/9.0;
	float foam = height_val;
	
	if (foam > 0.11 + (cos(TIME)/20.0 * 0.1)) {
		ALBEDO = mix(ocean_color, vec3(1.0, 1.0, 1.0), foam * abs(cos(TIME)/2.0) + 0.5);
		ALPHA = 0.9;
	} else {
		ALBEDO = ocean_color;
		ALPHA = 0.4;
	}
}
