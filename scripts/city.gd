extends Node3D

var camera

@onready var name_label = $ui_elements/city_name

func _ready():
	if get_parent():
		camera = get_parent().get_node("player").get_node("camera")

func _process(delta):
	if camera:
		# Display name of city
		name_label.position = camera.unproject_position(self.position) - name_label.size/2 + Vector2(0, 32)
