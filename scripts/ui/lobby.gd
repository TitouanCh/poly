extends VBoxContainer

@onready var player_list_node = $player_list

var player_scene = preload("res://scenes/ui/game.tscn")
var player_nodes_list = []

func _ready():
	if Multiplayer:
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
	
	if user_in_game_id == 0:
		clear_players()
	
	create_player(ready, connected, user_in_game_id, spectator, user_string)

func create_player(ready, connected, user_in_game_id, spectator, username):
	var a = player_scene.instantiate()
	player_list_node.add_child(a)
	player_nodes_list.append(a)
	a._setup_as_player(ready, connected, user_in_game_id, spectator, username)

func clear_players():
	for player_node in player_nodes_list:
		player_node.queue_free()
	player_nodes_list = []
