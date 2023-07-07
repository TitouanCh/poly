extends Node

var mouse_position = Vector2.ZERO

func create_unit():
	var script = load("res://unit_class.gd")
	var a = Unit.new()
	a.set_script(script)
	update_mouse_position.connect(a.get_mouse_position)
	return a

func create_unit_from(unit):
	var script = load("res://unit_class.gd")
	var a = unit.clone()
	a.set_script(script)
	update_mouse_position.connect(a.get_mouse_position)
	return a

signal update_mouse_position(mouse_position)

var soldier_compendium = [
	{
		"name": "swordmen",
		"attack": 2,
		"defense": 2,
		"health": 100
	},
]

var unit_compendium = [
	create_unit().setup(
		"Basic Melee",
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
		19,
		9,
	)
]

func _process(delta):
#	print(get_viewport().get_mouse_position())
	update_mouse_position.emit(get_viewport().get_mouse_position())
