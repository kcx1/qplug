---@meta
---@diagnostic disable: missing-return, unused-local

---@class Log
Log = {}

---Use the Log object to write messages and errors to the Q-SYS Core processor's system log file.
---@param message string
function Log.Message(message) end

---Write an error message to the Core's log file.
---@param message string
function Log.Error(message) end
