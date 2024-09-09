---@meta
---@diagnostic disable: missing-return, unused-local

---Use the SNMPTrap library in Lua to receive trap notifications from an SNMP-enabled device.
---Note: In the SNMP model, Q-SYS acts as the SNMP Manager, while the device you intend to monitor runs an SNMP Agent that allows for monitoring.
SNMPTrap = {}

---Create a new SNMP trap listener.
---trap_name : The name of the trap listener session.
function SNMPTrap.New(trap_name) end

---Begin a trap listening session.
function SNMPTrap:startSession() end

---Assign a Lua callback for successful SNMP events.
function SNMPTrap.EventHandler() end

---Assign a Lua callback for unsuccessful SNMP events.
function SNMPTrap.ErrorHandler() end
