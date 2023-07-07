extends Object

class_name Unit

var name = "Default"
var soldiers = [0, 0, 0, 0]
var positions = []
var n: int
var spacing: float
var width: int
var stance: String = "regular"

func setup(_name, _soldiers, _spacing, _width, _stance = "regular"):
	name = _name
	soldiers = _soldiers
	spacing = _spacing
	width = _width
	stance = _stance
	n = len(soldiers)
	return self

func clone():
	return Unit.new().setup(name, soldiers, spacing, width, stance)
