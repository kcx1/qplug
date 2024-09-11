---@meta
---@diagnostic disable: missing-return, unused-local

---@class DesignStatus
---@field DesignName string
---@field Platform string
---@field IsRedundant boolean
---@field DesignCode string

---@class DesignInventory
---@field Type string
---@field Name string
---@field Location string
---@field Model string
---@field Status.Message string
---@field Status.Code string

---@class Design
Design = {}

---Return a status table containing design information.
---@return DesignStatus
function Design.GetStatus() end

---Return an array of design inventory information, and the details for each inventory item.
---@return DesignInventory[]
function Design.GetInventory() end
