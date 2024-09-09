---@meta
---@diagnostic disable: missing-return, unused-local

---Serial port communication is supported via the RS-232 ports on some Q-SYS devices.
---You can use a scripting component, including Control Script and Block Controller, to create a serial port connection in Q-SYS.
SerialPorts = {
	[1] = {
		---Returns true if port is connected, is readonly
		IsOpen = false,
		---Number of bytes of data in buffer
		BufferLength = 1,
		---This table contains pre-defined END OF LINE values
		EOL = {
			Any = "",
			CrLf = "\r\n" or "\n",
			CrLfStrict = "\r\n",
			Lf = "\n",
			Null = "\x00",
			Custom = "",
		},
		Events = {
			"Connected",
			"Reconnect",
			"Data",
			"Closed",
			"Error",
			"Timeout",
		},
	},
}

---Function called on any serial event.
---port : the port where the event happen
---event : the event type
function SerialPorts.EventHandler(port, event) end

---Attempts to open the serial port with the specified baud rate (up to 230400 bits per second)
---baudRate : the baud rate of the port up to 230400 b/sec
---dataBits : dataBits - optional: 7, 8. Default = 8.
---parity : optional with dataBits: N (None), E (Even), O (Odd), M (Mark), S (Space)
function SerialPorts:Open(baudRate, dataBits, parity) end

---Closes the serial port
function SerialPorts:Close() end

---Writes specified data to the serial port. Raises error if port is not open.
---data : the data to write
function SerialPorts:Write(data) end

---Attempts to read up the 'length' bytes from serial buffer. Data is removed from serial buffer.
---length : length value to read
function SerialPorts:Read(length) end

---Attempts to read a 'line' from the serial buffer.
---EOL : is defined as in the SerialPorts.EOL table.
---custom : is an optional string only used by EOL.Custom.
function SerialPorts:RealLine(EOL, Custom) end

---Searches the serial buffer for string 'str'
---string : the string to find in the buffer
---start_pos : optional start index where the search starts
function SerialPorts:Search(string, start_pos) end

---Assign a function which is called upon connection to the serial port
function SerialPorts.Connected() end

---Assign a function which is called when socket is attempting to reconnect to the serial port
function SerialPorts.Reconnect() end

---Assign a function which is called when there is new data available in the serial buffer
function SerialPorts.Data() end

---Assign a function which is called when the serial port is closed
function SerialPorts.Closed() end

---Assign a function which is called when the serial port is closed due to error.
---The error argument in the function will contain more information, which can be displayed if a variable was created to catch the error message.
function SerialPorts.Error() end

---Assign a function which is called when a read or write timeout is triggered and the serial port was closed.
function SerialPorts.Timeout() end
