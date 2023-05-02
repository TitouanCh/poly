extends Node3D

var camera

@onready var name_label = $ui_elements/city_name
@onready var houses = $houses
@onready var noise = FastNoiseLite.new()

func _ready():
	if get_parent():
		camera = get_parent().get_node("player").get_node("camera")
	
	# Setup houses
	randomize()
	noise.seed = randi()
	noise.frequency = 1.0
	houses.process_material.set_shader_parameter("noisemap", ImageTexture.create_from_image(noise.get_image(512, 512)))
	houses.emitting = true

func _process(delta):
	if camera:
		# Display name of city
		name_label.position = camera.unproject_position(self.position) - name_label.size/2 + Vector2(0, 32)
