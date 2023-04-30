shader_type spatial;

uniform vec2 position = vec2(2000.0, 2000.0);
uniform vec2 map_size = vec2(8192.0, 4096.0);
uniform float map_scale = 20.0;

uniform vec2 mouse_position = vec2(0.5, 0.5);
uniform bool draw_mouse = false;

uniform vec3 color1 : source_color = vec3(0.0, 0.0, 0.0);
uniform vec3 color2 : source_color = vec3(0.36, 0.52, 0.30);

uniform sampler2D heightmap;
varying float height_val;

void vertex() {
	height_val = texture(heightmap, UV / map_scale + (position/map_size)).r;
	VERTEX.y += height_val * 20.0;
}

void fragment() {
	if (distance(mouse_position, UV) < 0.01 && draw_mouse) {
		ALBEDO = vec3(1.0, 0.0, 0.0);
	} else {
		ALBEDO = vec3(mix(color1, color2, height_val / 0.5));
	}
}
