extends VBoxContainer

@onready var player_list_node = $player_list
@onready var ready_button = $button_bar/ready
@onready var launch_button = $button_bar/launch
@onready var cancel_button = $button_bar/cancel

var player_scene = preload("res://scenes/ui/game.tscn")
var player_nodes_hash = {}

func _ready():
	if Multiplayer:
		ready_button.pressed.connect(Multiplayer._send_ready)
		launch_button.pressed.connect(Multiplayer._send_launch)
		cancel_button.pressed.connect(Multiplayer._send_leave)
		Multiplayer.received_lobby_state.connect(set_player)

func set_player(info_bytes, user_string):
	# info_bytes format:
	# [0]: ready
	# [1]: connected
	# [2-5]: user_in_game_id
	# [6]: spectator
	
	var ready = info_bytes[0] as bool  
	var connected = info_bytes[1] as bool
	var user_in_game_id = info_bytes.decode_u32(2)
	var spectator = info_bytes[6]
	
	clear_player(user_in_game_id)
	
	create_player(ready, connected, user_in_game_id, spectator, user_string)

func create_player(ready, connected, user_in_game_id, spectator, username):
	var a = player_scene.instantiate()
	player_list_node.add_child(a)
	player_nodes_hash[user_in_game_id] = a
	a._setup_as_player(ready, connected, user_in_game_id, spectator, username)

func clear_players():
	for player_node in player_nodes_hash:
		player_node.queue_free()
	player_nodes_hash = []

func clear_player(id):
	var old = player_nodes_hash.get(id)
	if old:
		old.queue_free()
		player_nodes_hash.erase(id)
