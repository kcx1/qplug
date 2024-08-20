# Todo

### Functionality

- <input type="checkbox"/> Parse user's config file into the main config
    - <input type="checkbox"/> Utilize user template in the "new" command.
        - <input type="checkbox"/> if UserConfig.me is not set - prompt user for input.
    - <input type="checkbox"/> Utilize user build file in the "build" command.
- <input type="checkbox"/> Create function to locate marker file.
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
- <input type="checkbox"/> Remove .git files from downloaded URLs. 


##### Commands
- Compile -> Only merge the files. (No update to build number)
- Update -> Allow user to update parts of the info.lua file.
    - Name, email, project description.
    - Version number.(This should honestly just be the build number Major.Minor)




### Roadmap
- <input type="checkbox"/> Allow users to set a table that will parse into a ```std::process::Command;``` that can be used instead of a function for the build tool. 
- <input type="checkbox"/> Allow users to set a qplug config per project.
- <input type="checkbox"/> Add cli commands to update info.lua.
    - <input type="checkbox"/> Name, email, project description.
- Allow users to use a flat qplug file.
- Allow users to specify paths in the cli. 
