extends Node3D

var camera
var terrain
var selected = false

@export var player : Player

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
