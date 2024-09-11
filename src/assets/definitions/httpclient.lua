---@meta
---@diagnostic disable: missing-return, unused-local

---@enum AuthMode
---| "any" # Default. Automatically selects the most secure mode
---| "basic" # HTTP Basic authentication. The only method that is in widespread use and supported virtually everywhere. This sends the user name and password over the network in plain text, easily captured by others.
---| "digest" # HTTP Digest authentication. Digest authentication is defined in RFC 2617 and is a more secure way to do authentication over public networks than the Basic method.

---@class HttpDataTable
---@field Url string
---@field Headers table<string, string>
---@field User string
---@field Password string
---@field Auth AuthMode
---@field Timeout number
---@field EventHandler fun(table, code, data, error, headers) # Signature is function( table, code, data, error, headers ). 'code' is the http return code (200 is good).

---Extended HttpDataTable to include http method
---@class HttpUploadTable: HttpDataTable
---@field Method "GET" | "POST" | "PUT" | "DELETE"

---A comma-separated list of parameters containing, at minimum, a URL name. Optionally specify a port, path, and query.
---@class UrlTable
---@field Host string
---@field Port 80 | 443 | number
---@field Path string
---@field Query string

---Use the methods to add URL references to your Lua script.
---Transfer data over a secure HTTP connection, or encode and decode a URL, parameters, and string data into a valid ASCII format without spaces.
---@class HttpClient
HttpClient = {}

---Specify a URL from which to download data.
---```lua
---function done(tbl, code, data, err, headers)
---  print(string.format( "HTTP response from '%s': Return Code=%i; Error=%s", tbl.Url, code, err or "None" ) )
---  print("Headers:")
---  for hName,Val in pairs(headers) do
---    if type(Val) == "table" then
---      print(string.format( "\t%s : ", hName ))
---      for k,v in pairs(Val) do
---        print(string.format( "\t\t%s", v ) )
---      end
---    else
---      print(string.format( "\t%s = %s", hName, Val ) )
---    end
---  end
---  print( "\rHTML Data: "..data )
---end
---HttpClient.Download { Url = "http://www.google.com", Headers = { ["Content-Type"] = "application/json" } , Timeout = 30, EventHandler = done }
---```
---@param table HttpDataTable A comma-separated list of parameters, use the HttpDataTable  as helper for parameters
function HttpClient.Download(table) end

---Specify a URL to which to upload data.
---
---Example:
---
---```lua
---HttpClient.Upload {
---    Url = "https://some.web.site/",
---    Method = "POST",
---    User = user,
---    Password = token,
---    Auth = "basic",
---    Headers = {
---     ["Content-Type"] = "application/json"
---    },
---    Data = qsc_json.encode(data),
---    EventHandler = function(table, code, data, error, headers)
---      print(table, code, data, error, headers)
---  }
---end
---```
---@param table HttpUploadTable A comma-separated list of parameters, use the HttpUploadTable as helper for parameters
function HttpClient.Upload(table) end

---Combine URL components into a complete encoded URL string.
---Example:
---
---```lua
---assert(HttpClient.CreateUrl( { Host =  "http://www.go.com", Path = "this/is/a/path with space", Query = { ["name with space"] = "blue", term = "hello | there" }})) == http://www.go.com/this/is/a/path%20with%20space?term=hello%20%7c%20there&name%20with%20space=blue
---```
---@param table UrlTable A comma-separated list of parameters containing, at minimum, a URL name. See UrlTable as helper
function HttpClient.CreateUrl(table) end

---Specify a comma-separated list of parameters to encode.
---The output are the encoded parameter in HTML, example: { ["name with space"] = "blue" } will return: 'name%20with%20space=blue'
---
---Example:
---```lua
---assert( HttpClient.EncodeParams( { ["name with space"] = "blue", term = "hello | there" })) == term=hello%20%7c%20there&name%20with%20space=blue
---```
---@param table table<string, string | number >
---@return string encoded_parameters
function HttpClient.EncodeParams(table) end

---Specify a string to encode.
---
---Example:
---```lua
---assert( HttpClient.EncodeString( "this is | some test")) == this%20is%20%7c%20some%20test
---```
---@param string string The string to encode. Example: 'Hello World' will return Hello%20World
---@return string encoded_string
function HttpClient.EncodeString(string) end

