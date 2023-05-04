extends RichTextLabel

@onready var chat_edit = $chat_edit

func _ready():
	if Multiplayer:
		Multiplayer.connect("received_chat_message", received_message)

func _process(delta):
	if Input.is_action_just_pressed("chat_enter"):
		if !chat_edit.has_focus():
			chat_edit.grab_focus()
		else:
			var msg = chat_edit.text.replace("\n", "")
			Multiplayer._send_chat_message(msg)
			chat_edit.text = ""
			chat_edit.release_focus()

func received_message(content):
	add_text("\n" + content)
