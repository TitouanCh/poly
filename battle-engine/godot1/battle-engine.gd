extends Node2D

var units = []
var base_mouse_position = null
var combat_range = 400

func engine_process(delta, units):
	for i in range(len(units)):
		# Move unit
		units[i].move(delta)
		units[i].process(delta, units)
		# Check if enemies are in range
#		units[i].incombat = false
		for j in range(i + 1, len(units)):
			if units[i].team != units[j].team:
				if units[i].center_of_mass.distance_to(units[j].center_of_mass) < combat_range:
#					# Check for duels
					duel(delta, units[i], units[j])
#					units[i].incombat = true
				if units[i].incombat and units[j].incombat:
					damage(delta, units[i], units[j])

func damage(delta, unit1: Unit, unit2: Unit):
	# Calculate damage taken to soldiers in duel
	for i in range(unit1.n):
		# Check if has a duel opponent, is in combat and alive
		if unit1.PSincombat[i] and unit1.PSopponent[i] and unit1.PSalive[i]:
			# Check if duel opponent is in unit2
			# PSopponent format [opponent_unit_idx, opponent_soldier_idx] --
			if unit1.PSopponent[i][0] == unit2.idx:
#				print("pow")
				unit1.PStake_damage(i, delta, unit2.PSattack[unit1.PSopponent[i][1]])
	
	# Do the same for the other unit
	for j in range(unit2.n):
		if unit2.PSincombat[j] and unit2.PSopponent[j] and unit2.PSalive[j]:
			if unit2.PSopponent[j][0] == unit1.idx:
#				print("pow")
				unit2.PStake_damage(j, delta, unit1.PSattack[unit2.PSopponent[j][1]])
	
	# Check for death
	for i in range(unit1.n):
		# If the soldier has an opponent
		if unit1.PSopponent[i]:
			if unit1.PSopponent[i][0] == unit2.idx:
				# If the soldier died or opponent died end combat
				if !unit1.PSalive[i] or !unit2.PSalive[unit1.PSopponent[i][1]]:
					unit1.PSincombat[i] = false
					unit1.PSopponent[i] = null
					unit1.soldiers_incombat -= 1
		
	# Do the same for other unit
	for j in range(unit2.n):
		# If the soldier has an opponent
		if unit2.PSopponent[j]:
			if unit2.PSopponent[j][0] == unit1.idx:
				# If the soldier died
				if !unit2.PSalive[j] or !unit1.PSalive[unit2.PSopponent[j][1]]:
					unit2.PSincombat[j] = false
					unit2.PSopponent[j] = null
					unit2.soldiers_incombat -= 1

func duel(delta, unit1: Unit, unit2: Unit):
	# Sort unit by width (corresponds to combat capacity)
	var max_width_unit = unit1
	var min_width_unit = unit2
	if unit2.get_actual_width() > unit1.get_actual_width():
		max_width_unit = unit2
		min_width_unit = unit1
	
	# A unit cannot have more active combatants than width
	var combatants = max_width_unit.soldiers_incombat
	
	# Setup duels
	for i in min_width_unit.sort_soldiers_by_distance_to_point(max_width_unit.center_of_mass):
		if combatants == max_width_unit.width:
			break
		
		# Check if current surveyed soldier is dead
		if !min_width_unit.PSalive[i]:
			continue
		
		# If not, find another soldier to fight with him
		if !min_width_unit.PSincombat[i]:
			for j in max_width_unit.sort_soldiers_by_distance_to_point(min_width_unit.PSposition[i]):
				# Check if oponent is dead
				if !max_width_unit.PSalive[j]:
					continue
				
				# If not and opponent is not in combat, set both soldiers in a duel
				if !max_width_unit.PSincombat[j]:
					min_width_unit.PSincombat[i] = true
					max_width_unit.PSincombat[j] = true
					min_width_unit.soldiers_incombat += 1
					max_width_unit.soldiers_incombat += 1
					min_width_unit.PSopponent[i] = [max_width_unit.idx, j]
					max_width_unit.PSopponent[j] = [min_width_unit.idx, i]
				
					combatants += 1
					# Calculate and set combat positions
					var combat_positions = get_combat_positions(min_width_unit.PSposition[i], max_width_unit.PSposition[j])
					min_width_unit.PScombat_position[i] = combat_positions[0]
					max_width_unit.PScombat_position[j] = combat_positions[1]
					break
		
				# If not, make them fight and take damage

func get_combat_positions(soldier1_position: Vector2, soldier2_position: Vector2):
	var avg = (soldier1_position + soldier2_position)/2
	return [avg, avg]

func _ready():
	# PERFORMANCE TEST
#	for i in range(100):
#		create_unit(0, Vector2(1000 * randf(), 1000 * randf()), 0, randi())
	
	create_unit(0, Vector2(800, 200), 0, 1)
	create_unit(0, Vector2(800, 800), 0, 2)
	create_unit(0, Vector2(200, 200), 0, 3)

func _process(delta):
	engine_process(delta, units)
	
	# Debug ? Inputs --- 
	var mouse_position = get_viewport().get_mouse_position()
	if Input.is_action_just_released("left_click"):
		var action_performed = false
		# Check for select action
		for unit in units:
			if mouse_position.distance_to(unit.center_of_mass) < 12:
				unit.selected = !unit.selected
				# Deselect other units
				if unit.selected: for _unit in units: if _unit != unit: _unit.selected = false
				action_performed = true
		# Check for movement action
		if !action_performed:
			for unit in units:
				if unit.selected:
					unit.orders.append(["r", (mouse_position - base_mouse_position).angle() + PI/2])
					unit.orders.append(["g", base_mouse_position])
		base_mouse_position = null
	
	if Input.is_action_just_pressed("print"):
		for unit in units:
			if unit.selected:
				unit.print_info()
	
	queue_redraw()

func _draw():
	for unit in units:
		draw_unit(unit)
	
	# Debug ? Unit indicators
	var mouse_position = get_viewport().get_mouse_position()
	if Input.is_action_pressed("left_click"):
		if base_mouse_position == null:
			base_mouse_position = mouse_position
		# Direction line
		draw_line(base_mouse_position, mouse_position, Color(255, 255, 255), 2)
		var angle = mouse_position - base_mouse_position
		if angle != Vector2.ZERO:
			angle = angle.angle()
		else:
			angle = 0
		# Draw a preview of where the unit will end up
		for unit in units:
			if unit.selected:
				draw_unit(unit.preview_at(base_mouse_position, angle + PI/2), true)

func draw_unit(unit: Unit, preview = false):
	for i in range(len(unit.PStype)):
		if unit.PShealth[i] > 0:
			draw_circle(unit.PSposition[i], 8, Color(255 * int(!preview), 255, 255))
		if unit.PShealth[i] < 100:
			draw_line(unit.PSposition[i] + Vector2(-5, -12), unit.PSposition[i] + Vector2(unit.PShealth[i] * 0.2 - 5, -12), Color(0, 255, 0), 1)
	draw_circle(unit.center_of_mass, 3, Color(int(!unit.selected) * 255, int(unit.selected) * 255, 0))	

func create_unit(compendium_idx, _position, _angle, _team):
	var unit = Constants.create_unit_from(Constants.unit_compendium[compendium_idx])
	unit.change_position(_position, _angle)
	unit.idx = len(units)
	unit.team = _team
	units.append(unit)
#	unit.target_positions = unit.place_soldiers(Vector2(800, 800), 180)
	
