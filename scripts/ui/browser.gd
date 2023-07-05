extends Control

var game_scene = preload("res://scenes/ui/game.tscn")
var game_list = []

@onready var game_list_node = $sbox/game_list
@onready var create_game_button = $button_bar/create_game

signal create_game_button_pressed
signal create_game(game_name: String, max_players: int)
signal send_join_game(game_id)

func _ready():
	if Multiplayer:
		create_game.connect(Multiplayer._send_create_game)
		send_join_game.connect(Multiplayer._send_join_game)
		Multiplayer.received_browser_state.connect(bytes_to_games)
	
	create_game_button.pressed.connect(create_game_clicked)

func _process(delta):
	if Input.is_action_just_pressed("shortcut_c"):
		create_game_button_pressed.emit()

func bytes_to_games(array: PackedByteArray):
	if len(array) == 0: return
	var games = []
	var i = 0
	while i < len(array):
		var game_id = array.decode_u32(i + 0)
		var game_name = array.slice(i + 4, i + 28).get_string_from_utf8()
		var game_phase = array.decode_u32(i + 28)
		var number_of_cities = array.decode_u32(i + 32)
		var maximum_number_of_players = array.decode_u32(i + 36)
		var number_of_players = array.decode_u32(i + 40) 
		i += 44
		print(game_id, ' ', len(game_name), ' ', game_phase, ' ', number_of_cities, ' ', number_of_players, ' ', maximum_number_of_players)
		games.append([game_id, game_name, number_of_players, maximum_number_of_players, game_phase])
	
	receive_games(games)

func receive_games(list_of_games):
	clear_games()
	# Each game is formatted this way : game_id, game_title, number_of_players, max_number_of_players
	for game in list_of_games:
		var a = game_scene.instantiate()
		game_list_node.add_child(a)
		game_list.append(a)
		a._setup(game[0], game[1], game[2], game[3], game[4])
		a.join.connect(join_game_clicked)

func clear_games():
	for game in game_list:
		game.queue_free()
	game_list = []

func create_game_clicked():
	create_game_button_pressed.emit()

func create_game_received(game_name: String, max_players: int):
	create_game.emit(game_name, max_players)

func join_game_clicked(id):
	send_join_game.emit(id)
