extends Prop

class_name Unit

var current_position: Vector2
var current_angle: float
var center_of_mass: Vector2
var team: int
var incombat: bool
var soldiers_alive: int
var soldiers_incombat: int

# Soldiers
var PSposition: Array[Vector2] = []
 
@export var unit_name = "Swordman"
@onready var name_label = $ui_elements/unit_name
@onready var unit_indicators = $ui_elements/indicators
@onready var multimesh = $mesh

func _initialize():
	meshes.append($mesh)
	pass

func _update(delta):
	# Display name of unit & indicators
	if camera:
		name_label.position = camera.unproject_position(self.position) - name_label.size/2 + Vector2(0, 32)
		unit_indicators.position = camera.unproject_position(self.position) + Vector2(32, 0)
	
	# Movement
#	if target != Vector3.ZERO:
#		var direction = target - position
#		position.x += direction.x * delta * speed
#		position.z += direction.z * delta * speed
	
#	if target_rotation != rotation.y:
#		rotation.y = lerp_angle(rotation.y, target_rotation, delta * speed)
	
	# Test movement
#	if is_selected and Input.is_action_just_released("right_click"):
#		target = renderer.get_unprojected_mouse_position()
	
	# Multimesh
#	for i in range(multimesh.multimesh.instance_count):
#		var mesh_position = Transform3D()
#		mesh_position = mesh_position.translated(Vector3(i % man_length, 0, floor(i / man_length)) * spacing - Vector3(spacing * man_length/2, 0, spacing * man_length/2))
#		multimesh.multimesh.set_instance_transform(i, mesh_position)
	for i in range(multimesh.multimesh.instance_count):
		if len(PSposition) > i:
			var mesh_position = Transform3D()
			mesh_position = mesh_position.translated(Vector3(PSposition[i].x, 0, PSposition[i].y))
			print("mesh_position: ", mesh_position)
			multimesh.multimesh.set_instance_transform(i, mesh_position)

func _to_string() -> String:
	return "Unit | {2} | position: {0} | selected: {1}".format([position, is_selected, unit_name])

#func attacked_by(attacker: Unit):
#	lose_health(attacker.attack)
#	multimesh.multimesh.instance_count -= 3
#
#func lose_health(amount):
#	health -= amount
#	if health <= 0:
#		_destroy_self()

func set_param(_current_position: Vector2, _current_angle: float, _center_of_mass: Vector2, _team: int, _incombat: bool, _soldiers_alive: int, _soldiers_incombat: int, _PSposition: Array[Vector2]):
	current_position =_current_position
	current_angle = _current_angle
	center_of_mass = _center_of_mass
	team = _team
	incombat = _incombat
	soldiers_incombat = _soldiers_incombat
	soldiers_alive = _soldiers_alive
	PSposition = _PSposition.duplicate()
	print("Unit: ", self)
	print(PSposition)
	set_coord(current_position)
	rotation.y = current_angle
