---@meta
---@diagnostic disable: missing-return, unused-local

---The TcpSocket object allows Q-SYS cores to make client TCP/IP connections to devices on the network.
---
---# Example:
---
---## EventHandler:
---```lua
--- address = "192.168.1.100"
--- port = 1234
--- sock = TcpSocket.New()
--- sock.ReadTimeout = 0
--- sock.WriteTimeout = 0
--- sock.ReconnectTimeout = 5
---
--- sendData = 'Hello\x0d\x0a'
---
--- sock.EventHandler = function(sock, evt, err)
---   if evt == TcpSocket.Events.Connected then
---     print( "socket connected" )
---   elseif evt == TcpSocket.Events.Reconnect then
---     print( "socket reconnecting..." )
---   elseif evt == TcpSocket.Events.Data then
---     print( "socket has data" )
---     message = sock:ReadLine(TcpSocket.EOL.Any)
---     while (message ~= nil) do
---       print( "reading until CrLf got "..message )
---       message = sock:ReadLine(TcpSocket.EOL.Any)
---     end
---   elseif evt == TcpSocket.Events.Closed then
---     print( "socket closed by remote" )
---   elseif evt == TcpSocket.Events.Error then
---     print( "socket closed due to error", err )
---   elseif evt == TcpSocket.Events.Timeout then
---     print( "socket closed due to timeout" )
---   else
---     print( "unknown socket event", evt ) --should never happen
---   end
--- end
---
--- Controls.Inputs[1].EventHandler = function()
---   sock:Write(sendData)
--- end
---```
---## Callbacks:
---```lua
---address = "192.168.1.100"
---port = 1234
---sock = TcpSocket.New()
---sock.ReadTimeout = 0
---sock.WriteTimeout = 0
---sock.ReconnectTimeout = 5
---
---sendData = 'Hello\x0d\x0a'
---
---sock.Connected = function(sock)
---  print("TCP socket is connected")
---end
---sock.Reconnect = function(sock)
---  print("TCP socket is reconnecting")
---end
---sock.Data = function(sock)
---  print("TCP socket has data:",sock:Read(sock.BufferLength) )
---end
---sock.Closed = function(sock)
---  print("TCP socket was closed by the remote end")
---end
---sock.Error = function(sock, err)
---  print("TCP socket had an error:",err)
---end
---sock.Timeout = function(sock, err)
---  print("TCP socket timed out",err)
---end
---
---Controls.Inputs[1].EventHandler = function()
---  sock:Write(sendData)
---end
---
---sock:Connect(address, port)
---```
---@class TcpSocket
---@field ReadTimeout number Time, in seconds, to wait for data to be available on socket before raising an Error through the EventHandler.
---@field WriteTimeout number Time, in seconds, to wait for data write to complete before raising an Error through the EventHandler.
---@field ReconnectTimeout number Time in seconds to wait before attempting to reconnect. 0 disables automatic reconnect.
---@field IsConnected boolean Returns true if socket is connected
---@field BufferLength number Amount of data in buffer, in bytes
---@field EOL EndOfLine
---@field Events Events
---@field EventHandler fun(sock: TcpSocket, event: Events, err: any)
---@field Connected fun(sock: TcpSocket)
---@field Reconnect fun(sock: TcpSocket)
---@field Data fun(sock: TcpSocket)
---@field Closed fun(sock: TcpSocket)
---@field Error fun(sock: TcpSocket, err: any)
---@field Timeout fun(sock: TcpSocket, err: any)
TcpSocket = {}

---Creates a new TcpSocket instance.
---@return TcpSocket
function TcpSocket:New() end

---Attempts to connect to the specified ip/host name and port, 'ipAddress/hostname' is string, 'port' is integer
---@param self TcpSocket
---@param ip_address string the ip address or host name to connect to
---@param port number control port
function TcpSocket:Connect(ip_address, port) end

---Disconnects the socket.
---@param self TcpSocket
function TcpSocket:Disconnect() end

---@param self TcpSocket
---Writes specified data to the socket. Raises error if socket is not connected.
---@param data string the data to write to the socket
function TcpSocket:Write(data) end

---@param self TcpSocket
---Attempts to read up to 'length' bytes from socket. These bytes are removed from the buffer, leaving any remaining bytes beyond the 'length' specified. 'length' is positive integer.
---@param length number of bytes to read
function TcpSocket:Read(length) end

---Attempts to read a 'line' from the socket buffer. 'EOL' is defined in the table below. '<custom>' is an optional string only used by EOL.Custom.
---@param self TcpSocket
---@param EOL EndOfLine see associated table for values
---@param custom string? if EOL.Custom is used this variable se the custom value
function TcpSocket:ReadLine(EOL, custom) end

---Searches the socket buffer for string 'str' (starting at integer 'start_pos') and returns the index of where 'str' is found. 'start_pos' defaults to 1.
---@param self TcpSocket
---@param str string the string/pattern to find
---@param start_pos number? optional index where to start to search, default value is 1
function TcpSocket:Search(str, start_pos) end

sock = TcpSocket:New()
