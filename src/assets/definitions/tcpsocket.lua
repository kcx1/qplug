---@meta
---@diagnostic disable: missing-return, unused-local

---The TcpSocket object allows Q-SYS cores to make client TCP/IP connections to devices on the network.
TcpSocket = {
	---Time, in seconds, to wait for data to be available on socket before raising an Error through the EventHandler.
	ReadTimeout = 0,
	---Time, in seconds, to wait for data write to complete before raising an Error through the EventHandler.
	WriteTimeout = 0,
	---Time in seconds to wait before attempting to reconnect. 0 disables automatic reconnect.
	ReconnectTimeout = 0,
	---Returns true if socket is connected
	IsConnected = false,
	---Amount of data in buffer, in bytes
	BufferLength = 0,
	---The socket has connected to the remote host callback
	Connected = nil,
	---The socket is attempting to reconnect to the remote host callback
	Reconnect = nil,
	---There is new data available in the socket buffer callback
	Data = nil,
	---The socket was closed by the remote host callback
	Closed = nil,
	---The socket was closed due to error.
	---The error argument to the EventHandler will have more information, which can be displayed if a variable was created to catch the error message.
	Error = nil,
	---A read or write timeout was triggered and the socket was closed.
	Timeout = nil,
	---END OF LINE Table values
	EOL = {
		Any = "",
		CrLf = "\r\n" or "\n",
		CrLfStrict = "\r\n",
		Lf = "\n",
		Null = "\x00",
		Custom = "",
	},
}

---Creates a new TcpSocket instance.
function TcpSocket:New() end

---Attempts to connect to the specified ip/host name and port, 'ipAddress/hostname' is string, 'port' is integer
---ip_address : the ip address or host name to connect to
---port : control port
function TcpSocket:Connect(ip_address, port) end

---Disconnects the socket.
function TcpSocket:Disconnect() end

---Writes specified data to the socket. Raises error if socket is not connected.
---data : the data to write to the socket
function TcpSocket:Write(data) end

---Attempts to read up to 'length' bytes from socket. These bytes are removed from the buffer, leaving any remaining bytes beyond the 'length' specified. 'length' is positive integer.
---length : number of bytes to read
function TcpSocket:Read(length) end

---Attempts to read a 'line' from the socket buffer. 'EOL' is defined in the table below. '<custom>' is an optional string only used by EOL.Custom.
---EOL : see associated table for values
---custom : if EOL.Custom is used this variable se the custom value
function TcpSocket:ReadLine(EOL, custom) end

---Searches the socket buffer for string 'str' (starting at integer 'start_pos') and returns the index of where 'str' is found. 'start_pos' defaults to 1.
---str : the string/pattern to find
---start_pos : optional index where to start to search, default value is 1
function TcpSocket:Search(str, start_pos) end
