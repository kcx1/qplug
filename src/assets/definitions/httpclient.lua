---@meta
---@diagnostic disable: missing-return, unused-local

---Use the methods to add URL references to your Lua script.
---Transfer data over a secure HTTP connection, or encode and decode a URL, parameters, and string data into a valid ASCII format without spaces.
---@class HttpClient
HttpClient = {}

---@enum AuthMode
---| "any" # Default. Automatically selects the most secure mode
---| "basic" # HTTP Basic authentication. The only method that is in widespread use and supported virtually everywhere. This sends the user name and password over the network in plain text, easily captured by others.
---| "digest" # HTTP Digest authentication. Digest authentication is defined in RFC 2617 and is a more secure way to do authentication over public networks than the Basic method.

---@class HttpDataTable
---@field Url string
---@field Headers table
---@field User string
---@field Password string
---@field Auth AuthMode
---@field Timeout number
---@package
HttpDataTable = {}

---Extended HttpDataTable to include http method
---@class HttpUploadTable: HttpDataTable
---@field Method "GET" | "POST" | "PUT" | "DELETE"

---EventHandler to call with status.
---Signature is function( table, code, data, error, headers ). 'code' is the http return code (200 is good).
---@param table HttpDataTable
---@param code number # 200 is success
---@param data any
---@param error any
---@param headers table
HttpDataTable.EventHandler = function(table, code, data, error, headers) end

---A comma-separated list of parameters containing, at minimum, a URL name. Optionally specify a port, path, and query.
---@class UrlTable
---@field Host string
---@field Port 80 | 443 | number
---@field Path string
---@field Query string

---Specify a URL from which to download data.
---@param table HttpDataTable A comma-separated list of parameters, use the HttpDataTable  as helper for parameters
function HttpClient.Download(table) end

---Specify a URL to which to upload data.
---@param table HttpUploadTable A comma-separated list of parameters, use the HttpUploadTable as helper for parameters
function HttpClient.Upload(table) end

---Combine URL components into a complete encoded URL string.
---@param table UrlTable A comma-separated list of parameters containing, at minimum, a URL name. See UrlTable as helper
function HttpClient.CreateUrl(table) end

---Specify a comma-separated list of parameters to encode.
---The output are the encoded parameter in HTML, example: { ["name with space"] = "blue" } will return: 'name%20with%20space=blue'
---@param table table<string, string | number >
---@return string encoded_parameters
function HttpClient.EncodeParams(table) end

---Specify a string to encode.
---@param string string The string to encode. Example: 'Hello World' will return Hello%20World
---@return string encoded_string
function HttpClient.EncodeString(string) end

---Specify an encoded string to decode.
---@param string string The encoded string to decode. Example: 'Hello%20World' will return 'Hello World'
---@return string decoded_string
function HttpClient.DecodeString(string) end

---Retrieves data from the URL specified in <table> using the GET request method
---@param table HttpDataTable
function HttpClient.Get(table) end

---Transfers data to the URL specified in <table> using the PUT request method.
---@param table HttpDataTable
function HttpClient.Put(table) end

---Transfers data to the URL specified in <table> using the POST request method.
---@param table HttpDataTable
function HttpClient.Post(table) end

---Modifies data at the URL specified in <table> using the PATCH request method.
---@param table HttpDataTable
function HttpClient.Patch(table) end

---Modifies data at the URL specified in <table> using the DELETE request method.
---@param table HttpDataTable
function HttpClient.Delete(table) end
