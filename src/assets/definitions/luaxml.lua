---@meta
---@diagnostic disable: missing-return, unused-local

---@alias iterator fun()

---@module 'LuaXML'
---@class LuaXML
local LuaXML

---AI Generated
---Creates a new LuaXML object.
---
---Example:
---```lua
----- create a new XML object and set its tag to "root"
---local x = xml.new("root")
----- append a new subordinate XML object, set its tag to "child", and its content to 123
---x:append("child")[1] = 123
---print(x)
---```
---@param arg string?
---@return LuaXML
function LuaXML.new(arg) end

---AI Generated
---Appends a new subordinate LuaXML object to an existing one, optionally sets tag.
---
---Example:
---```lua
----- append a new subordinate XML object, set its tag to "child", and its content to 123
---x:append("child")[1] = 123
---print(x)
---```
---@param tag string?
---@return table
function LuaXML.append(tag) end

---AI Generated
---Iterate subelements (“XML children”) as key – value pairs.
---
---Example:
---```lua
----- iterate over the XML object's children and print their tags and values
---for k, v in x:children() do
---    print(k, v)
---end
---```
---@return iterator
function LuaXML.children() end

---AI Generated
---Converts a string from XML encoding.
---
---Example:
---```lua
----- decode an XML encoded string
---local xmlstr = "<xml>test</xml>"
---local decoded = xml.decode(xmlstr)
---print(decoded) -- prints "test"
---```
---@param str string?
---@return string?
function LuaXML.decode(str) end

---AI Generated
---Converts a string to XML encoding.
---
---Example:
---```lua
----- encode an string into XML format
---local xmlstr = xml.encode("test")
---print(xmlstr) -- prints "<xml>test</xml>"
---```
---@param str string?
---@return string?
function LuaXML.encode(str) end

---AI Generated
---Iterates a LuaXML object, invoking a callback function for all matching (sub)elements.
---
---Example:
---```lua
----- iterate over the XML object and print its tags and values
---x:interate(function(k, v)
---    print(k, v)
---end)
---```
---@param cb function?
---@return nil
function LuaXML.iterate(cb) end

---AI Generated
---Match XML entity against given (optional) criteria.
---
---Example:
---```lua
----- match an XML element with a specific tag and attribute value
---local x = xml.match(x, "child", "attr", 123)
---print(x) -- prints the matching XML object
---```
---@param var table | LuaXML
---@param tag string?
---@param key string?
---@param value any?
---@return LuaXML?
function LuaXML.match(var, tag, key, value) end

---AI Generated
---Loads XML data from a file and returns it as table.
---
---Example:
---```lua
----- load an XML file into a LuaXML object
---local x = xml.load("example.xml")
---print(x)
---```
---@param filename string?
---@return table?
function LuaXML.load(filename) end

---AI Generated
---Saves a Lua var as XML file.
---
---Example:
---```lua
----- save a Lua variable to an XML file
---xml.save(xml.eval("<root><child>123</child></root>"), "example.xml")
---```
---@param var any?
---@param filename string?
---@return nil
function LuaXML.save(var, filename) end

---AI Generated
---Converts an XML string to a Lua table.
---
---Example:
---```lua
----- convert an XML string into a Lua table
---local x = xml.eval("<root><child>123</child></root>")
---print(x)
---```
---@param xmlstring string?
---@return table?
function LuaXML.eval(xmlstring) end

---AI Generated
---Sets or returns tag of a LuaXML object.
---
---Example:
---```lua
----- set the tag of an XML object and print it
---local x = xml.tag("root")
---print(x)
---```
---@param tag string?
---@return string?
function LuaXML.tag(tag) end

---AI Generated
---Converts any Lua var to an xml string.
---
---Example:
---```lua
----- convert a Lua variable into an XML string
---local xmlstr = xml.str(xml.eval("<root><child>123</child></root>"))
---print(xmlstr)
---```
---@param v any?
---@param indent number?
---@return string?
function LuaXML.str(v, indent) end

---AI Generated
---Iterate over the XML object's children and print their tags and values
---
---Example:
---```lua
----- iterate over the XML object's children and print their tags and values
---for k, v in x:children() do
---    print(k, v)
---end
---```
---@return iterator
function LuaXML.children() end

---AI Generated
---Registers a LuaXML function.
---
---Example:
---```lua
----- register an XML function to load an XML file into a LuaXML object
---xml.register_function("load_xml", xml.load)
---```
---@param name string?
---@param func function?
---@return nil
function LuaXML.register_function(name, func) end
