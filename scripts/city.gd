extends Node3D

var camera

func _ready():
	if get_parent():
		camera = get_parent().get_node("camera")
		print(camera)
