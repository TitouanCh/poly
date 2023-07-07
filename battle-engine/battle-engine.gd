extends Node2D

func placer(unit_position: Vector2, unit_angle: float, unit: Unit) -> Array:
	var soldier_positions = []
	
	for i in range(unit.n):
		soldier_positions.append(Vector2(i % unit.width, floor(i / unit.width)))
	
	return soldier_positions

func engine_process(detla, list_of_units):
	pass

func _ready():
#	var test2 = Unit.new()
	var test = Constants.unit_compendium[0].clone()
	print(test)

func _process(delta):
#	print(Constants.unit_compendium)
	queue_redraw()

#func _draw():
#	draw_circle()
