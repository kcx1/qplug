function GetControls(props)
	local ctrls = {}
	table.insert(ctrls, {
		Name = "SendButton",
		ControlType = "Button",
		ButtonType = "Momentary",
		Count = 1,
		UserPin = true,
		PinStyle = "Input",
		Icon = "Power",
	})
	return ctrls
end
