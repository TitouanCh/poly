extends Prop

class_name City

@export var city_name = "Orsay"

@onready var name_label = $ui_elements/city_name
@onready var houses = $houses
@onready var noise = FastNoiseLite.new()

func _initialize():
	for child in $fort.get_children():
		if child is MeshInstance3D:
			meshes.append(child)
	setup_houses()

func _update(delta):
	if camera:
		# Display name of city
		name_label.position = camera.unproject_position(self.position) - name_label.size/2 + Vector2(0, 32)

func _to_string() -> String:
	return "City | {2} | position: {0} | selected: {1}".format([position, is_selected, city_name])

func setup_houses():
	randomize()
	noise.seed = randi()
	noise.frequency = 1.0
	houses.process_material.set_shader_parameter("noisemap", ImageTexture.create_from_image(noise.get_image(512, 512)))
	houses.emitting = true
