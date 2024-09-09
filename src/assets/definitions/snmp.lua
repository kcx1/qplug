---@meta
---@diagnostic disable: missing-return, unused-local

---Use the SNMP library in Lua to monitor OIDs obtained from an SNMP-enabled device's MIB file.
---Note: In the SNMP model, Q-SYS acts as the SNMP Manager, while the device you intend to monitor runs an SNMP Agent that allows for monitoring.
SNMP = {
	AuthType = {
		---No authorization type and no privacy type.
		NoAuth = "NoAuth",
		--- Authorization with no privacy.
		AuthNoPriv = "AuthNoPriv",
		---Both authorization and privacy.
		AuthPriv = "AuthPriv",
	},

	AuthProtocol = {
		---Disable the authorization protocol.
		NoAuth = "NoAuth",
		---Enable the MD5 authorization protocol.
		MD5 = "MD5",
		---Enable the SHA authorization protocol.
		SHA = "SHA",
	},

	PrivProtocol = {
		---Disable the privacy protocol.
		NoPriv = "NoPriv",
		---Enable the AES privacy protocol.
		AES = "AES",
		--- Enable the DES privacy protocol.
		DES = "DES",
	},

	SNMPDataType = {
		unknown = "unknown",
		integer32 = "integer32",
		unsigned32 = "unsigned32",
		unsigned_integer32 = "unsigned_integer32",
		timeticks = "timeticks",
		ip_address = "ip_address",
		object_id = "object_id",
		octet_string = "octet_string",
		hex = "hex",
		decimal = "decimal",
		bit_string = "bit_string",
		integer64 = "integer64",
		unsigned64 = "unsigned64",
		float32 = "float32",
		double64 = "double64",
	},
}

---Create a new SNMP session.
function SNMP.New() end

---Specify the host to which to connect.
---hostname : The target host name.
function SNMP:setHostName(hostname) end

---For SNMP v3 only, set the authorization type for the session.
---type : see the AuthType for details
function SNMP:setAuthType(type) end

---For SNMP v3 only, set the authorization protocol for the session.
---type : see the AuthProtocol for details
function SNMP:setAuthProt(type) end

---For SNMP v3 only, set the user name for the session.
---username : The user name for the session.
function SNMP:setUserName(username) end

---For SNMP v3 only, set the authorization pass phrase for the session.
---The pass phrase for the corresponding user name.
function SNMP:setPassPhrase(passphrase) end

---For SNMP v3 only, set the privacy pass phrase for the session.
---privpass : The privacy pass phrase for the session.
function SNMP:setPrivPassPhrase(privpass) end

---For SNMP v2 sessions only, set the community name for the session.
---community : The community name for the session.
function SNMP:setCommunity(community) end

---Initiate the connection to the corresponding session.
function SNMP:startSession() end

---Request an object ID (OID) and pass the response to a Lua callback.
---oid : object ID
---callback : The Lua callback to which to pass the response.
function SNMP:getRequest(oid, callback) end

---Set a new value for a specified OID.
---oid : object ID
---new_value : The new value to which to set the specified object ID.
---type : the data type, see SNMPDataType
---callback : The Lua callback to which to pass the response.
function SNMP:setRequest(oid, new_value, type, callback) end

---Assign the Lua callback for successful SNMP events.
function SNMP.EventHandler() end

---Assign the Lua callback for unsuccessful SNMP events.
function SNMP.ErrorHandler() end
