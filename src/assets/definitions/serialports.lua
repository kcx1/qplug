---@meta
---@diagnostic disable: missing-return, unused-local

---@enum Parity
---| "N" # None
---| "E" # Even
---| "O" # Odd
---| "M" # Mark
---| "S" # Space

---### NOTE: This should not be used on it's own.
---
---@class SerialPort
---@field IsOpen boolean Returns true if port is connected, is readonly
---@field BufferLength number Number of bytes of data in buffer
---@field EOL EndOfLine This table contains pre-defined END OF LINE values
---@field Events Events[]
---@field EventHandler fun(port: number, event: Events) Function called on any serial event.
---@field Connected fun() Assign a function which is called upon connection to the serial port
---@field Reconnect fun() Assign a function which is called when socket is attempting to reconnect to the serial port
---@field Data fun(data: string) Assign a function which is called when there is new data available in the serial buffer
---@field Closed fun() Assign a function which is called when the serial port is closed
---@field Error fun(error: string) Assign a function which is called when the serial port is closed due to error. The error argument in the function will contain more information, which can be displayed if a variable was created to catch the error message.
---@field Timeout fun() Assign a function which is called when a read or write timeout is triggered and the serial port was closed.
SerialPort = {}

---Serial port communication is supported via the RS-232 ports on some Q-SYS devices.
---You can use a scripting component, including Control Script and Block Controller, to create a serial port connection in Q-SYS.
---@type SerialPort[]
SerialPorts = {}

---Attempts to open the serial port with the specified baud rate (up to 230400 bits per second)
---@param self SerialPort
---@param baudRate integer the baud rate of the port up to 230400 b/sec
---@param dataBits integer? optional: 7, 8. Default = 8.
---@param parity  Parity? optional with dataBits: N (None), E (Even), O (Odd), M (Mark), S (Space)
function SerialPort:Open(baudRate, dataBits, parity) end

---Closes the serial port
---@param self SerialPort
function SerialPort:Close() end

---Writes specified data to the serial port. Raises error if port is not open.
---@param self SerialPort
---@param data string the data to write
function SerialPort:Write(data) end

---Attempts to read up the 'length' bytes from serial buffer. Data is removed from serial buffer.
---@param self SerialPort
---@param length number length value to read
---@return string data read from serial buffer
function SerialPort:Read(length) end

---Attempts to read a 'line' from the serial buffer.
---@param self SerialPort
---@param EOL EndOfLine is defined as in the SerialPorts.EOL table.
---@param custom string? is an optional string only used by EOL.Custom.
---@return string data read up to EOL or custom string
function SerialPort:RealLine(EOL, custom) end

---Searches the serial buffer for string 'str'
---@param self SerialPort
---@param string string the string to find in the buffer
---@param start_pos number? optional start index where the search starts
function SerialPort:Search(string, start_pos) end
