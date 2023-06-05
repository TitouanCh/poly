extends Control

@export var unit: Unit
@onready var attack_label = $attack/label
@onready var defense_label = $defense/label
@onready var health_label = $health/label

func _process(delta):
	if unit:
		attack_label.text = str(unit.attack)
		defense_label.text = str(unit.defense)
		health_label.text = str(unit.health)
