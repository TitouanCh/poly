extends Node

const HOST: String = "127.0.0.1"
const PORT: int = 3000
const RECONNECT_TIMEOUT: float = 3.0

const Client = preload("res://scripts/client.gd")
var _client = Client.new()

signal received_chat_message(content : String)
signal received_start_game

func _ready() -> void:
	_client.connected.connect(_handle_client_connected)
	_client.disconnected.connect(_handle_client_disconnected)
	_client.error.connect(_handle_client_error)
	_client.data.connect(_handle_client_data)
	add_child(_client)
	_client.connect_to_host(HOST, PORT)

func _connect_after_timeout(timeout: float) -> void:
	await get_tree().create_timer(timeout).timeout
	_client.connect_to_host(HOST, PORT)

func _handle_client_connected() -> void:
	print("Client connected to server.")

func _handle_client_data(data) -> void:
	print("Received data: ", data.get_string_from_utf8())
	
	# Chat message
	if data[0] == 109:
		received_chat_message.emit(data.slice(1).get_string_from_utf8())
	
	


func _handle_client_disconnected() -> void:
	print("Client disconnected from server.")
	#_connect_after_timeout(RECONNECT_TIMEOUT) # Try to reconnect after 3 seconds

func _handle_client_error() -> void:
	print("Client error.")
	#_connect_after_timeout(RECONNECT_TIMEOUT) # Try to reconnect after 3 seconds

func _send_chat_message(message : String) -> void:
	_client.send(("msg" + message).to_utf8_buffer())

func _send_start_game() -> void:
	_client.send("sta".to_utf8_buffer())
