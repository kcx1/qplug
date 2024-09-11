---@meta
---@diagnostic disable: missing-return, unused-local

---Send and receive data over a UDP Socket connection.
---
---Example:
---```lua
-----(Port 2468 is the port for the Q-SYS Core's IO device queries sent every two seconds)
---udp = UdpSocket.New()  --Create new UDP object
---
---udp.EventHandler = function(udp, packet)
---    print( packet.Address, packet.Port, packet.Data )
---end
---
---udp:Open( "192.168.1.100", 2468 ) -- IP address is optional
---udp:JoinMulticast("224.0.23.175") -- Sends a multicast join report for the multicast address
---```
---@class UdpSocket
---@field MulticastTtl number Multicast TTL value 0 - 255
---@field EventHandler fun(self: UdpSocket, data: {Address: string, Port: number, Data: string})
---@field New fun(): UdpSocket Creates a new UdpSocket instance.
---@field Open fun(self: UdpSocket, ip_address: string, port: number) Open the socket on the specified port
---@field Close fun(self: UdpSocket) Closes the UDP socket
---@field Data fun(self: UdpSocket, data: {Address: string, Port: number, Data: string})
UdpSocket = {}

---Creates a UDP Socket instance.
---@return UdpSocket
function UdpSocket.New() end

---Opens the UDP 'listener.' Optionally bind to local IP and Port.
---@param ip_address string ip address where to listen to, ususally is a bind to the local address
---@param port number listening port
function UdpSocket:Open(ip_address, port) end

---Sends data to the specified ip address and port
---
---Example:
---```lua
---local udp = UdpSocket.New()
---
---udp:Open()
---udp:Send(
---    "224.0.23.175",
---    2467,
---    "<QDP><query_ref>device.ioframe.*</query_ref></QDP>" )
---```
---@param self UdpSocket
---@param ip_address string destination IP Address/Host
---@param port number listening port
---@param data string data to send
function UdpSocket:Send(ip_address, port, data) end

---Joins a specific multicast 'address', optionally binding to a local 'ip'.
---@param address string multicast address to join
---@param local_ip string? Local IP address to bind to. This is an optional argument for single NIC systems, otherwise QDS will randomly choose a NIC to receive packets from and might not be the correct one in a multi-NIC system.
function UdpSocket:JoinMulticast(address, local_ip) end
