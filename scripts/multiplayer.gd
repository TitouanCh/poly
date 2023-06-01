extends Node

const HOST: String = "127.0.0.1"
const PORT: int = 3000
const RECONNECT_TIMEOUT: float = 1.0

var USERNAME: String = ""

const Client = preload("res://scripts/client.gd")
var _client = Client.new()

signal received_chat_message(content: String)
signal received_global_chat_message(content: String, username: String)
signal received_start_game
signal received_city(city_data: Array)
signal received_joined_game
signal received_lobby_state(lobby_state: Array)
signal received_browser_state(browser_state: Array)

func _ready() -> void:
	_client.connected.connect(_handle_client_connected)
	_client.disconnected.connect(_handle_client_disconnected)
	_client.error.connect(_handle_client_error)
	_client.data.connect(_handle_client_data)
	add_child(_client)

func _connect(username):
	_client.connect_to_host(HOST, PORT)
	await get_tree().create_timer(RECONNECT_TIMEOUT).timeout
	_send_username(username)
	USERNAME = username

func _connect_after_timeout(timeout: float) -> void:
	await get_tree().create_timer(timeout).timeout
	_client.connect_to_host(HOST, PORT)

func _handle_client_connected() -> void:
	print("Client connected to server.")

func _handle_client_data(data) -> void:
	var messages = decode_bytes(data)
	print(messages)
	
	for message in messages:
		# Global chat message
		if message["content"][0] == "g":
			received_global_chat_message.emit(message["content"][1].get_string_from_utf8(), message["user"][1])
		
		# Join game
		if message["content"][0] == "j":
			print("Joining game")
			received_joined_game.emit()
		
		# Lobby state
		if message["content"][0] == "l":
			received_lobby_state.emit(message["content"][1], message["user"][1])
		
		# Game handler info
		if message["content"][0] == "i":
			received_browser_state.emit(message["content"][1])
		
#	var data_str = data.get_string_from_utf8()
#	print("Received data: ", data_str, " or ", data)
#
#	var data_arr = data_str.split("|end|")
#	var ignore_next = false
#
#	for i in range(len(data_arr) - 1):
#		if ignore_next:
#			ignore_next = false
#			continue
#
#		# Chat message
#		if data_arr[i][0] == "m":
#			received_chat_message.emit(data_arr[i].trim_prefix("m"))
#
#		# Global chat message
#		if data_arr[i][0] == "g":
#			received_global_chat_message.emit(data_arr[i].trim_prefix("g"), data_arr[i + 1])
#			# Skip next message which is username
#			ignore_next = true
#
#		# Start game message
#		if data_arr[i][0] == "s":
#			received_start_game.emit()
#
#		# City placement
#		if data_arr[i][0] == "c":
#			received_city.emit(data_arr[i].trim_prefix("c, ").split(", "))
#
#		# Game handler info
#		if data_arr[i][0] == "i":
#			pass
#
#		# Game state
#		if data_arr[i][0] == "a":
#			pass

# Decodes incoming byte stream, returns an array of message
func decode_bytes(bytes : Array) -> Array:
	var messages_bytes = _seperate_byte_array(bytes, [124, 101, 110, 100, 124])
	var messages_decoded = []
	
	for message_bytes in messages_bytes:
		var fragmented = _seperate_byte_array(message_bytes, [124, 117, 115, 101, 114, 124])
		
		if len(fragmented) != 2:
			print("Error decoding message: " + str(message_bytes))
			continue
		
		# ---------- Message Structure : {user, content} --------
		# --- with :
		# user : [u32, string]
		# content : [string (first three bytes), rest of the bytes]
		# -------------------------------------------------------
		
		var message = {
			"user" : [
				# id
				fragmented[0].decode_u32(0),
				# name
				fragmented[0].slice(4, len(fragmented[0])).get_string_from_utf8()
			],
			"content" : [
				# order
				fragmented[1].slice(0, 1).get_string_from_utf8(),
				# bytes
				fragmented[1].slice(1, len(fragmented[1]))
			]
		}
		
		messages_decoded.append(message)
		
	return messages_decoded

# Seperates a bytes array by an array of bytes
func _seperate_byte_array(array : Array, separator : Array) -> Array:
	separator.append("ok")
	var last_idx = 0
	var result = []
	for i in range(len(array) - len(separator) + 2):
		for j in range(len(separator)):
			if separator[j] is String:
				result.append(PackedByteArray(array.slice(last_idx, i)))
				last_idx = i + j
				break
			if !(separator[j] == array[i + j]):
				break
	
	if last_idx < len(array):
		result.append(PackedByteArray(array.slice(last_idx, len(array))))
	
	return result

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

func _send_create_game(game_name: String, max_players: int):
	# Format: ghi + 24 bytes for the game name + 1 byte for the max player number
	var message = PackedByteArray("ghc".to_utf8_buffer())
	while len(game_name) < 24:
		game_name += " "
	message += game_name.to_utf8_buffer()
	message.append(max_players)
	
	_client.send(message)

func _send_join_game(game_id: int):
	var message = PackedByteArray("ghj".to_utf8_buffer())
	message += PackedByteArray([0, 0, 0, 0])
	message.encode_u32(3, game_id)
	
	_client.send(message)

func _send_ready():
	var message = PackedByteArray("rea".to_utf8_buffer())
	_client.send(message)

func _send_launch():
	var message = PackedByteArray("lau".to_utf8_buffer())
	_client.send(message)
