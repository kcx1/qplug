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


### Roadmap
- [ ] Allow users to set a table that will parse into a ```std::process::Command;``` that can be used instead of a function for the build tool. 
- [ ] Allow users to set a qplug config per project.
- Allow users to use a flat qplug file.
- Allow users to specify paths in the cli. 
- Try to dynamically add user created controls to the def file. 
    - Read the controls.lua file and run the GetControls function and then append the table to the defs file. This way
