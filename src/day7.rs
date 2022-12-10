use std::{
    collections::HashMap,
    fmt::{Display, Debug},
    sync::{atomic::AtomicUsize},
};

pub struct Directory {
    directory_name: String,
    files: Vec<File>,
}

impl Directory {
    fn new(directory_name: String) -> Self {
        Self {
            directory_name,
            files: Vec::new(),
        }
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.directory_name)
    }
}

#[derive(Debug)]
pub struct File {
    file_name: String,
    file_size: usize,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.file_name)
    }
}

pub struct Node<T> {
    pub id: usize,
    pub data: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T> Node<T> {
    pub fn new(id: usize, parent_id: usize, data: T) -> Node<T> {
        Node {
            id,
            data,
            parent: Some(parent_id),
            children: Vec::new(),
        }
    }

    pub fn new_root(id: usize, data: T) -> Node<T> {
        Node {
            id,
            data,
            parent: None,
            children: Vec::new(),
        }
    }
}

pub struct Arena<T> 
{
    nodes: HashMap<usize, Node<T>>,
    atomic_counter: AtomicUsize,
}

impl<T> Default for Arena<T>
{
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            atomic_counter: AtomicUsize::new(Self::ROOT_NODE_INDEX + 1)
        }
    }
}

impl<T> Arena<T> {
    const ROOT_NODE_INDEX: usize = 0;

    pub fn get_node(&self, id: &usize) -> Option<&Node<T>> {
        self.nodes.get(id)
    }
    pub fn get_node_mut(&mut self, id: &usize) -> Option<&mut Node<T>> {
        self.nodes.get_mut(id)
    }

    pub fn add_root_node(&mut self, data: T) -> Result<usize, String> {
        if self.nodes.contains_key(&Self::ROOT_NODE_INDEX) {
            return Err("Root already exists.".to_string());
        }
        
        let root_node = Node::new_root(Self::ROOT_NODE_INDEX, data);
        
        self.nodes.insert(Self::ROOT_NODE_INDEX, root_node);

        Ok(Self::ROOT_NODE_INDEX)
    }

    pub fn add_node(&mut self, data: T, parent_id: usize) -> Result<usize, String> {
        if !self.node_exists(&parent_id)  {
            return Err(format!("Parent {} does not exist", parent_id));
        }

        let next_index = self.generate_id();
        let node = Node::new(next_index, parent_id, data);

        self.nodes.insert(next_index, node);
        
        let parent_node = self.nodes.get_mut(&parent_id).unwrap();
        parent_node.children.push(next_index);

        Ok(next_index)
    }


    pub fn node_exists(&self, node_id: &usize) -> bool {
        self.nodes.contains_key(node_id)
    }

    fn generate_id(&self) -> usize {
        self.atomic_counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}


pub struct Terminal {
    directory_arena: Arena<Directory>,
    current_directory: usize,
}

impl Default for Terminal {
    fn default() -> Self {
        let mut arena = Arena::default();
        let root_dir = Directory {
            directory_name: "/".to_string(),
            files: vec![],
        };

        let root_dir_index = arena
            .add_root_node(root_dir)
            .expect("root node should not exist yet");

        Self {
            directory_arena: arena,
            current_directory: root_dir_index,
        }
    }
}

impl Terminal {
    pub fn execute_command(&mut self, command: &str) -> Result<(), String> {
        if command.starts_with("cd") {
            self.change_directory(&command[3..])
        } else if command.starts_with("ls") {
            Ok(())
        } else {
            Err(format!("Unknown command '{}'", command))
        }
    }

    pub fn feed_command_result(&mut self, result_line: &str) -> Result<(), String> {
        let mut parts = result_line.split_ascii_whitespace();

        let size_or_dir = parts.next().expect("size or dir must be set");
        let dir_or_file_name = parts.next().expect("file or dir name must be set");

        if let Ok(size) = size_or_dir.parse::<usize>() {
            let file = File {
                file_name: dir_or_file_name.to_string(),
                file_size: size,
            };

            let current_node = self.directory_arena
                .get_node_mut(&self.current_directory)
                .expect("node should exist");

            current_node.data.files.push(file);
        } else {
            let directory = Directory::new(dir_or_file_name.to_string());
            self.directory_arena
                .add_node(directory, self.current_directory)
                .expect("node should be added");

        }

        Ok(())
    }

    fn change_directory(&mut self, new_dir: &str) -> Result<(), String> {
        match new_dir {
            ".." => self.go_back_dir(),
            "/" => {
                self.go_to_root();
                Ok(())
            },
            _ => self.go_to_dir(new_dir),
        }
    }

    fn go_back_dir(&mut self) -> Result<(), String> {
        let current_node = self.get_current_dir();
        
        if let Some(parent_dir_index) = current_node.parent {
            self.current_directory = parent_dir_index;
            Ok(())
        } else {
            Err("Current directory is root.".to_string())
        }
    }

    fn go_to_dir(&mut self, new_dir: &str) -> Result<(), String> {
        let current_node = self.get_current_dir();

        let target_directory = current_node.children
            .iter()
            .map(|x| self.directory_arena.get_node(x).unwrap())
            .find(|x|  x.data.directory_name == new_dir)
            .expect("given directory should exist");

        self.current_directory = target_directory.id;

        Ok(())
    }

    fn go_to_root(&mut self) {
        loop {
            let current_dir_is_root = self.go_back_dir().is_err();
            if current_dir_is_root {
                break;
            }
        }
    }

    pub fn get_current_dir(&self) -> &Node<Directory> {
        self.directory_arena.get_node(&self.current_directory).unwrap()
    }

    pub fn get_directory_sizes_from_selected_dir(&self, selected_node: &Node<Directory>) -> HashMap<usize, usize> {
        let mut result: HashMap<usize, usize> = HashMap::new();

        let selected_node_children = selected_node.children.iter()
            .map(|x| self.directory_arena.get_node(x).unwrap());

        let mut sum = 0;
        for selected_node_child in selected_node_children {
            let child_folder_sizes = self.get_directory_sizes_from_selected_dir(selected_node_child);
            let my_direct_child_size = child_folder_sizes.get(&selected_node_child.id).unwrap();
            sum += my_direct_child_size;
            result.extend(child_folder_sizes);
        }
        
        let selected_node_file_size_sum = selected_node.data.files.iter().fold(0, |sum, x| sum + x.file_size);
        sum += selected_node_file_size_sum;

        result.insert(selected_node.id, sum);

        result
    }

}
