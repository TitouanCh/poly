extends Node2D

var units = []
var base_mouse_position = null
var combat_range = 300

func engine_process(delta, units):
	for i in range(len(units)):
		# Move unit
		units[i].move(delta)
		# Check if enemies are in range
		units[i].incombat = false
		for j in range(i + 1, len(units)):
			if units[i].team != units[j].team:
				if units[i].center_of_mass.distance_to(units[j].center_of_mass) < combat_range:
#					# Attack
					combat(delta, units[i], units[j])
					units[i].incombat = true
		# If incombat, add time to combat timer
		if units[i].incombat:
			units[i].incombat_timer += delta

func combat(delta, unit1: Unit, unit2: Unit):
	var max_width_unit = unit1
	var min_width_unit = unit2
	if unit2.width - unit2.soldiers_incombat > unit1.width - unit1.soldiers_incombat:
		max_width_unit = unit2
		min_width_unit = unit1
	
	var combatants = max_width_unit.soldiers_incombat
	
	for i in range(len(min_width_unit.PStype)):
		if combatants == max_width_unit.width:
			break
		
		if !min_width_unit.PSincombat[i]:
			for j in range(len(max_width_unit.PStype)):
				if !max_width_unit.PSincombat[j]:
					min_width_unit.PSincombat[i] = true
					max_width_unit.PSincombat[j] = true
					min_width_unit.soldiers_incombat += 1
					max_width_unit.soldiers_incombat += 1
					combatants += 1
					
					var combat_positions = get_combat_positions(min_width_unit.PSposition[i], max_width_unit.PSposition[j])
					min_width_unit.PScombat_position[i] = combat_positions[0]
					max_width_unit.PScombat_position[j] = combat_positions[1]
					break

func get_combat_positions(soldier1_position: Vector2, soldier2_position: Vector2):
	var avg = (soldier1_position + soldier2_position)/2
	return [avg, avg]

func _ready():
#	# PERFORMANCE TEST
#	for i in range(100):
#		create_unit(0, Vector2(1000 * randf(), 1000 * randf()), 0)
	
	create_unit(0, Vector2(800, 200), 0, 1)
	create_unit(0, Vector2(200, 200), 0, 2)

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
		draw_circle(unit.PSposition[i], 2, Color(255 * int(!preview), 255, 255))
	draw_circle(unit.center_of_mass, 3, Color(int(!unit.selected) * 255, int(unit.selected) * 255, 0))	

func create_unit(compendium_idx, _position, _angle, _team):
	var unit = Constants.create_unit_from(Constants.unit_compendium[compendium_idx])
	unit.change_position(_position, _angle)
	unit.team = _team
	units.append(unit)
#	unit.target_positions = unit.place_soldiers(Vector2(800, 800), 180)
	
