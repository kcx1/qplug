-- Optional function to define components used within the plugin
function GetComponents(props)
	local components = {}
	table.insert(components, {
		Name = "main_mixer",
		Type = "mixer",
		Properties = {
			["n_inputs"] = 8,
			["n_outputs"] = 1,
		},
	})
	return components
end
