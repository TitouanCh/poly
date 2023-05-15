extends Node

class_name Operator

# For now, only one selection at the time
var selected_node : Prop = null

@onready var renderer : Renderer = $flask/ether/renderer 

func _process(delta):
	if Input.is_action_just_pressed("right_click"):
		if selected_node is Unit:
			selected_node.target = Vector3(renderer.player.mouse_projection.x, 0, renderer.player.mouse_projection.y)
			print("test")

func prop_clicked(prop : Prop):
	# Unselect previous selection
	if selected_node: selected_node.unselect()
	
	# Select newly clicked prop
	prop.select()
	selected_node = prop
	
