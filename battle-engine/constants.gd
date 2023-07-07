extends Node

func create_unit():
	var script = load("res://unit_class.gd")
	var a = Unit.new()
	a.set_script(script)
	return a

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
