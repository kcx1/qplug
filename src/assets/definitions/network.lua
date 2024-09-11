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
---
---Example:
---```lua
---local ghbn = Network.GetHostByName('localhost')
---print("GetHostByName("..ghbn.name..")")
---for _, item in ipairs(ghbn.addresses) do
---  print("-"..item)
---end
---```
---@param host string The host name for which to obtain network information.
---@return Host
function Network.GetHostByName(host) end

---Return a table of network interface names (.Interface) and the IP address (.Address), MAC address (.MACAddress), broadcast address (.BroadcastAddress), gateway (.Gateway), and netmask (.Netmask) for each.
---
---Example:
---```lua
---local ni = Network.Interfaces()
---for _, item in ipairs(ni) do
---  print("-"..item.Interface, " = \n IP ", item.Address,"\n MAC",item.MACAddress,"\n Broadcast",
---  item.BroadcastAddress,"\n Gateway",item.Gateway,"\n Netmask",item.Netmask)
---end
---```
---@return Interface[]
function Network.Interfaces() end
