extends Node2D

var units = []
var base_mouse_position = null

func engine_process(detla, list_of_units):
	pass

func _ready():
	create_unit(0, Vector2(200, 200), 0)
	create_unit(0, Vector2(400, 200), 0)

func _process(delta):
	for unit in units:
		unit.move(delta)
	queue_redraw()

func _draw():
	for unit in units:
		draw_unit(unit)
	
	# Inputs --- 
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
					unit.orders.append(["r", (get_viewport().get_mouse_position() - base_mouse_position).angle() + PI/2])
					unit.orders.append(["g", base_mouse_position])
		base_mouse_position = null

func draw_unit(unit: Unit, preview = false):
	for i in range(len(unit.soldiers)):
		draw_circle(unit.positions[i], 2, Color(255 * int(!preview), 255, 255))
	draw_circle(unit.center_of_mass, 3, Color(int(!unit.selected) * 255, int(unit.selected) * 255, 0))	

func create_unit(compendium_idx, _position, _angle):
	var unit = Constants.create_unit_from(Constants.unit_compendium[compendium_idx])
	unit.change_position(_position, _angle)
	units.append(unit)
#	unit.target_positions = unit.place_soldiers(Vector2(800, 800), 180)
	
