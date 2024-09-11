---@meta
---@diagnostic disable: missing-return, unused-local

---The Timer object is used to create delays or trigger events after a defined elapsed time.  It should be used instead of Lua's native delay and time functions.
---
---Example:
---```lua
---timer1 = Timer.New()
---timer2 = Timer.New()
---
---function timerFunc(timer)
---    if timer == timer1 then
---        print( "timer 1!" )
---    elseif timer == timer2 then
---        print ( "timer 2" )
---    end
---end
---
---timer1.EventHandler = timerFunc
---timer2.EventHandler = timerFunc
---
---timer1:Start(1)
---timer2:Start(2)
---```
---@class Timer
---@field New fun(): Timer Creates a new timer object
---@field EventHandler fun() Handle the timer event, is used when the time runs out
---@field IsRunning fun(): boolean Check if the timer is running
Timer = {}

---Starts the timer
---@param time number time interval in SECONDS, when the time expire the EventHandler is executed
function Timer:Start(time) end

---Stops the timer
function Timer:Stop() end

---Supply a named function to call after a given delay in milliseconds.
---@param function_pass fun() to execute after time
---@param time number time interval in MILLISECONDS, when the time expire the function_pass is executed
function Timer.CallAfter(function_pass, time) end
