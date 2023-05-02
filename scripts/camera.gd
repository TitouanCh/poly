extends Node3D

var speed = 200
var mouse_projection = Vector3(0, 0, 0)
@onready var terrain = $terrain
@onready var camera = $camera

func _ready():
	#Input.set_mouse_mode(Input.MOUSE_MODE_HIDDEN)
	pass

func _process(delta):
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
	
	update_mouse_projection()
	
	if Input.is_action_pressed("zoom_in"):
		camera.fov += delta * 15.0
	if Input.is_action_pressed("zoom_out"):
		camera.fov -= delta * 15.0

func update_mouse_projection():
	var vec = camera.project_ray_normal(get_viewport().get_mouse_position())
	var alpha = -camera.position.y/vec.y
	var point = Vector3(vec.x * alpha, -camera.position.y, vec.z * alpha) + camera.position
	mouse_projection = point/(terrain.dimension/2)
