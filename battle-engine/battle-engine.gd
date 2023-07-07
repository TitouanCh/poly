extends Node2D

var test: Unit

func engine_process(detla, list_of_units):
	pass

func _ready():
#	var test2 = Unit.new()
	test = Constants.unit_compendium[0].clone()
	test.change_position(Vector2(200, 200), 0)
	
	print(test.soldiers)

func _process(delta):
#	print(Constants.unit_compendium)
	queue_redraw()

func _draw():
	draw_unit(test)

func draw_unit(unit: Unit):
	for i in range(len(unit.soldiers)):
		draw_circle(unit.positions[i] + unit.position, 4, Color(255, 255, 255))
	draw_circle(unit.position, 3, Color(255, 0, 0))	
