extends Object

class_name Unit

# Unit type specific info
var name = "Default"
var stance: String = "regular"
var speed: float = 100
var spacing: float
var width: int
var n: int

# Realtime
var center_of_mass: Vector2 = Vector2.ZERO
var current_angle = 0
var current_position = Vector2.ZERO
var team: int = 0
var incombat = false

# Per soldier info
var PStype = [0, 0, 0, 0]
var PSposition = []
var PStarget_position = []
var PScombat_position = []
var PSincombat = []


var orders = []

# Combat
var incombat_timer = 0
var soldiers_incombat = 0

# DEBUG --
var selected = false

func setup(_name, _PStype, _spacing, _width, _stance = "regular"):
	name = _name
	PStype = _PStype
	spacing = _spacing
	width = _width
	stance = _stance
	n = len(PStype)
	for i in range(n): PSincombat.append(false)
	return self

func clone():
	return Unit.new().setup(name, PStype, spacing, width, stance)

func change_position(_position, _angle):
	PSposition = place_soldiers(_position, _angle)
	PScombat_position = PSposition.duplicate()
	center_of_mass = _position
	PStarget_position = PSposition

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
	for i in range(len(PStype)):
		if !PSincombat[i]:
			var direction = PStarget_position[i] - PSposition[i]
			var distance = direction.length()
			direction = direction.normalized()
			var speed_mod = speed
			if distance < deaccel_epsilon:
				speed_mod = max(lerp(0.0, speed, distance/deaccel_epsilon), 1.0)
			PSposition[i] += direction * speed_mod * delta
			sum += PSposition[i]
		else:
			var direction = PScombat_position[i] - PSposition[i]
			var distance = direction.length()
			direction = direction.normalized()
			var speed_mod = speed
			if distance < deaccel_epsilon:
				speed_mod = max(lerp(0.0, speed, distance/deaccel_epsilon), 1.0)
			PSposition[i] += direction * speed_mod * delta
	center_of_mass = sum/(len(PStype) - soldiers_incombat)
	
	order_check(delta)

func order_check(delta):
	var order_epsilon = 20
	var sum = 0
	for i in range(len(PSposition)):
		if !PSincombat[i]:
			sum += PSposition[i].distance_to(PStarget_position[i])
	
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
				PStarget_position = place_soldiers(orders[0][1], current_angle)
			# Rotate order
			if orders[0][0] == "r":
				PStarget_position = place_soldiers(current_position, orders[0][1])

# DEBUG --
func preview_at(_position, _angle) -> Unit:
	var preview = clone()
	preview.change_position(_position, _angle)
	return preview

