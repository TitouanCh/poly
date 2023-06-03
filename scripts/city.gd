extends Prop

class_name City

@export var city_name = "Orsay"
@export var city_id = 0

@onready var name_label = $ui_elements/city_name
@onready var houses = $houses
@onready var noise = FastNoiseLite.new()

var create_animation_timer = -1

func _initialize():
	for child in $fort.get_children():
		if child is MeshInstance3D:
			meshes.append(child)
	setup_houses()

func _update(delta):
	if camera:
		# Display name of city
		name_label.position = camera.unproject_position(self.position) - name_label.size/2 + Vector2(0, 32)
		
		if create_animation_timer >= 0:
			create_animation(delta)

func _to_string() -> String:
	return "City | {2} | position: {0} | selected: {1}".format([position, is_selected, city_name])

func set_param(id, n, coord, just_created):
	city_id = id
	set_city_name(n)
	set_coord(coord)
	
	if just_created:
		# Play a create animation
		create_animation_timer = 0
		on_floor = false
		houses.visible = false
		$flag.visible = false
		self.rotate_y(randf() * 2 * PI)

func set_city_name(n):
	if n != city_name:
		city_name = n
		name_label.text = city_name

func setup_houses():
	randomize()
	noise.seed = randi()
	noise.frequency = 1.0
	houses.process_material.set_shader_parameter("noisemap", ImageTexture.create_from_image(noise.get_image(512, 512)))
	houses.emitting = true

func set_active():
	visible = true
	name_label.visible = true

func set_unactive():
	visible = false
	name_label.visible = false

func create_animation(delta):
	var DROP_HEIGHT = 40
	self.position.y = get_height_at_position() + 40 * (1 - ((pow(create_animation_timer - 0.1, 2)-0.1)/0.7))
	$fort.scale = Vector3.ONE * lerp(0, 1, create_animation_timer)
	rotate_y((1/(create_animation_timer + 0.1))/20)
	
	create_animation_timer += delta
	if create_animation_timer >= 1:
		on_floor = true
		create_animation_timer = -1
		$fort.scale = Vector3.ONE
		houses.visible = true
		$flag.visible = true
