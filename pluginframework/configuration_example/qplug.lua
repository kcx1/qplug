-- Example QPlug configuration.
return {
	-- Set to a string if you want an external template. Can be a url or a path.
	external_template = nil,
	-- external_template = "https://github.com/qsys-plugins/BasePlugin",
	-- external_template = "https://github.com/qsys-plugins/ExamplePlugin"
	-- external_template = "https://bitbucket.org/qsc-communities/basicpluginframework/src/main/"

	-- Assign to nil if you want use the builtin build tool.
	-- Optionally, you can create your own. Here's an example using the original from QSC
	build_tool = function()
		local cmd = ".\\plugincompile|PLUGCC.exe . .\\plugin.lua"
		os.execute(cmd)
	end,
}
