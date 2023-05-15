extends Node3D

class_name Player

@export var speed = 200
var mouse_projection = Vector3(0, 0, 0)

@onready var terrain : Terrain = $terrain
@onready var camera : Camera3D = $camera

func _process(delta):
	# MOVEMENT
	var inputs = Vector2.ZERO
	
	if Input.is_action_pressed("up"):
		inputs.y -= 1
	if Input.is_action_pressed("down"):
		inputs.y += 1
	if Input.is_action_pressed("left"):
		inputs.x -= 1
	if Input.is_action_pressed("right"):
		inputs.x += 1
		
	position.x += speed * inputs.x * delta
	position.z += speed * inputs.y * delta
	
	# MOUSE IN TERRAIN POSITION
	update_mouse_projection()
	print(mouse_projection)
	
	# ZOOM
	if Input.is_action_pressed("zoom_in"):
		camera.fov += delta * 15.0
	if Input.is_action_pressed("zoom_out"):
		camera.fov -= delta * 15.0

func update_mouse_projection():
	var vec = camera.project_ray_normal(get_viewport().get_mouse_position())
	var alpha = -camera.position.y/vec.y
	var point = Vector3(vec.x * alpha, -camera.position.y, vec.z * alpha) + camera.position
	mouse_projection = point/(terrain.dimension/2)

