# Read Me

This plugin framework was adopted from the [BasicPluginFramework](https://bitbucket.org/qsc-communities/basicpluginframework/src/main/) by QSC.

The BasicPluginframework was built as a template plugin that you can download with some pre-written Lua code to get you started.

### Motivation for Changes

The original used ```--[[ #include "file/path" ]]``` in order to find and replace the contents of the plugin file with some arbitrary lua code. 

I wanted to use something more natural in Lua. So instead - you simply ```require("mod.name")``` and then that will be replaced by the content that module. Admittedly, this is slightly anti-pattern since the modules are not formatted as they would be in a typical lua module. (although they could be.)

Beyond making this more idiomatic in Lua it has the added benefit of the LSP recognizing code that is being required while working on the files. 

# QplugFramework

Here is the framework structure:

```
.
├── LICENSE.txt
├── README.md
├── control_components
│   ├── components.lua
│   ├── controls.lua
│   └── wiring.lua
├── init.lua
├── layout
│   └── layout.lua
├── properties
│   ├── pins.lua
│   ├── properties.lua
│   └── rectify_properties.lua
├── runtime
│   └── runtime.lua
└── setup
    ├── colors.lua
    ├── info.lua
    ├── model.lua
    ├── pages.lua
    └── pretty_name.lua
```

These are the source files and the resulting QPLUG file will be created in the parent directory to this framework.

The init.lua file is where everything gets glued together. 

From there each section is separated into categories. 

- Setup
    - This is where the setup of the plugin itself is done. None of these are user accessible. 
- Properties
    - This is where you define the properties of the plugin that will appear in the properties section of QSD. 
- Control components
    - This is where you define the components and the controls of the plugin.
- Layout
    - Define how the plugin looks in the UI.
- Runtime
    - Code that is run at runtime. This is the logic that the plugin does.


You are free to change this to suit your needs.


### Limitations

For now - the init.lua file is the only file that will allow content injection on the require statements. Do not require files that you expect to be available in the Q-Sys runtime environment inside of the init.lua. Instead - require them inside the sub module that is using it. Or create a new submodule that has it's own namespace and can be swapped into the init.lua file. 

This will not parse init.lua files within directories. You must require the full path and cannot use the init.lua as a proxy.

### Roadmap

- [] Allow user defined templates
- [] Make framework a little more idiomatic for lua and require statements.
- [] Allow loading modules from outside of the init.lua file
- [] Allow directories containing init.lua files to parse the additional modules required by the init.lua file

## Licensing Information

This subdirectory contains code derived from the Basic Framework Plugin by QSC.

- **Original Work**: Copyright (c) QSC, LLC, 2021
- **Modifications and New Work**: Copyright (c) Casey Compton / Ascend Studios, LLC, 2024

The code in this subdirectory is licensed under the MIT License. See the `LICENSE.txt` file in this directory for the full license text.

## Additional Information

Refer to the main README for an overview of the project's licensing and structure.