---Specify an encoded string to decode.
---
---Example:
---```lua
---assert( HttpClient.DecodeString("this%20is%20%7c%20some%20test")) == this is | some test
---```
---@param string string The encoded string to decode. Example: 'Hello%20World' will return 'Hello World'
---@return string decoded_string
function HttpClient.DecodeString(string) end

---Retrieves data from the URL specified in <table> using the GET request method
---
---Example:
---```lua
---function done(tbl, code, data, err, headers)
---  print(string.format( "HTTP response from '%s': Return Code=%i; Error=%s", tbl.Url, code, err or "None" ) )
---  print("Headers:")
---  for hName,Val in pairs(headers) do
---    if type(Val) == "table" then
---      print(string.format( "\t%s : ", hName ))
---      for k,v in pairs(Val) do
---        print(string.format( "\t\t%s", v ) )
---      end
---    else
---      print(string.format( "\t%s = %s", hName, Val ) )
---    end
---  end
---  print( "\rHTML Data: "..data )
---end
---HttpClient.Get { Url = "http://www.google.com", Headers = { ["Content-Type"] = "application/json" } , Timeout = 30, EventHandler = done }
---```
---@param table HttpDataTable
function HttpClient.Get(table) end

---Transfers data to the URL specified in <table> using the PUT request method.
---
---Example:
---```lua
---function done(tbl, code, d, e)
---  print( string.format("Response Code: %i\t\tErrors: %s\rData: %s",code, e or "None", d))
---end
---
---function Put()
---  url = string.format("https://posttestserver.dev/p/ms3mb6ij4bk2y5s8/post")
---  HttpClient.Put{
---    Url = url,
---    Data = "this is a test", -- This can be anything
---    Headers = {
---      ["Content-Type"] = "text/html",
---    },
---    EventHandler = done -- The function to call upon response
---  }
---end
---
---Put()
---```
---@param table HttpDataTable
function HttpClient.Put(table) end

---Transfers data to the URL specified in <table> using the POST request method.
---
---Example:
---```lua
---function done(tbl, code, d, e)
---  print( string.format("Response Code: %i\t\tErrors: %s\rData: %s",code, e or "None", d))
---end
---
---function Post()
---  url = string.format("https://posttestserver.dev/p/ms3mb6ij4bk2y5s8/post")
---  HttpClient.Post {
---    Url = url,
---    Data = "this is a test", -- This can be anything
---    Headers = {
---      ["Content-Type"] = "text/html",
---    },
---    EventHandler = done -- The function to call upon response
---  }
---end
---
---Post()
---```
---@param table HttpDataTable
function HttpClient.Post(table) end

---Modifies data at the URL specified in <table> using the PATCH request method.
---
---Example:
---```lua
---function done(tbl, code, d, e)
---  print( string.format("Response Code: %i\t\tErrors: %s\rData: %s",code, e or "None", d))
---end
---
---function Patch()
---  url = string.format("https://posttestserver.dev/p/ms3mb6ij4bk2y5s8/post")
---  HttpClient.Patch{
---    Url = url,
---    Data = "this is a test", -- This can be anything
---    Headers = {
---      ["Content-Type"] = "text/html",
---    },
---    EventHandler = done -- The function to call upon response
---  }
---end
---
---Patch()
---```
---@param table HttpDataTable
function HttpClient.Patch(table) end

---Modifies data at the URL specified in <table> using the DELETE request method.
---
---Example:
---```lua
---function done(tbl, code, d, e)
---  print( string.format("Response Code: %i\t\tErrors: %s\rData: %s",code, e or "None", d))
---end
---
---function Delete()
---  url = string.format("https://posttestserver.dev/p/ms3mb6ij4bk2y5s8/post")
---  HttpClient.Delete{
---    Url = url,
---    Headers = {
---      ["Content-Type"] = "text/html",
---    },
---    EventHandler = done -- The function to call upon response
---  }
---end
---
---Delete()
---```
---@param table HttpDataTable
function HttpClient.Delete(table) end
