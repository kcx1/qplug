---@meta
---@diagnostic disable: missing-return, unused-local

---Use the SNMPTrap library in Lua to receive trap notifications from an SNMP-enabled device.
---Note: In the SNMP model, Q-SYS acts as the SNMP Manager, while the device you intend to monitor runs an SNMP Agent that allows for monitoring.
---@class SNMPTrap
---@field New fun(trap_name : string) : SNMPTrap Create a new SNMP trap listener.
---@field startSession fun(self: SNMPTrap) Begin a trap listening session.
---@field EventHandler fun(response: SNMP.Response) Assign a Lua callback for successful SNMP events.
---@field ErrorHandler fun(response: SNMP.ErrorResponse) Assign a Lua callback for unsuccessful SNMP events.
SNMPTrap = {}
