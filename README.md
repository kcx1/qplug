# qplug
Cargo inspired cli for Q-SYS plugin development. 

This tool aims to make developing Q-SYS plugins easier by combining different tools into a single command line interface.

**Features:**
- Create new projects
    - Template based creation - Bring your own template or use the built in one
    - Automatically track changes using git
    - Includes Lua definitions for the Q-Sys Extensions to Lua
    - Auto fill the info.lua file with your information.
- Build plugins
    - Compile multiple lua files into a single qplug file
    - Use `require` statement to load your extra lua files
    - Automatically bump the version number when you build. 
    - Configure your own build tool (This let's you carry on using the one from the Q-Sys team if you want)
    - Automatically copy the built plugin to your plugin directory (But only if you're on windows ;-) )



## Installation:


#### Get the binaries

Head over to the release tab and download the latest release for your platform.

[Releases](https://github.com/kcx1/qplug/releases)

Once you have downloaded the release, unzip it and place the contents in your `PATH` (See below).


#### Build from source

Dependencies: 
- [rustup](https://www.rust-lang.org/tools/install)
- [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

Simply clone this repo and build it with 

``` bash
git clone https://github.com/kcx1/qplug 
cargo build --release
```

The newly created binaries are located in `./qplug/target/release`

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

Adding qplug to windows is pretty straightforward too. 

[Instructions](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/)

You'll simply want to add the directory containing the binaries to qplug.

I simply created a file in my home folder called qplug. Then I moved the downloaded binaries there and added that folder to my path. 


#### Configuration
...

## Using It

