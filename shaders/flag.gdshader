shader_type spatial;

void vertex() {
	VERTEX.z += cos(TIME + UV.x) * UV.x;
}

void fragment() {
	ALBEDO = vec3(1.0, 0.5, 0.0);
}