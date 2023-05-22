extends Node2D

@onready var title = $multiplayer/title
@onready var background = $background

# Login
@onready var login = $multiplayer/hbox/login
@onready var connect_button = $multiplayer/hbox/login/connect_button
@onready var username_input = $multiplayer/hbox/login/username_input

# Global chat
@onready var global_chat = $multiplayer/hbox/global_chat
@onready var global_chat_title = $multiplayer/hbox/global_chat/title
@onready var global_chat_text = $multiplayer/hbox/global_chat/chat
@onready var global_chat_input = $multiplayer/hbox/global_chat/input

# Game browser
@onready var game_browser = $multiplayer/hbox/browser

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
		elif phase == "browser":
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
		game_browser.visible = true
		
		phase = "browser"

func resize(d = dimensions):
	# Global
	dimensions = d
	background.size = dimensions
	$multiplayer.size = dimensions

func center():
	var screen_size = DisplayServer.window_get_size()
	position = Vector2(screen_size.x, screen_size.y)/2 - dimensions/2

func received_global_message(content : String, user : String):
	global_chat_text.text += "\n[color=pink]" + user + "[/color]: " + content
