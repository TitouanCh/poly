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
	create_game_button.pressed.connect(create_game_clicked)

func _process(delta):
	if Input.is_action_just_pressed("shortcut_c"):
		create_game_button_pressed.emit()

func receive_games(list_of_games):
	# Each game is formatted this way : game_id, game_title, number_of_players, max_number_of_players
	for game in list_of_games:
		var a = game_scene.instantiate()
		game_list_node.add_child(a)
		a._setup(game[0], game[1], game[2], game[3])
		a.join.connect(join_game_clicked)

func create_game_clicked():
	create_game_button_pressed.emit()

func create_game_received(game_name: String, max_players: int):
	create_game.emit(game_name, max_players)

func join_game_clicked(id):
	send_join_game.emit(id)
