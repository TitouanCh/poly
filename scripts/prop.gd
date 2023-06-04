extends Node3D

class_name Prop

var player : Player
var camera : Camera3D
var terrain : Terrain
var renderer : Renderer
var meshes : Array = []

var is_selected : bool = false

@export var clickable : bool = true
@export var clickable_distance : float = 10.0
@export var on_floor : bool = true

@onready var heightmap : Image = Image.load_from_file("res://test_heightmap2_blurred.png")
@onready var verticality : float = 40.0

signal clicked(myself : Prop)

func _ready():
	# Get useful other nodes from renderer
	if get_parent() is Renderer:
		player = get_parent().player
		camera = get_parent().player.camera
		terrain = get_parent().player.terrain
		renderer = get_parent()
	
	# Connect to operator : temporary
	if get_parent().get_parent():
		clicked.connect(get_tree().root.get_node("Operator").prop_clicked)
	
	_initialize()

func _initialize():
	pass

func _process(delta):
	# Check if prop gets clicked
	if clickable:
		if Input.is_action_just_pressed("left_click"):
			if get_viewport().get_mouse_position().distance_to(camera.unproject_position(self.position)) < clickable_distance:
				clicked.emit(self)
	
	# Height
	if heightmap and on_floor:
		position.y = get_height_at_position()
	
	_update(delta)

func _update(delta):
	pass

func _to_string() -> String:
	return "Prop | position: {0} | selected: {1}".format([position, is_selected])

func get_height(coord, heightmap : Image):
	var dimensions = heightmap.get_size()
	coord = coord + Vector2(300, 300)
	return heightmap.get_pixel(coord.x, coord.y).r

func get_height_at_position():
	return get_height(Vector2(position.x, position.z), heightmap) * verticality + 2.0

func set_active():
	visible = true

func set_unactive():
	visible = false

func set_coord(coord : Vector2):
	position.x = coord.x
	position.z = coord.y

func select():
	is_selected = true
	for mesh in meshes:
		mesh.set_instance_shader_parameter("color",  Color(255, 255, 255))
		mesh.set_instance_shader_parameter("outline_thickness", 0.4)
	terrain.selected_nodes.append(self)

func unselect():
	is_selected = false
	for mesh in meshes:
		mesh.set_instance_shader_parameter("color",  Color(0, 0, 0))
		mesh.set_instance_shader_parameter("outline_thickness", 0.2)
