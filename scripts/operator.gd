extends Node

class_name Operator

enum phase {STANDBY, PLACEMENT, INGAME}

var current_phase = phase.STANDBY
var built_frame0 = {
	"units": {}
}
var built_frame1 = {}

# For now, only one selection at the time
var selected_node : Prop = null

@onready var renderer : Renderer = $flask/ether1/renderer 

func _ready():
	if Multiplayer:
		Multiplayer.connect("received_start_game", received_start_game)
		Multiplayer.connect("received_frame_data", received_frame_data)

func _process(delta):
	if current_phase == phase.STANDBY:
		_process_standby(delta)
	elif current_phase == phase.PLACEMENT:
		_process_placement(delta)
	elif current_phase == phase.INGAME:
		_process_ingame(delta)

func prop_clicked(prop : Prop):
	# Unselect previous selection
	if selected_node: selected_node.unselect()
	
	# Select newly clicked prop
	prop.select()
	selected_node = prop

func _process_standby(delta):
	if Input.is_action_just_pressed("start_game"):
		Multiplayer._send_start_game()

func _process_placement(delta):
	if Input.is_action_just_pressed("left_click"):
		Multiplayer._send_placed_city(vec3_to_vec2i(renderer.get_unprojected_mouse_position()))
		renderer.set_city(1, "Test", vec3_to_vec2i(renderer.get_unprojected_mouse_position()), true)

func _process_ingame(delta):
	if Input.is_action_just_pressed("right_click"):
		if selected_node is Unit:
			selected_node.target = renderer.get_unprojected_mouse_position()

func received_start_game():
	if current_phase == phase.STANDBY:
		print("Received start game")
		current_phase = phase.PLACEMENT

func received_frame_data(frame_data):
	var identifier = frame_data.slice(0, 1).get_string_from_utf8()
	# Unit
	if identifier == "u":
		frame_data = frame_data.slice(1, len(frame_data))
		print(frame_data)
		var built_unit = Multiplayer.decode_unit(frame_data)

func vec3_to_vec2i(vector3 : Vector3) -> Vector2i:
	return Vector2i(int(vector3.x), int(vector3.z))
