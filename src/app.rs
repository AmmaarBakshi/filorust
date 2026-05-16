use std::path::PathBuf;
use filorust_core::FileItem;

pub struct AppState {
    /// The current directory path visible in the address bar
    pub current_directory: PathBuf,
    /// Items inside the current directory
    pub items: Vec<FileItem>,
    /// Index of the currently highlighted/selected file or folder
    pub selected_index: Option<usize>,
    /// History for the "Back" button
    pub back_stack: Vec<PathBuf>,
    /// History for the "Forward" button
    pub forward_stack: Vec<PathBuf>,
}

impl AppState {
    pub fn new(start_dir: PathBuf) -> Self {
        let mut app = Self {
            current_directory: start_dir,
            items: Vec::new(),
            selected_index: None,
            back_stack: Vec::new(),
            forward_stack: Vec::new(),
        };
        app.refresh_directory();
        app
    }

    /// Reloads files from the current folder path
    pub fn refresh_directory(&mut self) {
        if let Ok(new_items) = filorust_core::read_dir(&self.current_directory) {
            self.items = new_items;
            self.selected_index = if self.items.is_empty() { None } else { Some(0) };
        }
    }

    /// Triggered when double-clicking a directory or pressing Enter
    pub fn navigate_to(&mut self, new_path: PathBuf) {
        if new_path.is_dir() {
            let old_path = self.current_directory.clone();
            self.back_stack.push(old_path);
            self.forward_stack.clear(); // Navigating to a new branch clears the forward stack
            self.current_directory = new_path;
            self.refresh_directory();
        }
    }

    /// Windows Explorer "Back Arrow" action
    pub fn navigate_back(&mut self) {
        if let Some(prev_path) = self.back_stack.pop() {
            let current = self.current_directory.clone();
            self.forward_stack.push(current);
            self.current_directory = prev_path;
            self.refresh_directory();
        }
    }

    /// Windows Explorer "Forward Arrow" action
    pub fn navigate_forward(&mut self) {
        if let Some(next_path) = self.forward_stack.pop() {
            self.back_stack.push(self.current_directory.clone());
            self.current_directory = next_path;
            self.refresh_directory();
        }
    }

    /// Windows Explorer "Up Arrow" action (Parent Directory)
    pub fn navigate_up(&mut self) {
        if let Some(parent) = self.current_directory.parent() {
            let parent_buf = parent.to_path_buf();
            self.navigate_to(parent_buf);
        }
    }
}