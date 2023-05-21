extends Node

const HOST: String = "127.0.0.1"
const PORT: int = 3000
const RECONNECT_TIMEOUT: float = 3.0

const Client = preload("res://scripts/client.gd")
var _client = Client.new()

signal received_chat_message(content : String)
signal received_start_game
signal received_city(city_data : Array)

func _ready() -> void:
	_client.connected.connect(_handle_client_connected)
	_client.disconnected.connect(_handle_client_disconnected)
	_client.error.connect(_handle_client_error)
	_client.data.connect(_handle_client_data)
	add_child(_client)

func _connect(username):
	_client.connect_to_host(HOST, PORT)
	await get_tree().create_timer(1.0).timeout
	_send_username(username)

func _connect_after_timeout(timeout: float) -> void:
	await get_tree().create_timer(timeout).timeout
	_client.connect_to_host(HOST, PORT)

func _handle_client_connected() -> void:
	print("Client connected to server.")

func _handle_client_data(data) -> void:
	var data_str = data.get_string_from_utf8()
	print("Received data: ", data_str, " or ", data)
	
	var data_arr = data_str.split("|end|")
	
	for i in range(len(data_arr) - 1):
		# Chat message
		if data_arr[i][0] == "m":
			received_chat_message.emit(data_arr[i].lstrip("m"))
		
		# Start game message
		if data_arr[i][0] == "s":
			received_start_game.emit()
		
		# City placement
		if data_arr[i][0] == "c":
			received_city.emit(data_arr[i].lstrip("c, ").split(", "))

func _handle_client_disconnected() -> void:
	print("Client disconnected from server.")
	#_connect_after_timeout(RECONNECT_TIMEOUT) # Try to reconnect after 3 seconds

func _handle_client_error() -> void:
	print("Client error.")
	#_connect_after_timeout(RECONNECT_TIMEOUT) # Try to reconnect after 3 seconds

func _send_username(username: String):
	_client.send(username.to_utf8_buffer())

func _send_global_chat_message(message : String) -> void:
	_client.send(("glo" + message).to_utf8_buffer())

func _send_chat_message(message : String) -> void:
	_client.send(("msg" + message).to_utf8_buffer())

func _send_start_game() -> void:
	_client.send("sta".to_utf8_buffer())

func _send_placed_city(coord : Vector2i):
	var message = PackedByteArray("plc".to_utf8_buffer())
	message += PackedByteArray([0, 0, 0, 0, 0, 0, 0, 0])
	message.encode_u32(3, coord.x)
	message.encode_u32(7, coord.y)

	_client.send(message)
