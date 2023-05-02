shader_type particles;

uniform float max_distance_to_city = 3.0;
uniform float min_distance_to_city = 3.0;

uniform sampler2D noisemap;
uniform vec2 noisemap_size = vec2(512, 512);


void process() {
	// GET RANDOM VALUES FROM NOISEMAP
	float rotation = texture(noisemap, vec2(float(INDEX), 1.0) / noisemap_size).r * 2.0 * 3.14;
	float distance_to_city = (texture(noisemap, vec2(float(INDEX), 2.0) / noisemap_size).r * (max_distance_to_city - min_distance_to_city)) + min_distance_to_city;
	vec3 direction = vec3(
		texture(noisemap, vec2(float(INDEX), 3.0) / noisemap_size).r - 0.5,
		0.0,
		texture(noisemap, vec2(float(INDEX), 4.0) / noisemap_size).r - 0.5
	);
	
	// NORMALIZATION
	direction /= distance(vec3(0.0, 0.0, 0.0), direction);
	
	// POSITION
	TRANSFORM[3].xyz = direction * distance_to_city * 2.0;
	
	// ROTATION
	TRANSFORM[2][0] = cos(rotation);
	TRANSFORM[2][1] = 0.0;
	TRANSFORM[2][2] = sin(rotation);
	
	TRANSFORM[0][0] = sin(rotation);
	TRANSFORM[0][1] = 0.0;
	TRANSFORM[0][2] = -cos(rotation);
}
