# Todo

### Functionality

- [ ] Parse user's config file into the main config
    - [ ] Utilize user template in the "new" command.
        - [x] if UserConfig.me is not set - prompt user for input.
    - [ ] Utilize user build file in the "build" command.
- Fix Defs file. :warning:
- [x] Create function to locate marker file.
    ```rust
    fn find_project_root(starting_dir: &Path) -> Option<PathBuf> {
        let mut current_dir = starting_dir;

        loop {
            if current_dir.join(".my_cmd").exists() {
                return Some(current_dir.to_path_buf());
            }

            match current_dir.parent() {
                Some(parent) => current_dir = parent,
                None => break,
            }
        }

        None
    }
    ```
- [x] Remove .git files from downloaded URLs. 

### Notes

- Update the build_tool to potentially accept args. 
    - Currently, build_tool grabs the init.lua file no matter what. I'd like to provide the user a way to pass in the file that they want to build / compile. This would require moving the locate the init.lua file function to the build tool. The default tool lives in the compile.rs file. the find init needs to be removed from here and passed in as an Optional argument.
    - Implementing path is safe to use a string type.
- Make any CLI options available in the config file.
- Speed up program by using config to define project layout. Fallback to using manual searches.
- Create init command that useds an enum of additional commands that allow a user to automatically do the things that I think they would want to do often. For example, init legacy, init existing, init touch. (Create a profile that runs the oringal Q-SYS version, create a qplug for existing project, no git files or anything. init touch: just create a blank .qplug file)
- Add encryption method. 
add the requires.lua file to the template. 


### Roadmap
- [ ] Allow users to set a table that will parse into a ```std::process::Command;``` that can be used instead of a function for the build tool. 
- [ ] Allow users to set a qplug config per project.
- Create a table of functions that users can define and call from the command line to allow custom automations. 
- Allow users to use a flat qplug file.
- Allow users to specify paths in the cli. 
- Try to dynamically add user created controls to the def file. 
    - Read the controls.lua file and run the GetControls function and then append the table to the defs file. This way
