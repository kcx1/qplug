---@meta
---@diagnostic disable: missing-return, unused-local

---Similar to the E-mailer component, the Email library allows creation of emails directly from a Lua script.
---Note: Messages can be a maximum of 16 KB in size.
---@class Email
Email = {}

---@class EmailSendTable
---@field From string
---@field Subject string
---@field Body string
---@field To [string]
---@field CC [string]
---@field Server string
---@field Username string # If a 'Username' is not specified, it will be taken from the 'From' field.
---@field Password string
---@package
EmailSendTable = {}

---function to call with status, signature is 'function( table, error ),' where 'table' is the table passed into Send and 'error' is a string (if error occurred) or nil.
---@fun(table, error)
---@param table EmailSendTable
---@param error string | nil
EmailSendTable.EventHandler = function(table, error) end

---Send an email using specified parameters.
---@param table EmailSendTable A table containing the details of the email to be sent. Use the EmailSendTable object as helper
function Email.Send(table) end
