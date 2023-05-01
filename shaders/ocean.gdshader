shader_type spatial;

uniform vec3 ocean_color : source_color = vec3(0.5, 0.5, 0.5); 

uniform vec2 position = vec2(2000.0, 2000.0);
uniform vec2 map_size = vec2(4096.0, 4096.0);

uniform sampler2D heightmap;
uniform float map_scale = 4.0;

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
	float height_val = texture(heightmap, UV / map_scale + (position/map_size)).r;
	
	// AVERAGE NEIGHBORS
	float sum = 0.0;
	for (int i = 0; i < 8; i++) {
		float nei_val = texture(heightmap, (UV + 40.0/2048.0 * vec_array[i]) / map_scale + (position/map_size)).r;
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
