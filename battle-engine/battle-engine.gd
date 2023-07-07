extends Node2D

var test: Unit

func engine_process(detla, list_of_units):
	pass

func _ready():
#	var test2 = Unit.new()
	test = Constants.create_unit_from(Constants.unit_compendium[0])
	test.change_position(Vector2(200, 200), 0)
	test.target_positions = test.place_soldiers(Vector2(800, 800), 180)
	print(test.target_positions)
	print(test.soldiers)

func _process(delta):
	test.move(delta * 8)
	test.process(delta)
	queue_redraw()

func _draw():
	draw_unit(test)

func draw_unit(unit: Unit):
	for i in range(len(unit.soldiers)):
		draw_circle(unit.positions[i], 2, Color(255, 255, 255))
	draw_circle(unit.center_of_mass, 3, Color(int(!unit.selected) * 255, int(unit.selected) * 255, 0))	
