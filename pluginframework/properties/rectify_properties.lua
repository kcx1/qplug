function RectifyProperties(props)
	if props.plugin_show_debug.Value == false then
		props["Debug Print"].IsHidden = true
	end
	return props
end
