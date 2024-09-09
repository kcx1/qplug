---@meta
---@diagnostic disable: missing-return, unused-local

---@class Host
---@field name string
---@field addresses string[]

---@class Interface
---@field Interface string
---@field Address string
---@field MACAddress string
---@field BroadcastAddress string
---@field Gateway string
---@field Netmask string

---@class Network
Network = {}

---Return an object with the name (.name) and addresses (.addresses) of a specified host, where .addresses is a table of strings.
---@param host string The host name for which to obtain network information.
---@return Host
function Network.GetHostByName(host) end

---Return a table of network interface names (.Interface) and the IP address (.Address), MAC address (.MACAddress), broadcast address (.BroadcastAddress),
---gateway (.Gateway), and netmask (.Netmask) for each.
---@return Interface[]
function Network.Interfaces() end
