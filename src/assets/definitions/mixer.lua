---@meta
---@diagnostic disable: missing-return, unused-local

---Mixer objects allow access to Mixer components that have been named in the design.
---To create a mixer object, call Mixer.New( mixerName ).
---The mixer object uses a string specification to determine which inputs and outputs to apply changes to.
---The syntax supports either space or comma separated numbers, ranges of numbers or all (*).
---It supports negation of selection with the '!' operator.
---@class Mixer
---@field mixerName string
Mixer = {}

---Create a new Mixer Object
---@param self Mixer
---@param mixerName string
---@return Mixer
function Mixer:New(mixerName) end

---Sets specified cross point gains with optional ramp time.
---@param ins string[] list of input
---@param outs string[] list of output
---@param gain number Gain value to set
---@param ramp number? optional ramp time to get get to the selected value
function Mixer:SetCrossPointGain(ins, outs, gain, ramp) end

---Sets specified cross point mutes with boolean
---@param ins string[] list of input
---@param outs string[] list of output
---@param mute boolean value for mute
function Mixer:SetCrossPointMute(ins, outs, mute) end

---Sets specified cross point solos with boolean
---@param ins string[] list of input
---@param outs string[] list of output
---@param solo boolean value for mute
function Mixer:SetCrossPointSolo(ins, outs, solo) end

---Sets specified cross point delay with optional ramp time
---@param ins string[] list of input
---@param outs string[] list of output
---@param delay number the delay value in milliseconds
---@param ramp number? optional ramp time to get get to the selected value
function Mixer:SetCrossPointDelay(ins, outs, delay, ramp) end

---Sets specified input gain with optional ramp time
---@param ins string[] list of input
---@param gain number Gain value to set
---@param ramp number? optional ramp time to get get to the selected value
function Mixer:SetInputGain(ins, gain, ramp) end

---Sets specified input mutes
---@param ins string[] list of input
---@param mute boolean value for mute
function Mixer:SetInputMute(ins, mute) end

---Sets specified input solos
---@param ins string[] list of input
---@param solo boolean for solo
function Mixer:SetInputSolo(ins, solo) end

---Sets specified output gain with optional ramp time
---@param outs string[] list of output
---@param gain number Gain value to set
---@param ramp number? optional ramp time to get get to the selected value
function Mixer:SetOutputGain(outs, gain, ramp) end

---Sets specified output mutes
---@param outs string[] list of output
---@param mute boolean value for mute
function Mixer:SetOutputMute(outs, mute) end

---Sets specified input cues enables
---@param ins string[] list of input
---@param cues string[] list of cue
---@param enable boolean to enable/disable cue
function Mixer:SetInputCueEnable(ins, cues, enable) end

---Sets specified input AFL enables
---@param ins string[] list of input
---@param afls number[] list of AFL (After fader level)
---@param enable boolean to enable/disable AFL
function Mixer:SetInputCueAfl(ins, afls, enable) end

---Sets specified cue gains with optional ramp time
---@param cues string[] list of cue
---@param gain number Gain value to set
---@param ramp number? optional ramp time to get get to the selected value
function Mixer:SetCueGain(cues, gain, ramp) end

---Sets specified cue mutes
---@param cues string[] list of cue
---@param mute boolean optional ramp time to get get to the selected value
function Mixer:SetCueMute(cues, mute) end

---@class CrossPoint
---@field Input number
---@field Output number
---@field Gain number
---@field Mute boolean
---@field Solo boolean
---@field Delay number

---Gets specified cross point values
---@param ins string[] list of input
---@param outs string[] list of output
---@return CrossPoint[]
function Mixer:GetMixerCrossPoints(ins, outs) end
