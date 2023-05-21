extends Node2D

@onready var title = $multiplayer/title
@onready var background = $multiplayer/background
@onready var connect_button = $multiplayer/connect_button
@onready var username_input = $multiplayer/username_input
@onready var global_chat = $multiplayer/global_chat
@onready var global_chat_input = $multiplayer/global_chat_input

@export var dimensions : Vector2 = Vector2(400, 400)

func _ready():
	if Multiplayer:
		connect_button.pressed.connect(_connect)
	
	resize()

func _process(delta):
	if Input.is_action_just_pressed("chat_enter"):
		if !global_chat_input.has_focus():
			global_chat_input.grab_focus()
		else:
			var msg = global_chat_input.text.replace("\n", "")
			Multiplayer._send_global_chat_message(msg)
			global_chat_input.text = ""
			global_chat_input.release_focus()

func _connect():
	if username_input.text != "":
		title.text += " - " + username_input.text
		Multiplayer._connect(username_input.text)
		
		connect_button.visible = false
		username_input.visible = false
		
		global_chat.visible = true
		global_chat_input.visible = true

func resize(d = dimensions):
	dimensions = d
	
	background.size = dimensions
	connect_button.position = dimensions/2 - connect_button.size/2 + Vector2(0, 48)
	username_input.position = dimensions/2 - username_input.size/2 + Vector2(0, -24)
