// When move some code to a new file in a different directory, 
// directory itself acts as a module. And mod.rs in the module 
// root is the entry point to the directory module. 
// All other files in that directory, acts as a sub module of that directory.
pub mod points;
pub mod vehicles;