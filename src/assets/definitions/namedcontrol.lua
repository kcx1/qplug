---@meta
---@diagnostic disable: missing-return, unused-local

---The methods in NamedControl are used to read or set the values of Named Controls.
NamedControl = {}

---Sets control with specified Control name to specified string value. If Control name does not exist an error is raised.
---@param control_name string the name of the control
---@param string string value to set
function NamedControl.SetString(control_name, string) end

---Returns string of control with specified Control name. If Control name does not exist an error is raised.
---@param control_name string the name of the control
---@return string string
function NamedControl.GetString(control_name) end

---Sets control with specified Control name to specified position. The ramp time is optional, and is in seconds. If Control name does not exist an error is raised.
---
---@param control_name string the name of the control
---@param position number floating point position which goes from 0.0 -> 1.0
---@param ramp number? optional ramp time to get to the selected value
function NamedControl.SetPosition(control_name, position, ramp) end

---Returns position of control with specified Control name. If Control name does not exist an error is raised.
---@param control_name string the name of the control
---@return number position
function NamedControl.GetPosition(control_name) end

---Sets control with specified Control name to specified value. The ramp time is optional, and is in seconds. If Control name does not exist an error is raised.
---@param control_name string the name of the control
---@param value number string representation of control value
---@param ramp number? optional ramp time to get to the selected value
function NamedControl.SetValue(control_name, value, ramp) end

---Returns value of control with specified Control name. If Control name does not exist an error is raised.
---@param control_name string the name of the control
---@return number value
function NamedControl.GetValue(control_name) end

---Triggers control with specified Control name. If Control name does not exist an error is raised.
---@param control_name string the name of the control
function NamedControl.Trigger(control_name) end
