extends Panel

@onready var game_name_input = $vbox/game_name/input
@onready var number_of_players_input = $vbox/number_of_players/input

signal create_game(game_name: String, number_of_players: int)
signal cancel

func _on_confirm_pressed():
	var number_of_players = number_of_players_input.value
	var game_name = game_name_input.text
	
	if number_of_players <= 16 and number_of_players > 1:
		if len(game_name) < 24:
			create_game.emit(game_name, number_of_players)

func _on_cancel_pressed():
	cancel.emit()
