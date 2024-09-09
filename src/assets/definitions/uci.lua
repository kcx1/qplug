---@meta
---@diagnostic disable: missing-return, unused-local

Uci = {}

---Table that store UCI elements
DialogTable = {
	Title = "",
	Message = "",
	---The button list is a table consisting of strings – one string per desired response button.
	Buttons = {},
	---The EventHandler Function receives an integer index of which button was pressed.
	---The EventHandler argument is zero-based, so add 1 to the integer to match a Lua table entry.
	Handler = nil,
}

---Display a dialog in a UCI that contains a title, message, and button selection list.
---UCI_Name : The name of the target UCI for which to display the dialog.
---DialogTable : table with the dialog elements, see DialogTable object for details
function Uci.ShowDialog(UCI_Name, DialogTable) end

---Set the screen status of a TSC touchscreen controller or UCI Viewer.
---TSC_Name : The name of the TSC touchscreen controller or UCI Viewer.
---State : "On" | "Off" | "Dim"
function Uci.SetScreen(TSC_Name, State) end

---Set which UCI to display on a TSC touchscreen controller or UCI Viewer.
---TSC_Name : The name of the TSC touchscreen controller or UCI Viewer.
---UCI_Name : String. The name of the UCI.
function Uci.SetUCI(TSC_Name, UCI_Name) end

---Set which UCI page to display on a TSC touchscreen controller or UCI Viewer.
---TSC_Name : The name of the TSC touchscreen controller or UCI Viewer.
---Page_in_UCI : The UCI page to show.
function Uci.SetPage(TSC_Name, Page_in_UCI) end

---Set whether and how a layer is made visible within a specified UCI name and page.
---UCI_Name : String. The name of the UCI.
---Page_Name : String. The name of the UCI page.
---Layer_Name : String. The name of the UCI layer.
---Visibility : true | false
---Transition_Type : "none" | "fade" | "left" | "right" | "bottom" | "top"
function Uci.SetLayerVisibility(UCI_Name, Page_Name, Layer_Name, Visibility, Transition_Type) end

---Set whether and how a shared layer is made visible within a specified UCI name and page.
---UCI_Name : String. The name of the UCI.
---Layer_Name : String. The name of the UCI layer.
---Visibility : true | false
---Transition_Type : "none" | "fade" | "left" | "right" | "bottom" | "top"
function Uci.SetSharedLayerVisibility(UCI_Name, Layer_Name, Visibility, Transition_Type) end

---Log off from a specified TSC touchscreen controller or UCI Viewer
---The name of the TSC touchscreen controller or UCI Viewer.
function Uci.LogOff(TSC_Name) end
