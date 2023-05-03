extends Node

@export var SOCKET_URL = "ws://127.0.0.1:3000" 

var _socket = WebSocketPeer.new()

func _ready():
	var error = _socket.connect_to_url(SOCKET_URL)
	print("Connected: ", _socket.get_connected_host(), ", Error: ", error)

func _process(delta):
	_socket.poll()
	var state = _socket.get_ready_state()
	if state == WebSocketPeer.STATE_OPEN:
		while _socket.get_available_packet_count() > 0:
			print("Packet: ", _socket.get_packet().get_string_from_utf8())
	elif state == WebSocketPeer.STATE_CLOSING:
			# Keep polling to achieve proper close.
			pass
	elif state == WebSocketPeer.STATE_CLOSED:
		var code = _socket.get_close_code()
		var reason = _socket.get_close_reason()
		print("Disconnected: ", code, " | ", reason)
		set_process(false)
