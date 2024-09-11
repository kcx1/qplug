---@meta
---@diagnostic disable: missing-return, unused-local

---@enum UserControls
local UserControls = {
	-- Some Controls
}

--- Represents a control object in Q-SYS.
--- @see Controls
---@class Control
---@field String string
---@field Value number
---@field Position number
---@field Boolean boolean
---@field Choices table<number, string>
---@field Color string Color of the control
---@field CssClass string
---@field IsDisabled boolean is the control disabled
---@field IsIndeterminate boolean is the control in an indeterminate state
---@field IsInvisible boolean Is the control invisible
---@field Legend string Legend of the Control
---@field RampTime number time in seconds to ramp to the new value
---@field EventHandler fun(self: Control) Function which is called when any control property changes

---@alias controlDict table<string, Control>
---@alias controlArray table<number, Control>
---@alias controlDictArrays table<string, controlArray>

---Q-Sys Control table.
---@type controlDict | controlArray | controlDictArrays
Controls = {}
