extends Node3D

class_name Renderer

@export var player : Player

func get_unprojected_mouse_position() -> Vector3:
	var vec = player.camera.project_ray_normal(get_viewport().get_mouse_position())
	var alpha = -player.camera.position.y/vec.y
	var point = Vector3(vec.x * alpha, -player.camera.position.y, vec.z * alpha) + player.camera.position + player.position
	return point
