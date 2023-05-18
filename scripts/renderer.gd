extends Node3D

class_name Renderer

@export var player : Player
@export var city_scene : PackedScene

@export var cities_pool_number = 12
var cities : Array[City] = []

func _ready():
	setup_city_pool(cities_pool_number)
	
	if Multiplayer:
		Multiplayer.connect("received_city", set_city)

func get_unprojected_mouse_position() -> Vector3:
	var vec = player.camera.project_ray_normal(get_viewport().get_mouse_position())
	var alpha = -player.camera.position.y/vec.y
	var point = Vector3(vec.x * alpha, -player.camera.position.y, vec.z * alpha) + player.camera.position + player.position
	return point

func setup_city_pool(number):
	for i in range(number):
		spawn_city()

func spawn_city():
	var city = city_scene.instantiate()
	add_child(city)
	cities.append(city)
	city.set_unactive()

func set_city(city_data : PackedStringArray):
	# Decode
	var id = int(city_data[0])
	var coord = Vector2(float(city_data[1]), float(city_data[2]))
	
	# Setup
	if (id < cities_pool_number):
		cities[id].set_active()
		cities[id].set_coord(coord)
