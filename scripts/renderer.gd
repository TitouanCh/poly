extends Node3D

class_name Renderer

@export var player: Player
@export var city_scene: PackedScene
@export var unit_scene: PackedScene 

@export var cities_pool_number = 12
var cities : Array[City] = []

var units : Dictionary = {} # idx | Unit

var operator : Operator

func _ready():
	setup_city_pool(cities_pool_number)
	
	if Multiplayer:
		Multiplayer.connect("received_city", set_city)
	
	if get_parent(): if get_parent(): if get_parent():
		operator = get_parent().get_parent().get_parent()
		operator.connect("set_unit", set_unit)
	
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

func spawn_unit(idx, built_unit):
	var unit = unit_scene.instantiate()
	add_child(unit)
	units[idx] = unit

func set_unit(idx_built_unit):
	var unit_idx = idx_built_unit[0]
	var built_unit = idx_built_unit[1]
	if !units.keys().has(unit_idx):
		spawn_unit(unit_idx, built_unit)
	# ------------- Built unit format -------------- :
	# idx: float | Array: [ n: int, team: int, current_position: Vector2, center_of_mass: Vector2,
	#                       current_angle: float, incombat: bool, soldier_alive: int, soldiers_combat, [Soldiers], [Orders] ]
	var PSposition: Array[Vector2] = []
	for soldier in built_unit[8]:
		PSposition.append(soldier[0])
	units[unit_idx].set_param(built_unit[2], built_unit[4], built_unit[3], built_unit[1], built_unit[5], built_unit[6], built_unit[7], PSposition)

func unit_attack(attacker: Unit, defender: Unit):
	if attacker and defender:
		var direction = (attacker.position - defender.position).normalized()
		var distance = defender.position.distance_to(attacker.position)
		attacker.target = attacker.position - direction * (distance - 20)
		attacker.target_rotation = direction.angle_to(Vector3.ONE)
		defender.target_rotation = -direction.angle_to(Vector3.ONE)
		defender.attacked_by(attacker)
