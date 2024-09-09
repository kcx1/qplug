---@meta
---@diagnostic disable: missing-return, unused-local

---The TcpServer object allows Q-SYS cores to accept client TCP/IP connections from devices on the network.
TcpSocketServer = {}

---Creates a new TcpServer instance
function TcpSocketServer:New() end

---Open a port and listen for incoming connection
---port : port number for listen to incoming connection
function TcpSocketServer:Listen(port) end

---Close the listeing port and disconnect all the connected clients
function TcpSocketServer:Close() end

---Function called on any incoming socket event.
function TcpSocketServer.EventHandler() end
