---@meta
---@diagnostic disable: missing-return, unused-local

---@enum Mode
---@private
---| "low"
---| "medium"
---| "quartile"
---| "high"

---Rappresent a QRCode, NOT used normally, use only the functions/methods
---@class QRCode
---@field url string
---@field mode Mode
QRCode = {}

---Generate a QR code SVG graphic.
---url : The URL string to encode.
---mode : Optional error correction mode string. Replace with low, medium, quartile, or high. Defaults to "medium" if not passed.
---low = Level L, up to 7% error correction.
---medium = Level M, up to 15% error correction.
---quartile = Level Q, up to 25% error correction.
---high = Level H, up to 30% error correction.
---@param url string
---@param mode Mode
function QRCode.GenerateSVG(url, mode) end
