-- Example QPlug configuration.
return {
	-- Set to a string if you want an external template. Can be a url or a path.
	external_template = nil,

	-- Which build tool to use. This can be a string or a function.
	build_tool = function()
		local cmd = ".\\plugincompile|PLUGCC.exe . .\\plugin.lua"
		os.execute(cmd)
	end,
}
