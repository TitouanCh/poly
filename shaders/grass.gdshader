shader_type particles;

// UGLY

global uniform vec2 position;
global uniform vec2 map_size;
global uniform float verticality;
global uniform float smoothness;
global uniform vec2 display_size;

uniform float items_per_row = 100;

global uniform sampler2D heightmap;

uniform sampler2D noisemap;
uniform vec2 noisemap_size = vec2(512, 512);

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

void process() {
	vec3 pos = vec3(0.0, 0.0, 0.0);
	float spacing = display_size.x / items_per_row;
	
	// POSITION
	pos.x = mod(float(INDEX), items_per_row) + spacing;
	pos.z = floor(float(INDEX) / items_per_row) + spacing;
	
	pos.x -= mod(position.x, spacing); 
	pos.z -= mod(position.y, spacing); 
	
	// RANDOMNESS
	vec2 test = vec2(mod(position.x, spacing), mod(position.y, spacing));
	pos.x -= texture(noisemap, (position + pos.xz)).r * 5.0;
	pos.z -= texture(noisemap, (position + pos.zx)).r * 5.0;
	
	// CENTER
//	pos.x -= (items_per_row - 1.0)/2.0;
//	pos.z -= (items_per_row - 1.0)/2.0;
	
	// SPACING
	pos *= spacing;
	
	// HEIGHT
	float height_val = texture(heightmap, (pos.xz + position)/map_size).r;
	float sum = 0.0;
	for (int i = 0; i < 8; i++) {
		float nei_val = texture(heightmap, (pos.xz + position + vec_array[i] * smoothness)/map_size).r;
		sum += nei_val;
	}
	
	height_val = (height_val + sum)/9.0;
	
	pos.y = height_val * verticality + 1.0;
	
	if (pos.y < 10.0 || pos.x < 0.0 || pos.z < 0.0 || pos.x > display_size.x || pos.z > display_size.y) {
		ACTIVE = false;
	}
	
	TRANSFORM[3][0] = pos.x ;
	TRANSFORM[3][1] = pos.y;
	TRANSFORM[3][2] = pos.z;
	
}
