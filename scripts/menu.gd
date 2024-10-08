extends Node2D

@onready var title = $multiplayer/title
@onready var background = $background

# Login
@onready var login = $multiplayer/hbox/login
@onready var connect_button = $multiplayer/hbox/login/connect_button
@onready var username_input = $multiplayer/hbox/login/username_input

# Chat
@onready var chat = $multiplayer/hbox/chat

# Global chat
@onready var global_chat = $multiplayer/hbox/chat/global_chat
@onready var global_chat_title = $multiplayer/hbox/chat/global_chat/title
@onready var global_chat_text = $multiplayer/hbox/chat/global_chat/chat
@onready var global_chat_input = $multiplayer/hbox/chat/global_chat/input

# Game chat
@onready var game_chat = $multiplayer/hbox/chat/game_chat
@onready var game_chat_title = $multiplayer/hbox/chat/game_chat/title
@onready var game_chat_text = $multiplayer/hbox/chat/game_chat/chat
@onready var game_chat_input = $multiplayer/hbox/chat/game_chat/input

# Game browser
@onready var game_browser = $multiplayer/hbox/browser

# Lobby
@onready var lobby = $multiplayer/hbox/lobby

@export var dimensions : Vector2 = Vector2(800, 800)

var game_creator_scene = preload("res://scenes/ui/create_game.tscn")
var game_creator = null

var phase = "login"


func _ready():
	if Multiplayer:
		connect_button.pressed.connect(_connect)
		Multiplayer.connect("received_global_chat_message", received_global_message)
		Multiplayer.connect("received_joined_game", open_lobby)
		Multiplayer.connect("received_start_game", close_menu)
	
	if game_browser:
		game_browser.create_game_button_pressed.connect(open_game_creator)
	
	if lobby:
		lobby.exit.connect(close_lobby)
	
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
		
		# Game creator --
		elif phase == "creator":
			game_creator._on_confirm_pressed()
	
	# Chat
	var any_visible = false
	for children in chat.get_children():
		if children.visible:
			any_visible = true
	chat.visible = any_visible
	
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

func open_game_creator():
	if !game_creator and phase == "browser":
		game_creator = game_creator_scene.instantiate()
		add_child(game_creator)
		game_creator.create_game.connect(game_browser.create_game_received)
		game_creator.create_game.connect(func(_a, _b): game_creator.queue_free(); game_creator = null; phase = "browser")
		game_creator.cancel.connect(func(): game_creator.queue_free(); game_creator = null; phase = "browser")
		phase = "creator"

func open_lobby():
	phase = "lobby"
	login.visible = false
	global_chat.visible = true
	game_browser.visible = false
	game_chat.visible = true
	
	lobby.visible = true

func close_lobby():
	phase = "browser"
	
	login.visible = false
	global_chat.visible = true
	game_browser.visible = true
	game_chat.visible = false
	
	lobby.visible = false

func close_menu():
	visible = false
