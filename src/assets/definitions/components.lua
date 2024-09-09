---@meta
---@diagnostic disable: missing-return, unused-local

---@class ComponentProperties
---@field Value any
---@field String string
---@field Position number
---@field Type string
---@field Direcetion string
---@field MinValue number
---@field MaxValue number
---@field MinString string
---@field MaxString string
-- ComponentProperties = {}

---@class Components
---@field Name string
---@field Properties ComponentProperties

--- Represents a component object in Q-SYS.
---@class Component
Component = {}

--- Creates a new Component instance with a specified name.
--- @param Name string The name of the component
--- @return self Component # The newly created Component instance
function Component:New(Name)
	return self
end

--- Return a table of all Named Components in the design and their properties.
---@return table<number, Components> Components
function Component.GetComponents() end

--- Returns a table of all controls in the specified Named Component.
---@param Name string | Component The name of the component
---@return table<number, Control> Components A table where keys are control names and values are ControlInputOutput objects
function Component.GetControls(Name) end
