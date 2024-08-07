-- Optional function to define model if plugin supports more than one model
function GetModel(props)
	local model = {}
	if props.Model ~= nil and props.Model.Value ~= "" then
		table.insert(model, { props.Model.Value })
	else
		table.insert(model, { "Base Model" })
	end
	return model
end
