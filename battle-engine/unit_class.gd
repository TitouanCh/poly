extends Object

class_name Unit

var name = "Default"
var soldiers = [0, 0, 0, 0]
var positions = []
var target_positions = []
var speed: float = 8
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
	positions = place_soldiers(_position, _angle)

func place_soldiers(_position: Vector2, unit_angle: float = 0.0) -> Array:
	var soldier_positions = []

	for i in range(n):
		# Offset
		soldier_positions.append((Vector2(i % width, floor(i / width)) 
		- Vector2((float(width) - 1)/2, (ceil(float(n)/float(width)) - 1)/2)) * spacing)
		
		# Rotation
		soldier_positions[i] = soldier_positions[i].rotated(unit_angle)
		
		# Position
		soldier_positions[i] += _position
	
	return soldier_positions

func move(delta):
	for i in range(len(soldiers)):
		var direction = (target_positions[i] - positions[i]).normalized()
		positions[i] += direction * speed * delta
