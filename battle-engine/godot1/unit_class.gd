extends Object

class_name Unit

# Global param
var LERP_MOVEMENT = false

# Unit type specific info
var name: String = "Default"
var idx: int = 0
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
var incombat = true
var alive = 0

# Per soldier info
var PStype = []
var PSposition = []
var PSmass = []
var PSspeed = []
var PStarget_position = []
var PScombat_position = []
var PSincombat = []
var PSopponent = []
var PShealth = []
var PSattack = []
var PSdefense = []
var PSalive = []

var orders = []

# Combat
var incombat_timer = 0
var soldiers_incombat = 0

# DEBUG --
var selected = false

func get_actual_width() -> int:
	return width - soldiers_incombat + floor(incombat_timer)

func sort_soldiers_by_distance_to_point(point: Vector2) -> Array:
	var min
	var min_i
	var sorted = []
	while len(sorted) < n:
		min = -1
		min_i = 0
		for i in range(n):
			if (PSposition[i].distance_squared_to(point) < min or min == -1) and !sorted.has(i):
				min = PSposition[i].distance_squared_to(point)
				min_i = i
		sorted.append(min_i)
#	print(sorted)
	return sorted

func PStake_damage(soldier_idx, delta, opponent_attack):
	PShealth[soldier_idx] -= opponent_attack * delta * (1.0 + 0.5 * randf()) * 10
	if PShealth[soldier_idx] <= 0:
		PShealth[soldier_idx] = 0
		if PSalive[soldier_idx]:
			PSalive[soldier_idx] = false
			alive -= 1

func setup(_name, _idx, _PStype, _spacing, _width, _soldier_compendium, _stance = "regular"):
	name = _name
	idx = _idx
	PStype = _PStype
	spacing = _spacing
	width = _width
	stance = _stance
	n = len(PStype)
	alive = n
	for i in range(n):
		PSincombat.append(false)
		PSopponent.append(null)
		PSalive.append(true)
		PStarget_position.append(Vector2.ZERO)
		PSspeed.append(Vector2.ZERO)
		PSmass.append(8)
		PShealth.append(_soldier_compendium[PStype[i]]["health"])
		PSdefense.append(_soldier_compendium[PStype[i]]["defense"])
		PSattack.append(_soldier_compendium[PStype[i]]["attack"])
	return self

func clone():
	return Unit.new().setup(name, idx, PStype, spacing, width, Constants.soldier_compendium, stance)

func change_position(_position, _angle):
	PSposition = place_soldiers(_position, _angle)
	PScombat_position = PSposition.duplicate()
	center_of_mass = _position
	PStarget_position = PSposition
	current_position = _position
	current_angle = _angle

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

func process(delta, other_units):
	incombat = false
	for _incombat in PSincombat:
		if _incombat:
			incombat = true
			break
	order_check(delta)
	
	# Physics -- Simple ---
	for unit in other_units: 
		if unit.center_of_mass.distance_to(center_of_mass) < 1000:
			for i in range(n):
				for j in unit.n:
					while PSposition[i].distance_to(unit.PSposition[j]) < PSmass[i] + unit.PSmass[j] and unit.PSposition[j] != PSposition[i]:
						# Collision
#						print("Collision!!")
						var collision_axe = (unit.PSposition[j] - PSposition[i]).normalized()
						PSposition[i] -= collision_axe * unit.PSmass[j] * 0.1
						unit.PSposition[j] += collision_axe * PSmass[i] * 0.1

func move(delta):
	var sum = Vector2.ZERO
	var deaccel_epsilon = 50
	for i in range(len(PStype)):
		if !PSincombat[i] and PSalive[i]:
			var direction = PStarget_position[i] - PSposition[i]
			var distance = direction.length()
			direction = direction.normalized()
			var speed_mod = speed
			if distance < deaccel_epsilon:
				if LERP_MOVEMENT:
					speed_mod = max(lerp(0.0, speed, distance/deaccel_epsilon), 1.0)
				else: speed_mod = (distance/deaccel_epsilon) * speed
			if incombat:
				speed_mod *= 0.1
			PSspeed[i] = direction * speed_mod * delta - PSposition[i]
			PSposition[i] += direction * speed_mod * delta
			sum += PSposition[i]
		else:
			var direction = PScombat_position[i] - PSposition[i]
			var distance = direction.length()
			direction = direction.normalized()
			var speed_mod = speed
			if distance < deaccel_epsilon:
				if LERP_MOVEMENT:
					speed_mod = max(lerp(0.0, speed, distance/deaccel_epsilon), 1.0)
				else: speed_mod = (distance/deaccel_epsilon) * speed
			PSspeed[i] = direction * speed_mod * delta - PSposition[i]
			PSposition[i] += direction * speed_mod * delta
	center_of_mass = sum/(alive - soldiers_incombat)

func order_check(delta):
	var order_epsilon = 20
	var sum = 0

	for i in range(len(PSposition)):
		if !PSincombat[i] and PSalive[i]:
			sum += PSposition[i].distance_to(PStarget_position[i])
	
	if sum < order_epsilon:
		queue_next_order()

func queue_next_order():
	if len(orders) > 0:
#		print("NEXT ORDER!!!")
		if orders[0][0] == "r":
			current_angle = orders[0][1]
		if orders[0][0] == "g":
			current_position = orders[0][1]
#		if orders[0][0] == "e":
#			current_position
		
		orders.remove_at(0)
		
		if len(orders) > 0:
			# Go order
			if orders[0][0] == "g":
				PStarget_position = place_soldiers(orders[0][1], current_angle)
			# Rotate order
			if orders[0][0] == "r":
				PStarget_position = place_soldiers(current_position, orders[0][1])
	if len(orders) == 0:
		# Add an empty order
		orders.append(["e"])

# DEBUG --
func preview_at(_position, _angle) -> Unit:
	var preview = clone()
	preview.change_position(_position, _angle)
	return preview

func print_info():
	print(name, " ", idx, " ---")
	print("h: ", PShealth)
	print("a: ", PSalive)
	print("c: ", PSincombat)
	print("t: ", PStarget_position)
	print("soldiers in combat: ", soldiers_incombat)
	print("alive: ", alive)
	print("orders: ", orders)
	print("---")
