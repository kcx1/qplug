---@meta
---@diagnostic disable: missing-return, unused-local

---Use the System table to return Q-SYS environment information.
---@class System
---@field BuildVersion string The build version number of Q-SYS environment.
---@field LockingId string The Q-SYS Core's Locking ID, which is used for Q-SYS feature license activation. For more information, see Licensing.
---@field IsEmulating boolean whether the design is emulating or running on a Core.
---@field MajorVersion string The major version number of Q-SYS environment.
---@field MinorVersion string The minor version number of Q-SYS environment.
---@field Version string The complete version number of Q-SYS environment.
System = {}
