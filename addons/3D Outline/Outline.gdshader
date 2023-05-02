shader_type spatial;
render_mode unshaded, cull_front;

instance uniform bool enable = true; // on and off switsch to diesable/enable the outline
// outline costumization
instance uniform float outline_thickness = 0.2; // how thick is the outline?
instance uniform vec4 color : source_color = vec4(0.0); // which color does the outline have?


void vertex() {
	if (enable) {
	VERTEX += NORMAL*outline_thickness; // apply the outlines thickness	
	}
}

void fragment() {
	if (enable) {
	ALBEDO = color.rgb; // apply the outlines color
	}
}