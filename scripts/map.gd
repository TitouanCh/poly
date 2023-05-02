extends Node3D

var camera
var dimension = 600

@onready var continent = $continent
@onready var ocean = $ocean
@onready var grass = $grass

func _ready():
	if get_parent():
		camera = get_parent()

func _process(delta):
	if camera:
		RenderingServer.global_shader_parameter_set("position", Vector2(camera.position.x, camera.position.z))
