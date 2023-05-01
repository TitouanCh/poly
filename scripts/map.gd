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
#		continent.mesh.material.set_shader_parameter("position", Vector2(camera.position.x, camera.position.z))
#		ocean.mesh.material.set_shader_parameter("position", Vector2(camera.position.x, camera.position.z))
#		grass.process_material.set_shader_parameter("position", Vector2(camera.position.x, camera.position.z))
#		mesh.mesh.material.set_shader_parameter("mouse_position", Vector2(camera.mouse_projection.x/2 + 0.5, camera.mouse_projection.z/2 + 0.5))
	
