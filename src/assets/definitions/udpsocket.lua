---@meta
---@diagnostic disable: missing-return, unused-local

---Send and receive data over a UDP Socket connection.
---@class UdpSocket
---@field Data fun(data)?
UdpSocket = {}

---Creates a UDP Socket instance.
---@return UdpSocket
function UdpSocket.New() end

---Opens the UDP 'listener.' Optionally bind to local IP and Port.
---@param self UdpSocket
---@param ip_address string ip address where to listen to, ususally is a bind to the local address
---@param port number listening port
function UdpSocket:Open(ip_address, port) end

---Closes the UDP socket
---@param self UdpSocket
function UdpSocket:Close() end

---Sends data to the specified ip address and port
---@param self UdpSocket
---@param ip_address string destination IP Address/Host
---@param port number listening port
---@param data string data to send
function UdpSocket:Send(ip_address, port, data) end

---Joins a specific multicast 'address', optionally binding to a local 'ip'.
---@param address string multicast address to join
---@param ip string? optional local ip binding
function UdpSocket:JoinMulticast(address, ip) end
