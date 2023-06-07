extends Prop

class_name Unit

var target = Vector3.ZERO
var target_rotation = 0
var speed = 20
var spacing = 3
var man_length = 3

# Combat
var attack = 1
var defense = 1
var health = 3
 
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
	if target != Vector3.ZERO:
		var direction = target - position
		position.x += direction.x * delta * speed
		position.z += direction.z * delta * speed
	
	if target_rotation != rotation.y:
		rotation.y = lerp_angle(rotation.y, target_rotation, delta * speed)
	
	# Test movement
#	if is_selected and Input.is_action_just_released("right_click"):
#		target = renderer.get_unprojected_mouse_position()
	
	# Multimesh
	for i in range(multimesh.multimesh.instance_count):
		var mesh_position = Transform3D()
		mesh_position = mesh_position.translated(Vector3(i % man_length, 0, floor(i / man_length)) * spacing - Vector3(spacing * man_length/2, 0, spacing * man_length/2))
		multimesh.multimesh.set_instance_transform(i, mesh_position)

func _to_string() -> String:
	return "Unit | {2} | position: {0} | selected: {1}".format([position, is_selected, unit_name])

func attacked_by(attacker: Unit):
	lose_health(attacker.attack)
	multimesh.multimesh.instance_count -= 3

func lose_health(amount):
	health -= amount
	if health <= 0:
		_destroy_self()
