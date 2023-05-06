shader_type spatial;

global uniform vec2 position;
global uniform vec2 map_size;
global uniform vec2 display_size;
global uniform float verticality;
global uniform float smoothness;
global uniform sampler2D heightmap;

uniform vec2 mouse_position = vec2(0.5, 0.5);
uniform bool draw_mouse = false;
uniform float mouse_outline = 0.002;

uniform vec3 upper_grass_color : source_color = vec3(0.0, 0.0, 0.0);
uniform vec3 lower_grass_color : source_color = vec3(0.36, 0.52, 0.30);
uniform vec3 cliff_color : source_color = vec3(0.4, 0.4, 0.10);

varying float height_val;
varying float height_variance;
varying vec2 calculated_offset;

uniform vec3 selected_points[32];

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
	calculated_offset = offset/display_size;
}

void fragment() {
	// TERRAIN PAINT
	vec3 grass_color = mix(lower_grass_color, upper_grass_color, height_val * 2.0);
	ALBEDO = mix(grass_color, cliff_color, height_variance * 50.0);
	
	// MOUSE UI PAINT
	if (draw_mouse) {
		// WHITE
		float distance_to_mouse = distance(mouse_position - calculated_offset, UV);
		bool whited = false;
		if (distance_to_mouse < 0.01) {
			ALBEDO = mix(ALBEDO, vec3(1.0, 1.0, 1.0), 0.4);
			whited = true;
		} else {
			// SELECTED POINTS DRAW
			for (int i = 0; i < selected_points.length(); i++) {
				if (selected_points[i] == vec3(0.0, 0.0, 0.0)) {
					break;
				}
				float distance_to_point = distance(selected_points[i].xz - calculated_offset, UV);
				if (distance_to_point < 0.005) {
					ALBEDO = mix(ALBEDO, vec3(1.0, 1.0, 1.0), 0.4);
					whited = true;
				}
			}
		}
		
		if (!whited) {
			// BLACK OUTLINES
			if (distance_to_mouse < 0.01 + mouse_outline) {
				ALBEDO = vec3(0.0, 0.0, 0.0);
			} else {
				for (int i = 0; i < selected_points.length(); i++) {
					if (selected_points[i] == vec3(0.0, 0.0, 0.0)) {
						break;
					}
					float distance_to_point = distance(selected_points[i].xz - calculated_offset, UV);
					if (distance_to_point < 0.005 + mouse_outline) {
						ALBEDO = vec3(0.0, 0.0, 0.0);
					}
				}
			}
		}
	}
}
