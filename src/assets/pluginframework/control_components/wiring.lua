-- Optional function to define wiring of components used within the plugin
function GetWiring(props)
	local wiring = {}
	table.insert(wiring, { "Audio Output", "main_mixer Output 1" })
	return wiring
end
