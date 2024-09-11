---@meta
---@diagnostic disable: missing-return, unused-local

---Use the SNMP library in Lua to monitor OIDs obtained from an SNMP-enabled device's MIB file.
---Note: In the SNMP model, Q-SYS acts as the SNMP Manager, while the device you intend to monitor runs an SNMP Agent that allows for monitoring.
---
---Example:
---```lua
---QueryList = {
---  '.1.3.6.1.4.1.1536.1.1.2.1.0',
---  '.1.3.6.1.4.1.1536.1.1.2.2.0',
---  '.1.3.6.1.4.1.1536.1.1.2.3.0',
---
---  '.1.3.6.1.4.1.1536.1.2.2.1.1.0',
---
---  '1.3.6.1.4.1.1536.1.2.2.2.1.1.1',
---  '1.3.6.1.4.1.1536.1.2.2.2.1.1.2',
---  '1.3.6.1.4.1.1536.1.2.2.2.1.1.3',
---  '1.3.6.1.4.1.1536.1.2.2.2.1.1.4',
---  '1.3.6.1.4.1.1536.1.2.2.2.1.1.5',
---  '1.3.6.1.4.1.1536.1.2.2.2.1.1.6',
---  '1.3.6.1.4.1.1536.1.2.2.2.1.1.7',
---
---  '1.3.6.1.4.1.1536.1.2.2.3.1.1.1',
---  '1.3.6.1.4.1.1536.1.2.2.3.1.1.2',
---  '1.3.6.1.4.1.1536.1.2.2.3.1.1.3',
---  '1.3.6.1.4.1.1536.1.2.2.3.1.1.4',
---  '1.3.6.1.4.1.1536.1.2.2.3.1.1.5',
---}
---listPtr = 1
---
---snmp_session = SNMPSession.New(SNMP.SessionType.v3) -- Create a new SNMP session.
---snmp_session:setHostName('192.168.1.100') -- Type in the Host Name
---snmp_session:setAuthType(SNMP.AuthType.AuthNo)
---snmp_session:setAuthProt(SNMP.AuthProtocol.NoAuth)
---snmp_session:setPrivProt(SNMP.PrivProtocol.NoAuth)
---snmp_session:setUserName('defaultuser')
---snmp_session:setPassPhrase('')
---snmp_session:setPrivPassPhrase('')
---
---snmp_session.EventHandler = function(response)
---  print("OID Start")
---  for k,v in pairs(response) do
---    print(k,v)
---  end
---  print("OID End")
---end
---
---snmp_session.ErrorHandler = function(response)
---  print("Err Start")
---  for k,v in pairs(response) do
---    print(k,v)
---  end
---  print("Err End")
---end
---
---snmp_session:startSession()
---
---function myCallback(dataout)
---  print("Data Start")
---  for k,v in pairs(dataout) do
---    print(k,v)
---  end
---  print("Data End")
---end
---
---function requestSNMP()
---  print("Send",listPtr,QueryList[listPtr])
---  snmp_session:getRequest(QueryList[listPtr], myCallback)
---  if (listPtr == #QueryList) then
---    listPtr = 1
---  else
---    lisPtr = listPtr + 1
---  end
---end
---
---
---OIDpoll_timer = Timer.New()
---OIDpoll_timer.EventHandler = requestSNMP
---OIDpoll_timer:Start(5)
---```
---@class SNMP
---@field AuthType SNMP.AuthType
---@field AuthProtocol SNMP.AuthProtocol
---@field PrivProtocol SNMP.PrivProtocol
---@field SNMPDataType SNMP.SNMPDataType
---@field New fun(session_type:SNMP.SessionType):SNMP Create a new SNMP session.
---@field setHostName fun(self, hostname:string) Specify the host to which to connect.
---@field setAuthType fun(self, type:SNMP.AuthType) For SNMP v3 only, set the authorization type for the session.
---@field setAuthProt fun(self, type:SNMP.AuthProtocol) For SNMP v3 only, set the authorization protocol for the session.
---@field setUserName fun(self, username:string) For SNMP v3 only, set the user name for the session.
---@field setPassPhrase fun(self, passphrase:string) For SNMP v3 only, set the authorization pass phrase for the session.
---@field setPrivPassPhrase fun(self, privpass:string) For SNMP v3 only, set the privacy pass phrase for the session.
---@field setCommunity fun(self, community:string) For SNMP v2 sessions only, set the community name for the session.
---@field startSession fun(self) Initiate the connection to the corresponding session.
---@field getRequest fun(self, oid:string, callback:fun(response:table<string, string>))
---@field setRequest fun(self, oid:string, new_value:string, type:SNMP.SNMPDataType, callback:fun(response:table<string, string>))
---@field EventHandler fun(SNMP.Response) Assign the Lua callback for successful SNMP events.
---@field ErrorHandler fun(SNMP.ErrorResponse) Assign the Lua callback for unsuccessful SNMP events.
SNMP = {}

---@class SNMP.SessionType
---@field v2c string
---@field v3 string

---@class SNMP.AuthType
---@field NoAuth string No authorization type and no privacy type.
---@field AuthNoPriv string Authorization with no privacy.
---@field AuthPriv string Both authorization and privacy.

---@class SNMP.AuthProtocol
---@field NoAuth string Disable the authorization protocol.
---@field MD5 string Enable the MD5 authorization protocol.
---@field SHA string Enable the SHA authorization protocol.

---@class SNMP.PrivProtocol
---@field NoPriv string Disable the privacy protocol.
---@field AES string Enable the AES privacy protocol.
---@field DES string Enable the DES privacy protocol.

---@class SNMP.SNMPDataType
---@field unknown string
---@field integer32 string
---@field unsigned32 string
---@field unsigned_integer32 string
---@field timeticks string
---@field ip_address string
---@field object_id string
---@field octet_string string
---@field hex string
---@field decimal string
---@field bit_string string
---@field integer64 string
---@field unsigned64 string
---@field float32 string
---@field double64 string

---@class SNMP.Response
---@field RequestID integer The request ID for bookkeeping purposes.
---@field OID string The object ID for the response.
---@field Value string the string representation for the current state of the object ID
---@field Hostname string The host name for the response

---@enum Error
---| "Time Out"
---| "Send Failed"
---| "Sec Error"
---| "SNMP Error"
---| "No Such Instance"
---| "No Such Object"

---@class SNMP.ErrorResponse: SNMP.Response
---@field Error Error

---Request an object ID (OID) and pass the response to a Lua callback.
---
---Example:
---```lua
---function myCallback(dataout)
---    for k,v in pairs(dataout) do
---      print(k,v)
---    end
---end
---
---snmp_session:getRequest(".1.3.6.1.2.1.1.3.0", myCallback)
---```
---@param oid string object ID
---@param callback fun(response:table<string, string>) The Lua callback to which to pass the response.
function SNMP:getRequest(oid, callback) end

---Set a new value for a specified OID.
---
---Example:
---```lua
---function myCallback(dataout)
---    for k,v in pairs(dataout) do
---      print(k,v)
---    end
---end
---
---snmp_session:setRequest(".1.3.6.1.2.1.1.5.0", "new_switch_name", SNMP.SNMPDataType.octet_string, myCallback)
---```
---@param oid string object ID
---@param new_value string The new value to which to set the specified object ID.
---@param type SNMP.SNMPDataType For the data type,
---@see SNMPDataType
---@param callback fun(response:table<string, string>) The Lua callback to which to pass the response.
function SNMP:setRequest(oid, new_value, type, callback) end
