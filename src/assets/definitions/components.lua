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

---@class Components
---@field Name string
---@field Properties ComponentProperties

--- Represents a component object in Q-SYS.
---@class Component
---@field GetComponents fun(): table<number, Components> Return a table of all Named Components in the design and their properties.
---@field GetControls fun(Name: string | Component): table<number, Control>
Component = {}
---```lua
---comps = Component.GetComponents()
---  -- iterate components
--- for _,comp in pairs(comps) do
---  print(string.format("Component '%s' of Type '%s'", comp.Name, comp.Type))
---  -- iterate properties
---  print("---- Properties")
---  for _,prop in pairs(comp.Properties) do
---    print(string.format("  %s ( %s ) : %s", prop.PrettyName, prop.Name, prop.Value))
---  end
---end
---```
---@return table<number, Components>
Component.GetComponents = function() end

---Returns a table of all controls in the specified Named Component.
---
---Example:
---```lua
---blinker = Component.New("led")
---b_ctrls = Component.GetControls(blinker)
---
---print("Component GetControls: ")
---for _,b_element in ipairs(b_ctrls) do
---  print("Name: "..tostring(b_element.Name))
---  print(".\tValue:     "..tostring(b_element.Value))
---  print(".\tString:    "..tostring(b_element.String))
---  print(".\tPosition:  "..tostring(b_element.Position))
---  print(".\tType:      "..tostring(b_element.Type))
---  print(".\tDirection: "..tostring(b_element.Direction))
---  print(".\tMinValue:  "..tostring(b_element.MinValue))
---  print(".\tMaxValue:  "..tostring(b_element.MaxValue))
---  print(".\tMinString: "..tostring(b_element.MinString))
---  print(".\tMaxString: "..tostring(b_element.MaxString))
---end
---```
---@param Name string
---@return table<number, Control>
Component.GetControls = function(Name) end

--- Creates a new Component instance with a specified name.
--- Example:
--- ```lua
---gainA = Component.New("gain a")
---gainB = Component.New("gain b")
---gainA.gain.EventHandler = function( ctl )
--- if gainA.gain.Value > 0 then
---  gainB.mute.Boolean = true
--- else
---  gainB.mute.Boolean = false
--- end
---end
---```
--- @param Name string The name of the component
--- @return Components # The newly created Component instance
function Component:New(Name) end
