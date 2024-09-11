---@meta
---@diagnostic disable: missing-return, unused-local

---@class Uci
Uci = {}

---Table that store UCI elements
---@class DialogTable
---@field Title string
---@field Message string
---@field Buttons string[]
---@field Handler fun(index: number) Index of the button selected, index start at 0

---Display a dialog in a UCI that contains a title, message, and button selection list.
---
---Example:
---```lua
---ButtonText = {
---  "Button 1 was pushed",
---  "Button 2 was pushed",
---  "Button 3 was pushed",
---}
---
---function UCIDialogHandler(choiceInt)
---  print(choiceInt,ButtonText[choiceInt+1])
---  Controls.WhichButton.String = ButtonText[choiceInt+1]
---end
---
---function ShowDialog()
---  Uci.ShowDialog(
---    "Main UCI",
---    {
---      Title = "UCI Dialog Titlebar",
---      Message = "Which button would you like to push?",
---      Buttons = {
---        "Button 1",
---        "Button 2",
---        "Button 3",
---      },
---      Handler = UCIDialogHandler,
---    }
---  )
---end
---
---Controls.ShowDialog.EventHandler = ShowDialog
---```
---@param UCI_Name string The name of the target UCI for which to display the dialog.
---@param DialogTable DialogTable table with the dialog elements, see DialogTable object for details
function Uci.ShowDialog(UCI_Name, DialogTable) end

---Set the screen status of a TSC touchscreen controller or UCI Viewer.
---
---Example:
---```lua
----- Control Alias
---TSC = Controls.TSC_Name -- This textbox should contain the name of the TSC or UCI Viewer Inventory instance
---
----- Set Screen
---for _,state in pairs{"On","Off","Dim"} do
---  Controls[state].EventHandler = function()
---    local TSC_Name = TSC.String
---    Uci.SetScreen(TSC_Name,state)
---  end
---end
---```
---@param TSC_Name string The name of the TSC touchscreen controller or UCI Viewer.
---@param State "On" | "Off" | "Dim"
function Uci.SetScreen(TSC_Name, State) end

---Set which UCI to display on a TSC touchscreen controller or UCI Viewer.
---
---Example:
---```lua
----- Control Alias
---TSC = Controls.TSC_Name -- This textbox should contain the name of the TSC or UCI Viewer Inventory instance
---
----- Set UCI
---Controls["Main UCI"].EventHandler = function() local
---  TSC_Name = TSC.String
---  Uci.SetUCI(TSC_Name,"Main UCI")
---end
---Controls["Other UCI"].EventHandler = function()
---  local TSC_Name = TSC.String
---  Uci.SetUCI(TSC_Name,"Other UCI")
---end
---```
---@param TSC_Name string The name of the TSC touchscreen controller or UCI Viewer.
---@param UCI_Name string The name of the UCI.
function Uci.SetUCI(TSC_Name, UCI_Name) end

---Set which UCI page to display on a TSC touchscreen controller or UCI Viewer.
---
---Example:
---```lua
----- Control Alias
---TSC = Controls.TSC_Name -- This textbox should contain the name of the TSC or UCI Viewer Inventory instance
---
----- Set UCI Page
---for key,ctl in ipairs(Controls.Page) do
---  ctl.EventHandler = function()
---    local TSC_Name = TSC.String
---    Uci.SetPage(TSC_Name,"Page "..key)
---  end
---end
---```
---@param TSC_Name string The name of the TSC touchscreen controller or UCI Viewer.
---@param Page_in_UCI string The UCI page to show.
function Uci.SetPage(TSC_Name, Page_in_UCI) end

---Set whether and how a layer is made visible within a specified UCI name and page.
---
---Example:
---```lua
-----Set Layer Visibility
---trans = Controls.Transition
---trans.Choices = {"none","fade","left","right","bottom","top"}
---if #trans.String==0 then trans.String = "none" end
---for ix,layer in ipairs{"The Top Layer","The Middle Layer","The Bottom Layer"} do
---  Controls[layer].EventHandler = function(ctl)
---    Uci.SetLayerVisibility( "Main UCI", "Page 1", layer, ctl.Boolean, trans.String )
---  end
---  Uci.SetLayerVisibility( "Main UCI", "Page 1", layer, Controls[layer].Boolean, trans.String )
---end
---```
---@param UCI_Name string The name of the UCI.
---@param Page_Name string The name of the UCI page.
---@param Layer_Name string The name of the UCI layer.
---@param Visibility  boolean
---@param Transition_Type "none" | "fade" | "left" | "right" | "bottom" | "top"
function Uci.SetLayerVisibility(UCI_Name, Page_Name, Layer_Name, Visibility, Transition_Type) end

---Set whether and how a shared layer is made visible within a specified UCI name and page.---
---Example:
---```lua
-----Set Layer Visibility
---trans = Controls.Transition
---trans.Choices = {"none","fade","left","right","bottom","top"}
---if #trans.String==0 then trans.String = "none" end
---for ix,layer in ipairs{"The Top Layer","The Middle Layer","The Bottom Layer"} do
---  Controls[layer].EventHandler = function(ctl)
---    Uci.SetSharedLayerVisibility( "Main UCI", "Page 1", layer, ctl.Boolean, trans.String )
---  end
---  Uci.SetSharedLayerVisibility( "Main UCI", "Page 1", layer, Controls[layer].Boolean, trans.String )
---end
---```
---@param UCI_Name string The name of the UCI.
---@param Layer_Name string The name of the UCI layer.
---@param Visibility boolean
---@param Transition_Type "none" | "fade" | "left" | "right" | "bottom" | "top"
function Uci.SetSharedLayerVisibility(UCI_Name, Layer_Name, Visibility, Transition_Type) end

---Log off from a specified TSC touchscreen controller or UCI Viewer
---
---Example:
---```lua
----- Control Alias
---TSC = Controls.TSC_Name -- This textbox should contain the name of the TSC or UCI Viewer Inventory instance
---
----- UCI Log Off
---Controls["Log Me Out"].EventHandler = function()
---  print("Uci.LogOff() can only log out if PIN security is enabled on the panel")
---  Uci.LogOff(TSC.String)
---end
---```
---@param TSC_Name string The name of the TSC touchscreen controller or UCI Viewer.
function Uci.LogOff(TSC_Name) end
