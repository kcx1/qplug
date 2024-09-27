# 🚀 Q-Plug: Your Q-SYS Plugin Development Superhero! 💪
Cargo inspired CLI for Q-SYS plugin development. 

This tool aims to make developing Q-SYS plugins easier by combining different tools into a single command line interface.

**Features:**
- ✨ Create new projects with ease!
    - Template based creation - Bring your own template or use the built-in one 🎉
    - Automatically track changes using git 🔍
    - Includes Lua definitions for the Q-SYS Extensions to Lua 📚
    - Auto fill the info.lua file with your information. 💬
- ⏱️ Build plugins quickly!
    - Compile multiple lua files into a single qplug file 💻
    - Use `require` statement to load your extra lua files 🔧
    - Automatically bump the version number when you build. 📈
    - Configure your own build tool (This lets you carry on using the one from Q-SYS team if you want 😊)
    - Automatically copy the built plugin to your plugin directory (But only if you're on Windows 🤣)
- Auto Shell Completion 🔁
- Self Updating 💸

## Installation:


#### Get the binaries

Head over to the release tab and download the latest release for your platform. 🎉

[Releases](https://github.com/kcx1/qplug/releases)

Once you have downloaded the release, unzip it and place the contents in your `PATH` (See below).

#### Build from source

Dependencies: 
- [rustup](https://www.rust-lang.org/tools/install) 👋
- [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) 🤫

Simply clone this repo and build it with 

```bash
git clone https://github.com/kcx1/qplug 
cargo build --release
```

The newly created binaries are located in `./qplug/target/release` 💻.

### Set it up

#### Add it to PATH

To make things a little easier, add the new binaries to your `$PATH`. You can do this two ways: 

1. Add the binaries to a directory that is already on your `$PATH`
2. Add a new directory containing the binaries to your `$PATH`

##### Linux and MacOS:
Adding a new directory containing the new binary to your `$PATH`
```bash
PATH="some/folder/with/qplug":$PATH"
```

> [!NOTE]  
> You can make this permanent by adding this line to your `.bashrc` or `.zshrc` file. 
> ```bash
> export PATH=$PATH:some/folder/with/qplug
> ``` 


##### Windows:

Adding Q-Plug to windows is pretty straightforward too. 

[Instructions](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/)

You'll simply want to add the directory containing the binaries to qplug.

I simply created a file in my home folder called qplug. Then I moved the downloaded binaries there and added that folder to my path. 😊

## Using It


### Available Commands
```help
Commands:
  new      Create a new plugin template.
  build    Build and complie the plugin.
  update   Update the qplug utility to the latest version.
  copy     Copy the plugin to the plugin folder.
  compile  Complie the plugin. Do not increment versioning or copy to plugin folder.
  check    check if current directory is a valid plugin.
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Getting Started:
Get started with Q-Plug in no time!

You can create a new project using the new command. 

##### Create a new project

You can create a new project using the new command. This will create a new Q-SYS plugin project. If you provide a name, it will be used as the name of the project, otherwise it will be the name of the current directory. 

By default, this will create a couple of directories and files. 

- `.git/`: This is the git repository. If you don't want to initialize git, add the --no-git flag to your new command.
- `definitions/`: This is the directory that contains the Lua definitions for the Q-SYS Extensions. If you don't want to create the definitions, add the --no-defs flag to your new command.
- `plugin_src/`: This is the source code for the plugin. It is generated from the template (either provided or the builtin). If you don't want to create a template, add the --no-template flag to your new command.
- `.qplug`: This is a marker file that is used to identify a Q-Plug project. Additionally, it can be used as a local project config file.

```help
qplug new [name]

Create a new Q-Plug project.  

Usage: qplug new [OPTIONS] [Name]

Arguments:
  [Name] name of the project. If you don't provide one, it will be the name of the current directory.


Options:
      --no-git      Do not initialize git
      --no-defs     Do not create the definitions files
      --no-template Do not create a template
  -h, --help        Print help
```

##### Build a plugin

You can build a plugin using the build command. The build command executes (3) steps:
    1. **Increment** the version number. By default, if not provided, it will bump the Dev version number. 
        - Dev
        - Patch
        - Minor
        - Major
    2. **Compile** the plugin. This will create a single `.qplug` file. This step is the same as running the `qplug compile` command.
    3. **Copy** the plugin to the plugin folder. This feature only works on Windows.
```help
Build and compile the plugin.

Usage: qplug build [Increment Build Version]

Arguments:
  [Increment Build Version]  [default: dev] [possible values: dev, patch, minor, major]

Options:
  -h, --help  Print help
```

##### Compile a plugin

This will compile a `.qplug` file based on the contents of the Lua files within the `plugin_src` directory.
By default, it will use the builtin build tool. However, you can specify your own within the config file. (see below)

```help
Compile the plugin. Do not increment versioning or copy to plugin folder.

Usage: qplug compile

Options:
  -h, --help  Print help
```

##### Copy the plugin to the plugin folder
This will copy the plugin to the Q-SYS plugin folder.

```help
Copy the plugin to the plugin folder.

Usage: qplug copy

Options:
  -h, --help  Print help
```

##### Check Q-plug
This command will allow you to check a few different things. 
- Version: This will return the version number of the Q-plug itself (this can also be found using `qplug -V`)
- Q-plug: Check if your current directory is a Q-Plug project!
- Config: Get the path of the discovered configuration, if any.

```help
check    check if current directory is a valid plugin.

Options:
  -h, --help     Print help
```

##### Update Q-plug
Have Q-Plug update itself to the latest version so you don't have to worry about downloading new binaries and updating your path. 

You can specify a version to update to (or roll back). If you don't specify one, it will update to the latest version.
```help
Update the qplug utility to the latest version.

Usage: qplug update

Options:
  -v, --version <Version>  Specify the version to update (or roll back) to. If omitted, defaults to the latest version. Pass the current version 
to force update.
  -h, --help               Print help
```
### Configuration

You can configure various aspects of Q-Plug using the following commands and files.

#### Global configuration file

You may have multiple projects that require different settings. One way to accomplish this is by defining your own global configuration file. You can store this in either `~/.config/qplug/qplug.lua` or by adding a `.qplug.lua` file directly in your home directory.

Here's an example:
```lua
-- Example Q-Plug configuration.
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

    -- [[ External Build Tool ]] --
    -- You can define your own build tool. Must be a function that doesn't take any arguments or return any values.

    build_tool = function()
        local cmd = ".\\plugincompile|PLUGCC.exe . .\\plugin.lua"
	os.execute(cmd)
    end,
}
```

## Contributing
Contributions to Q-Plug are welcome! Please follow standard coding practices and ensure that any changes do not break existing functionality.

## Reporting Issues
If you encounter any issues while using Q-Plug, feel free to report them. Your feedback is essential in helping to improve the app.

## License
Q-Plug is released under the MIT license.
