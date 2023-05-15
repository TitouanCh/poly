extends Prop

class_name Unit

var target = Vector3.ZERO
var speed = 20
 
@export var unit_name = "Swordman"
@onready var name_label = $ui_elements/unit_name

func _initialize():
	meshes.append($mesh)

func _update(delta):
	# Display name of unit
	if camera:
		name_label.position = camera.unproject_position(self.position) - name_label.size/2 + Vector2(0, 32)
	
	# Test movement
	if target != Vector3.ZERO:
		var direction = target - position
		position.x += direction.x * delta * speed
		position.z += direction.z * delta * speed

func _to_string() -> String:
	return "Unit | {2} | position: {0} | selected: {1}".format([position, is_selected, unit_name])
