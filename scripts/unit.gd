extends Node3D

var camera
var terrain
var selected = false
var target = Vector3.ZERO
var speed = 20
 
@export var player : Player

@onready var heightmap : Image = Image.load_from_file("res://test_heightmap2_blurred.png")
@onready var name_label = $ui_elements/unit_name
@onready var mesh = $mesh

func _ready():
	if get_parent():
		camera = get_parent().get_node("player").get_node("camera")
	
	if player:
		terrain = player.get_node("terrain")
	

func _process(delta):
	# Display name of unit
	name_label.position = camera.unproject_position(self.position) - name_label.size/2 + Vector2(0, 32)
	
	if Input.is_action_just_pressed("left_click"):
		if get_viewport().get_mouse_position().distance_to(camera.unproject_position(self.position)) < 10.0:
			if !selected: select()
			else: unselect()
		elif selected:
			target = get_unprojected_mouse_position()
	
	if target != Vector3.ZERO:
		var direction = target - position
		position.x += direction.x * delta * speed
		position.z += direction.z * delta * speed
	
	# Height
	if heightmap:
		position.y = get_height(Vector2(position.x, position.z), heightmap) * 40

func select():
	selected = true
	mesh.set_instance_shader_parameter("color",  Color(255, 255, 255))
	mesh.set_instance_shader_parameter("outline_thickness", 0.4)
	terrain.selected_nodes.append(self)

func unselect():
	selected = false
	mesh.set_instance_shader_parameter("color",  Color(0, 0, 0))
	mesh.set_instance_shader_parameter("outline_thickness", 0.2)
	terrain.selected_nodes.erase(self)

func get_unprojected_mouse_position():
	var vec = camera.project_ray_normal(get_viewport().get_mouse_position())
	var alpha = -camera.position.y/vec.y
	var point = Vector3(vec.x * alpha, -camera.position.y, vec.z * alpha) + camera.position + player.position
	return point

func get_height(coord, heightmap : Image):
	var dimensions = heightmap.get_size()
	coord = coord + Vector2(300, 300)
	return heightmap.get_pixel(coord.x, coord.y).r
