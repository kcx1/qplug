---@meta
---@diagnostic disable: missing-return, unused-local

---The WebSocket Protocol enables two-way communication between a client running code in a controlled environment to a remote host that has opted-in to communications from that code. The security model used for this is the origin based security model commonly used by web browsers. The protocol consists of an opening handshake followed by basic message framing, layered over TCP. The goal of this technology is to provide a mechanism for browser-based applications that need two-way communication with servers that does not rely on opening multiple HTTP connections.
---
---Example:
---```lua
---ws = WebSocket.New()
---pingTimer = Timer.New()
---
---pingTimer.EventHandler = function()
---  ws:Ping()
---end
---
---ws.Connected = function(ws)
---  print("Connected")
---
---  -- send ping every 10 seconds
---  pingTimer:Start(10)
---
---  ws:Write("Hey!")
---end
---
---ws.Error = function( ws, err)
---  pingTimer:Stop()
---  print("Error", err)
---end
---
---ws.Closed = function(ws)
---  pingTimer:Stop()
---  print("closed")
---end
---
---ws.Data = function( ws, data)
---  print("Data", data)
---end
---
---ws:Connect("wss", "192.168.0.201", "/qrc", 443)
---```
---@class WebSocket
---@field New fun(): WebSocket Creates a new WebSocket instance.
---@field Connect fun(self: WebSocket, protocol: "ws" | "wss", host: string, url: string, port:number, sub_protocol: string?,  headers: table<string, string>?): WebSocket
---@field Write fun(self: WebSocket, data: string, isBinary: boolean?)
---@field Close fun(self: WebSocket) Closes the WebSocket connection.
---@field Connected fun(self: WebSocket) Callback function for when WebSocket is connected.
---@field Data fun(self: WebSocket, data: string) Callback function for when WebSocket receives data.
---@field Error fun(self: WebSocket, error: string) Callback function for when WebSocket errors.
---@field Closed fun(self: WebSocket) Callback function for when WebSocket is closed.
---@field Ping fun(self: WebSocket) Callback function for when WebSocket pings.
WebSocket = {}

---Connect to host with given protocol (ws or wss) to URL and port with optional sub-protocol.
---
---Example:
---```lua
---ws = WebSocket.New()
---protocol = "wss"
---host = "example.com"
---url = "/my-websocket-endpoint"
---port = 443
---sub_protocol = nil  -- No specific sub-protocol
---headers = {
---    ["Authorization"] = "Bearer my-auth-token",
---    ["Custom-Header"] = "SomeValue"
---}
---
---ws:Connect(protocol, host, url, port, sub_protocol, headers)
---```
---@param protocol "ws"|"wss"
---@param host string
---@param url string
---@param port number
---@param sub_protocol string
---@param headers table<string, string>
function WebSocket:Connect(protocol, host, url, port, sub_protocol, headers) end

---Sends message to server.
---
---Example:
---```lua
---ws:Write("Hello World!")
---```
---@param data string
---@param isBinary boolean?
function WebSocket:Write(data, isBinary) end
