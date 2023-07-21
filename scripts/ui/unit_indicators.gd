extends Control

@export var unit: Unit
@onready var attack_label = $attack/label
@onready var defense_label = $defense/label
@onready var health_label = $health/label

func _process(delta):
	if unit:
		attack_label.text = str(1)
		defense_label.text = str(1)
		health_label.text = str(1)
