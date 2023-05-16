extends Node

class_name Operator

enum phase {STANDBY, PLACEMENT, INGAME}

var current_phase = phase.STANDBY
# For now, only one selection at the time
var selected_node : Prop = null

@onready var renderer : Renderer = $flask/ether/renderer 

func _process(delta):
	if Input.is_action_just_pressed("right_click"):
		if selected_node is Unit:
			selected_node.target = renderer.get_unprojected_mouse_position()
	
	if Input.is_action_just_pressed("start_game"):
		Multiplayer._send_start_game()

func prop_clicked(prop : Prop):
	# Unselect previous selection
	if selected_node: selected_node.unselect()
	
	# Select newly clicked prop
	prop.select()
	selected_node = prop
