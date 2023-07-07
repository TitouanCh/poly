extends Object

class_name Unit

var name = "Default"
var soldiers = [0, 0, 0, 0]
var position: Vector2 = Vector2.ZERO
var positions = []
var n: int
var spacing: float
var width: int
var stance: String = "regular"

func setup(_name, _soldiers, _spacing, _width, _stance = "regular"):
	name = _name
	soldiers = _soldiers
	spacing = _spacing
	width = _width
	stance = _stance
	n = len(soldiers)
	return self

func clone():
	return Unit.new().setup(name, soldiers, spacing, width, stance)

func change_position(_position, _angle):
	position = _position
	positions = place_soldiers(_angle)

func place_soldiers(unit_angle: float) -> Array:
	var soldier_positions = []
	print((width - 1)/2)
	for i in range(n):
		soldier_positions.append((Vector2(i % width, floor(i / width)) 
		- Vector2((float(width) - 1)/2, (ceil(float(n)/float(width)) - 1)/2)) * spacing)
	
	return soldier_positions
