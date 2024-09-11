---@meta
---@diagnostic disable: missing-return, unused-local

---Use the directory commands to list folders and files, create folders, and delete folders within the media/ or design/ locations on the file system. For security and stability reasons, these are the only locations accessible by the Lua libraries.
---The media/ folder is the location for all media files, while the design/ folder is the location where uncompressed design configuration files reside while a design is being emulated or running on a Q-SYS Core processor. (It is not intended for storage of user-created design files, and is not remotely accessible.)
---@class dir
dir = {}

---@class DirEntry
---@field name string
---@field size number
---@field type "file" | "directory"

---List directories and files in a media/ or design/ path.
---@param path string Must begin with /media or design/. Surround the path with quotes.
---@return DirEntry[]
function dir.get(path) end

---Create a new folder within a media/ or design/ path.
---@param path string Must begin with /media or design/. Surround the path with quotes.
function dir.create(path) end

---Remove an empty folder within a media/ or design/ path.
---@param path string Must begin with /media or design/. Surround the path with quotes.
---Note: The folder you are removing must be empty. See the Example section for an iterative method to remove files before removing the folder.
function dir.remove(path) end
