---@meta
---@diagnostic disable: missing-return, unused-local

---Use the RapidJSON module to encode and decode large documents quickly.
---It is similar to the standard JSON module, but can handle large amounts of data without risk of raising execution count errors.
---To use the RapidJSON module, add the following line to your script: rapidjson = require("rapidjson")
---@module 'rapidjson'
local rapidjson

---@class JsonOptions
---@field pretty boolean Set true to make output string to be pretty formated. Default is false.
---@field sort_keys boolean Set true to make JSON object keys be sorted. Default is false.
---@field empty_table_as_array boolean Set true to make empty table encode as JSON array. Default is false.

---encode : Encode a Lua table to JSON string. JSON object keys are sorted by this function.
---@param value any
---@param option JsonOptions? A optional table contains the following field:
---@return json_string
function rapidjson.encode(value, option) end

---decode : Decode JSON to a Lua table.
---@param value string A JSON value string to be decoded.
---@return table<string | number, any>
function rapidjson.decode(value) end

---load : Load JSON file into Lua table.
---@param filename string The JSON file to be loaded.
---@return table<string | number, any>
function rapidjson.load(filename) end

---dump : Dump a Lua value to a JSON file.
---@param value any Same as in rapidjson.encode().
---@param filename string The file path string where to save the rapidjson string.
---@param option JsonOptions? A optional table contains the following field:
function rapidjson.dump(value, filename, option) end

---null : Placeholder for null values in rapidjson.
function rapidjson.null() end

---Create a new empty table that has the metatable field __jsontype set as 'object' so that the encode and dump function will encode it as a JSON object.
---@param table table? Optional table to set the metatable with meta field __jsontype set as 'object'.
function rapidjson.object(table) end

---Same as rapidjson.object(), except the metatable field __jsontype is set as 'array'. The encode and dump function will encode it as JSON array.
function rapidjson.array() end

---Ai Generated
---@class SchemaDocument
---@field schema table A Lua table representation of a JSON schema.

---Ai Generated
---Creates a SchemaDocument from Document or a Lua table or a string contains a JSON schema.
---@param doc? SchemaDocument|table|string The the JSON schema stored in rapidjson.Document object, or a Lua table representation of a JSON schema, or a string contains a JSON schema.
function rapidjson.SchemaDocument(doc) end

---Ai Generated
---@class SchemaValidator
---SchemaValidator is used to validate a document against a json-schema.
---@field schema table A Lua table representation of the JSON schema.
SchemaValidator = {}
---Creates a SchemaValidator from a Schema Document
---@param sd? SchemaDocument The SchemaDocument to create the validator. SchemaDocument can be shared by schema validators.
function rapidjson.SchemaValidator(sd) end

---Ai Generated
---Validates a JSON document.
---@param value table A JSON document to be validated against the schema stored inside the validator.
---@return boolean valid Whether the document is valid and an error message if invalid (or nil if no message).
function SchemaValidator:validate(value) end
