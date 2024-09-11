---@meta
---@diagnostic disable: missing-return, unused-local

---Use the Snapshot class to view, load, and save snapshots in the running design.
--- AI Generated
---@class Snapshot
Snapshot = {}

--- AI Generated
---Get a table containing all snapshot names in the current design.
---This function returns a table with strings representing each snapshot's name.
---@return table
function Snapshot.GetNames() end

--- AI Generated
---Load a snapshot from the specified bank with an optional ramp time.
---The snapshot is loaded into the running design and its values are applied.
---
---@param name string The name of the snapshot bank, in quotes, from which to load the snapshot.
---@param bank integer The bank number that identifies a snapshot within the specified bank (1-10).
---@param ramp? number? Optional. Ramp time, in seconds, when loading the snapshot (default 0).
function Snapshot.Load(name, bank, ramp) end

--- AI Generated
---Save a snapshot with the specified name and bank.
---The snapshot is saved to the current design and its values are written out.
---
---@param name string The name of the snapshot bank to which to save the snapshot.
---@param bank integer The bank number that identifies a snapshot within the specified bank (1-10).
function Snapshot.Save(name, bank) end
