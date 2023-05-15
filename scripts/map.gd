extends Node3D

class_name Terrain

var player
var dimension = 600
var selected_nodes = []

@onready var continent = $continent
@onready var ocean = $ocean
@onready var grass = $grass

func _ready():
	if get_parent():
		player = get_parent()

func _process(delta):
	if player:
		RenderingServer.global_shader_parameter_set("position", Vector2(player.position.x, player.position.z))
		continent.mesh.material.set_shader_parameter("mouse_position", Vector2(player.mouse_projection.x/2 + 0.5, player.mouse_projection.z/2 + 0.5))
	
	var arr : PackedVector3Array
	for node in selected_nodes:
		var vec = (Vector2(node.position.x, node.position.z) - Vector2(player.position.x, player.position.z)) / (dimension/2) + Vector2.ONE
		vec /= 2
		arr.append(Vector3(vec.x, 0, vec.y))
	
	continent.mesh.material.set_shader_parameter("selected_points", arr)
