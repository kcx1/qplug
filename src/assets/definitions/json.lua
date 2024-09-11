---@meta
---@diagnostic disable: missing-return, unused-local

---Use the JSON module to encode and decode Lua tables to and from JSON strings.
---For faster performance, and to avoid raising execution count errors with large amounts of data, use RapidJSON instead.
---@module 'json'
local json

---Returns the Lua object JSON encoded into a string.
---@param lua_object any lua object
---@return json_string
function json.encode(lua_object) end

---@alias json_string string

---Decodes the JSON encoded data structure, and returns a Lua object with the appropriate data.
---@param json_string json_string
---@return table lua_object
function json.decode(json_string) end

---Returns a unique value that will be encoded as a null in a JSON encoding.
function json.null() end
