extends Node2D

@onready var title = $multiplayer/title
@onready var background = $multiplayer/background

# Login
@onready var login = $multiplayer/login
@onready var connect_button = $multiplayer/login/connect_button
@onready var username_input = $multiplayer/login/username_input

# Global chat
@onready var global_chat = $multiplayer/global_chat
@onready var global_chat_title = $multiplayer/global_chat/title
@onready var global_chat_text = $multiplayer/global_chat/chat
@onready var global_chat_input = $multiplayer/global_chat/input

@export var dimensions : Vector2 = Vector2(800, 800)

var phase = "login"

func _ready():
	if Multiplayer:
		connect_button.pressed.connect(_connect)
		Multiplayer.connect("received_global_chat_message", received_global_message)
	
	resize()
	center()

func _process(delta):
	if Input.is_action_just_pressed("chat_enter"):
		# Login screen --
		if phase == "login":
			if !username_input.has_focus():
				username_input.grab_focus()
			else:
				_connect()
		
		# Global chat --
		if phase == "browser":
			if !global_chat_input.has_focus():
				global_chat_input.grab_focus()
			elif global_chat_input.text != "":
				var msg = global_chat_input.text.replace("\n", "")
				Multiplayer._send_global_chat_message(msg)
				global_chat_input.text = ""
				global_chat_input.release_focus()
	
	center()

func _connect():
	if username_input.text != "":
		title.text += " - " + username_input.text
		Multiplayer._connect(username_input.text.replace("\n", ""))
		
		login.visible = false
		global_chat.visible = true
		
		phase = "browser"

func resize(d = dimensions):
	# Global
	dimensions = d
	background.size = dimensions
	
	# Login
	connect_button.position = dimensions/2 - connect_button.size/2 + Vector2(0, 48)
	username_input.position = dimensions/2 - username_input.size/2 + Vector2(0, -24)
	
	# Chat
	global_chat_text.size.y = dimensions.y - 100 - 60
	global_chat_text.size.x = dimensions.x * 0.333
	global_chat_text.position.x = 5
	global_chat_text.position.y = 50
	
	global_chat_input.position.x = 5
	global_chat_input.position.y = global_chat_text.size.y + 10 + 50
	
	global_chat_title.position.x = 5
	global_chat_title.position.y = 10
	
	global_chat_input.size.x = dimensions.x * 0.333 - 10
	global_chat_input.position.x = 5
	
	global_chat.position.y = 50
	global_chat.position.x = 0.667 * dimensions.x

func center():
	var screen_size = DisplayServer.window_get_size()
	position = Vector2(screen_size.x, screen_size.y)/2 - dimensions/2

func received_global_message(content : String, user : String):
	global_chat_text.add_text("\n" + user + ": " + content)
