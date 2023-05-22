extends Control

var game_scene = preload("res://scenes/ui/game.tscn")
var game_list = []

@onready var game_list_node = $sbox/game_list

signal send_join_game(game_id)

func _ready():
#	receive_games([
#		[0, "Frezio", 3, 4],
#		[2, "Billy Bob", 1, 2],
#		[3, "Bonkers", 3, 15]
#	])
	pass

func receive_games(list_of_games):
	# Each game is formatted this way : game_id, game_title, number_of_players, max_number_of_players
	for game in list_of_games:
		var a = game_scene.instantiate()
		game_list_node.add_child(a)
		a._setup(game[0], game[1], game[2], game[3])
		a.join.connect(join_game_clicked)

func join_game_clicked(id):
	send_join_game.emit(id)
