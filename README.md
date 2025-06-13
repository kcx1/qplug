# 🚀 Q-Plug: Q-SYS Plugin Development Tool! 💪
Cargo inspired CLI for Q-SYS plugin development. 

This tool aims to make developing Q-SYS plugins easier by combining different tools into a single command line interface.

** New Stuff:**
Here's a brief list of new stuff in 0.4.0:

- Definitions for Q-SYS Extensions to LuaLS are no longer embedded in the binary! But you can install them separately using the new install command. You can now find them at: [Q-SYS-LuaLS-Definitions](https://github.com/kcx1/Q-SYS-LuaLS-Definitions)
    - As the scope of qplug grows, it's important to be able to update portions like the definitions without having to rebuild the binary. This also provides an entry point to be able to install and manage new tools in the future.
- Sync Command!
    - Sync files from your IDE to Q-SYS controls.
- Install Command!
    - Install helper tools for Q-Plug (Including the previously included definitions)
    - Install the QSC encryption tool to encrypt your Q-SYS plugins.
    - Install the legacy build tool developed by the Q-SYS team in order to support older plugins.
- Encryption Tool!
    - Wrapper command to leverage the official Q-SYS encryption tool.
    - NOTE: You do need to install the encryption tool first!


**Features:**
- ✨ Create new projects with ease!
    - Template based creation - Bring your own template or use the built-in one 🎉
    - Automatically track changes using git 🔍
    - Includes Lua definitions for the Q-SYS Extensions to Lua 📚
    - Auto fill the info.lua file with your information. 💬
- ⏱️ Build plugins quickly!
    - Compile multiple lua files into a single `.qplug` file 💻
    - Use `require` statement to load your extra lua files 🔧
    - Automatically bump the version number when you build. 📈
    - Configure your own build tool (This lets you carry on using the one from Q-SYS team if you want 😊)
    - Automatically copy the built plugin to your plugin directory *But only if you're on Windows* ¯\\_(ツ)_/¯
- Auto Shell Completion :pencil2:
- Self Updating :shipit:
- Cross Platform 🔀
- Install and use Encryption tool :lock:
- Sync files from your IDE to regular scripting controls within Q-SYS. 󰓦
    - One shot - Sync the current file to Q-SYS
    - Watch mode - Sync the current file to Q-SYS and watch for changes. :eyes:
    - Specify multiple controls to sync various files at once.

## Installation:


#### Get the binaries

Head over to the release tab and download the latest release for your platform. 🎉

[Releases](https://github.com/kcx1/qplug/releases)

Once you have downloaded the release, unzip it and place the contents in your `PATH` (See below).

#### Build from source

Dependencies: 
- [rustup](https://www.rust-lang.org/tools/install) :crab:
- [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) :twisted_rightwards_arrows:

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
  check    check if current directory is a valid plugin.
  install  Install helper
  sync     Sync a lua script to a Q-SYS scripting component on a running/emulating core
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
  -c, --copy_to <Path to copy>
      --build_only             Build only. Do not update plugin info or copy to plugin directory
  -h, --help                   Print help
```

There are a couple of options that you can pass to the build command. 

    -c, --copy_to: This will copy the plugin to the specified path. This is especiallly useful if you are building on an OS other than Windows and would like to copy to a VM or some other location. If you don't provide a path, it will copy the plugin to the qplug plugin folder.

    --build_only: This will only build the plugin. It will not update the plugin info or copy it to the plugin folder. This option replaces the deprecated compile command.

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

##### Install
This command will allow you to install various helper tools.

> [!IMPORTANT]
> As of 0.4.0 the definitions will no longer be embedded in the binary. You will need to install them separately.

The install command will allow you to install external tools to be used by qplug. Each of these tools will be installed to the user's config directory and will display the path of the tool for convenience.


```help
Install helper

Usage: qplug install

Arguments:
  [install]  [possible values: definitions, encryption, legacybuild]

Options:
  -h, --help  Print help
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


##### Sync
Sync local lua files with Q-SYS scripting components.

Okay, okay, so this isn't really plugin management, but it seems like a great addition to the tool and will enhance the workflow.

Simply write a json file(default: `core.json`) and populate it with the following information:


```json
{
    "core": {
            "hostname": "127.0.0.1",
            "username": "core1",
            "password": "password",
            "components": [
                {
                    "name": "SomeComponentName",
                    "script": "Some/Nested/Path/To/SomeComponentName.lua"
                }
            ]
        },
}
```

For your convenience, there is also schema in the doc directory of this repo.

There are a few things to note here:

- The username and password are only required if you have them setup on the core, and they are the same as the ones you use to log into Q-SYS
- The hostname is the IP address or hostname of the core. This can be found in the Q-SYS configuration page.
- The components are the names of the components you want to sync.
- If you provide a script path for a component, it will be used to sync the file with the component. This can be an absolute path or a relative path to the path passed as an argument to the install command. If you don't specify a script, it will use the same script name as the component name relative to the path you pass as an argument.
- You can have multiple components in the `core.json` file, but only one core. 
- You have to use the `.lua` extension for the script name that you want to sync.  

>[!IMPORTANT]
> You **MUST** enable external script access for any of the Q-SYS scripting components you want to sync. The control name needs to be the component name provided in the `core.json` file.


BTW - This is all asynchronous. And will leverage your computer's processing cores to watch files, read from TCP socket, and write back to the TCP socket completely parallel! 😏 thanks rust! 🦀


```help
Sync a lua script to a Q-SYS scripting component on a running/emulating core

Usage: qplug sync [OPTIONS] [script] [config]

Arguments:
  [script]  The parent directory of the script to sync. 
  [config]  Path to the core configuration file. [default: core.json]

Options:
  -w  --watch If this is set, the specified scripts will be watched for changes. and will sync any changes to the core. 
  -h, --help  Print help
```

>[!TIP]
> If you launch the sync command with the watch option, you can cancel it at any time by pressing `CTRL + C`.

>[!TIP]
> You can also use some bash tricks to run the sync command in the background. For example: `qplug sync some/script/path -w &` or after running it `CTRL + Z` to pause the task and then `bg` to put it in the background. Once in the background, you can `fg` to bring it back to the foreground.

>[!TIP]
> The sync command understands the `.` as current directory. So you can create your file system like this: 
> ```
>SomeProjectDir
>    ├── core.json
>    ├── AnotherController.lua
>    └── FirstController.lua
> ```
>Now, simply navigate the `SomeProjectDir`  and there's no need to specify the script path for your components as long as your component names match your file names. You can just call qplug like this: `qplug sync .` or `qplug sync . -w`

#### Global and Local configuration files

You may have multiple projects that require different settings. You can start with a nice global configuration file that has sane default settings for your workflow. Just put the file in `~/.config/qplug/qplug.lua` or directly in your home directory.  `~/.qplug.lua`

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
