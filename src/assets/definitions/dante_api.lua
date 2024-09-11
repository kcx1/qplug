---@meta
---@diagnostic disable: missing-return, unused-local

---@class DanteBrowser
---@field New fun(): DanteBrowser Creates a new Browser instance.
---@field Browse fun(name: string, event: "ADDED" | "REMOVED") callback function when a device is added or removed.
DanteBrowser = {}

--- To Implement
function DanteDevice() end
