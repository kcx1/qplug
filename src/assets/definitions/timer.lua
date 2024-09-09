---@meta
---@diagnostic disable: missing-return, unused-local

---The Timer object is used to create delays or trigger events after a defined elapsed time.  It should be used instead of Lua's native delay and time functions.
Timer = {}

---Creates a new timer object
function Timer.New() end

---Handle the timer event, is used when the time runs out
function Timer.EventHandler() end

---Starts the timer
---time : time interval in SECONDS, when the time expire the EventHandler is executed
function Timer:Start(time) end

---Stops the timer
function Timer:Stop() end

---Supply a named function to call after a given delay in milliseconds.
---function_pass : function to execute after time
---time : time interval in MILLISECONDS, when the time expire the function_pass is executed
function Timer.CallAfter(function_pass, time) end
