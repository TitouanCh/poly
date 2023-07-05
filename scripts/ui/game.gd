extends Panel

@onready var title_node = $hbox/title
@onready var description_node = $hbox/description
@onready var join_button = $hbox/join

var id = null
var title = ""

signal join(n)

func _ready():
	join_button.pressed.connect(join_game_clicked)

func _setup(n: int, t: String, number_of_players: int, max_number_of_players: int, phase: int):
	id = n
	title = t
	
	title_node.text = title
	description_node.text = str(number_of_players) + "/" + str(max_number_of_players) + " players"
	join_button.disabled = phase != 0

func join_game_clicked():
	join.emit(id)

func _setup_as_player(ready: bool, connected: bool, user_in_game_id: int, spectator: bool, username: String):
	title_node.text = username
	
	join_button.disabled = true
	join_button.text = "spectator" if spectator else "ready" if ready else "not ready"
	
	description_node.text = "[color=green](id: " + str(user_in_game_id) + ")[/color]" if connected else "[color=grey](id: " + str(user_in_game_id) + ")[/color]"
