---@meta
---@diagnostic disable: missing-return, unused-local

---@class DanteBrowser
DanteBrowser = {}

---@alias DanteBrowserObject DanteBrowser
DanteBrowserObject = {}

---This returned object can be assigned a callback function. This function is called when a device is added or removed from the network. It takes two arguments - the name of the device and the event type ( "ADDED","REMOVED" ) .-
---@type fun(name, event)
---@param name string
---@param event "ADDED" | "REMOVED"
DanteBrowserObject.Browse = function(name, event) end

---@return DanteBrowserObject
function DanteBrowser.New() end

--- To Implement
function DanteDevice() end
