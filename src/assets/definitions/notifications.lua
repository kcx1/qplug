---@meta
---@diagnostic disable: missing-return, unused-local

---Use the following methods to subscribe to a notification, publish a notification with specified data, and unsubscribe from a notification.
---This allows scripts running within the same Core to communicate with each other using control wiring or Component.
---@class Notifications
---@field noteid string The token to use when unsubscribing from a notification.
Notifications = {}

---Subscribe to a notification with a given name.
---@param name string
---@param func function(name, data) The callback to call when the named notification is triggered. The signature is function( name, data ).
function Notifications.Subscribe(name, func) end

---Publish a named notification with given data. The data can be either a Lua table or a string.
---Note: You cannot call the Notifications.Publish() method inside a Notifications.Subscribe() callback. This will raise a Lua error.
---@param name string The name of the notification.
---@param data string | table the notification data
function Notifications.Publish(name, data) end

---Unsubscribe from a notification with the specified noteid.
---@param noteid string The name of the notification.
function Notifications.Unsubscribe(noteid) end
