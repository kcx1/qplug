use crate::files::find_marker_file;

pub fn check() {
    let marker_file = find_marker_file(None);
    match marker_file {
        Some(f) => println!("Qplug plugin found! {:?}", f),
        None => println!("Not a Qplug plugin. You may want to try `qplug init` or navigating to a qplug directory."),
    }
}
