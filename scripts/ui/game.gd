extends Panel

@onready var title_node = $hbox/title
@onready var description_node = $hbox/description
@onready var join_button = $hbox/join

var id = null
var title = ""

signal join(n)

func _ready():
	join_button.pressed.connect(join_game_clicked)

func _setup(n: int, t: String, number_of_players: int, max_number_of_players: int):
	id = n
	title = t
	
	title_node.text = title
	description_node.text = str(number_of_players) + "/" + str(max_number_of_players) + " players"

func join_game_clicked():
	join.emit(id)
