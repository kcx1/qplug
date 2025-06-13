-- Example QPlug configuration.
return {
	-- [[ Your info ]] --
	-- Must be a table called "me". Any unimplemented data will just be ignored. 
	-- This data will be used to populate the plugin info.lua table
	me = { name = "Your name", email = "Your email" },

	-- [[ External Template ]] --
	-- You can define your own template. This can either be an absolute path on your computer. Or a git repo hosted on the internet.
	-- NOTE: Address must be https:// if you want to fetch from github. If using a local path, please point it to the parent directory. 
	-- If not defined or set to nil, the builtin template will be used. 

	external_template = nil,
	-- external_template = "/home/me/Documents/some/folder/template/"
	-- external_template = "https://github.com/qsys-plugins/BasePlugin",
	-- external_template = "https://github.com/qsys-plugins/ExamplePlugin"
	-- external_template = "https://bitbucket.org/qsc-communities/basicpluginframework/src/main/"

	-- [[ External Build/Compile Tool ]] --
	-- Assign to nil if you want use the builtin build tool.
	-- Optionally, you can create your own. If so, it must a function that takes no arguments. 
	--  Here's an example using the original from QSC
	build_tool = function()
		local cmd = ".\\plugincompile|PLUGCC.exe . .\\plugin.lua"
		os.execute(cmd)
	end,
}
