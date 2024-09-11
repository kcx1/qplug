---@meta
---@diagnostic disable: missing-return, unused-local

---The TcpServer object allows Q-SYS cores to accept client TCP/IP connections from devices on the network.
---
---Example:
---```lua
---server = TcpSocketServer.New()
---
----- table to store connected client sockets
----- this is required so the sockets don't
----- get garbage collected since there aren't
----- any other references to them in the script
---sockets = {}
---
---function RemoveSocketFromTable(sock)
---  for k,v in pairs(sockets) do
---    if v == sock then
---      table.remove(sockets, k)
---      return
---    end
---  end
---end
---
---
---function SocketHandler(sock, event) -- the arguments for this EventHandler are documented in the EventHandler definition of TcpSocket Properties
---  print( "TCP Socket Event: "..event )
---  if event == TcpSocket.Events.Data then
---    print( sock, sock:Read(sock.BufferLength) )
---  elseif event == TcpSocket.Events.Closed or
---         event == TcpSocket.Events.Error or
---         event == TcpSocket.Events.Timeout then
---    -- remove reference of socket from table so
---    -- it's available for garbage collection
---    RemoveSocketFromTable(sock)
---  end
---end
---
---server.EventHandler = function(SocketInstance) -- the properties of this socket instance are those of the TcpSocket library
---  SocketInstance.ReadTimeout = 10
---  print( "Got connect", SocketInstance )
---  table.insert(sockets, SocketInstance)
---  SocketInstance.EventHandler = SocketHandler
---end
---
---server:Listen(1720) -- This listen port is opened on all network interfaces
---```
---@class TcpSocketServer
---@field EventHandler fun() Function called on any incoming socket event
TcpSocketServer = {}

---Creates a new TcpServer instance
---@param self TcpSocketServer
function TcpSocketServer:New() end

---Open a port and listen for incoming connection
---@param self TcpSocketServer
---@param port number port number for listen to incoming connection
function TcpSocketServer:Listen(port) end

---Close the listeing port and disconnect all the connected clients
---@param self TcpSocketServer
function TcpSocketServer:Close() end
