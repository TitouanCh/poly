extends Node3D

var camera
var dimension = 600

@onready var mesh = $mesh

func _ready():
	if get_parent():
		camera = get_parent()

func _process(delta):
	if camera:
		mesh.mesh.material.set_shader_parameter("position", Vector2(camera.position.x, camera.position.z))
#		mesh.mesh.material.set_shader_parameter("mouse_position", Vector2(camera.mouse_projection.x/2 + 0.5, camera.mouse_projection.z/2 + 0.5))
	
