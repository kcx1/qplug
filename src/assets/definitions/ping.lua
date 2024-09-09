---@meta
---@diagnostic disable: missing-return, unused-local

---Use the Ping library in Lua to check whether a device is reachable on the network.
---Note: The Lua Ping library requires that you run Q-SYS Designer as administrator ("Run as administrator" option in Windows) when emulating your design.
---If you see a non-terminating "Socket failed to open" error message in the debug window, re-launch Designer as administrator.
---Ping object MUST be initialized with the .New function
---@class Ping
---@field host string
Ping = {}

---@class PingResponses
---@field HostName string
---@field ElapsedTime number

---Create a new ping object.
---@param target_host string The hostname to ping.
---@return Ping
function Ping.New(target_host) end

---Begin the ping session.
---@param self Ping
---@param single_shot boolean Set to true if you want only a single ping attempt.
function Ping:start(single_shot) end

---Stop the ping session.
---@param self Ping
function Ping:stop() end

---Set the timeout for waiting for a ping response.
---@param self Ping
---@param interval number The timeout duration, in seconds.
function Ping:setTimeoutInterval(interval) end

---Set the interval for retrying after a ping request.
---@param self Ping
---@param interval number The timeout duration, in seconds.
function Ping:setPingInterval(interval) end

---Assign a Lua callback for successful ping events.
---@param response PingResponses callback for responseobject
Ping.EventHandler = function(response) end

---Assign a Lua callback for unsuccessful ping events.
---@param response PingResponses callback for responseobject
Ping.ErrorHandler = function(response) end
