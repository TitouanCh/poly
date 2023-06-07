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
	
#	set_city(1, "Goeogie", Vector2(1050, 1050), true)

func _process(delta):
	# Test attack
#	if Input.is_action_just_pressed("space"):
#		unit_attack($unit, $unit4)
	pass

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

func set_city(id, city_name, city_position, just_created = false):
	if (id < cities_pool_number):
		cities[id].set_active()
		cities[id].set_param(id, city_name, city_position, just_created)

func unit_attack(attacker: Unit, defender: Unit):
	if attacker and defender:
		var direction = (attacker.position - defender.position).normalized()
		var distance = defender.position.distance_to(attacker.position)
		attacker.target = attacker.position - direction * (distance - 20)
		attacker.target_rotation = direction.angle_to(Vector3.ONE)
		defender.target_rotation = -direction.angle_to(Vector3.ONE)
		defender.attacked_by(attacker)
