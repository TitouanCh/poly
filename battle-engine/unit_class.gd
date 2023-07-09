extends Object

class_name Unit

var name = "Default"
var soldiers = [0, 0, 0, 0]
var positions = []
var center_of_mass: Vector2 = Vector2.ZERO
var target_positions = []
var orders = []
var speed: float = 100
var current_angle = 0
var current_position = Vector2.ZERO
var n: int
var spacing: float
var width: int
var stance: String = "regular"

# DEBUG --
var selected = false

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
	target_positions = positions

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
	var sum = Vector2.ZERO
	var deaccel_epsilon = 50
	for i in range(len(soldiers)):
		var direction = target_positions[i] - positions[i]
		var distance = direction.length()
		direction = direction.normalized()
		var speed_mod = speed
		if distance < deaccel_epsilon:
			speed_mod = max(lerp(0.0, speed, distance/deaccel_epsilon), 1.0)
		positions[i] += direction * speed_mod * delta
		sum += positions[i]
	center_of_mass = sum/len(soldiers)
	
	order_check(delta)

func order_check(delta):
	var order_epsilon = 20
	var sum = 0
	for i in range(len(positions)):
		sum += positions[i].distance_to(target_positions[i])
	
	if sum < order_epsilon:
#		print("NEXT ORDER!!!")
		queue_next_order()

func queue_next_order():
	if len(orders) > 0:
		if orders[0][0] == "r":
			current_angle = orders[0][1]
		if orders[0][0] == "g":
			current_position = orders[0][1]
		
		orders.remove_at(0)
		
		if len(orders) > 0:
			# Go order
			if orders[0][0] == "g":
				target_positions = place_soldiers(orders[0][1], current_angle)
			# Rotate order
			if orders[0][0] == "r":
				target_positions = place_soldiers(current_position, orders[0][1])

# DEBUG --
func preview_at(_position, _angle) -> Unit:
	var preview = clone()
	preview.change_position(_position, _angle)
	return preview

