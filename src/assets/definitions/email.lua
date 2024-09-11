---@meta
---@diagnostic disable: missing-return, unused-local

---Similar to the E-mailer component, the Email library allows creation of emails directly from a Lua script.
---Note: Messages can be a maximum of 16 KB in size.
---# EXAMPLE:
--- ```lua
--- email = Email
---
--- email.To = { "Me@aol.com" }
--- email.From = "Me@aol.com"
--- email.Subject = "Test"
--- email.Body = "Test"
--- email.Server = "aol.com"
--- email.Username = "Me"
--- email.Password = "password"
--- email.CC = { "you@aol.com" }
--- email.EventHandler = function(t, err)
--- 	if err then
--- 		print("ERROR", err)
--- 	else
--- 		print("ALL GOOD!")
--- 	end
--- end
--
--- email:Send()
--- ```
---@class Email
---@field From string
---@field Subject string
---@field Body string
---@field To [string]
---@field CC [string]
---@field Server string
---@field Username string # If a 'Username' is not specified, it will be taken from the 'From' field.
---@field Password string
---@field EventHandler fun(table: Email, error: string | nil) # function to call with status, signature is 'function( table, error ),' where 'table' is the table passed into Send and 'error' is a string (if error occurred) or nil.
Email = {}

---Send an email using specified parameters.
---@param table Email # A table containing the details of the email to be sent. Use the EmailSendTable object as helper
function Email.Send(table) end
