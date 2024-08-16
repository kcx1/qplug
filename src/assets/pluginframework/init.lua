-- Derived from Basic Framework Plugin by QSC
-- Original work created in October 2020

-- This file is part of a project licensed under the MIT License.
-- See the LICENSE.txt file in this directory for the full license text.

-- Plugin Framework Template
-- by Casey Compton / Ascend Studios
-- September 2023

------------------
-- Setup plugin --
------------------

-- Information block for the plugin
require("setup.info")
require("setup.colors")
require("setup.pretty_name")

-- Define the pages for the plugin
require("setup.pages")

-- OPTIONAL: Define the model for the plugin
require("setup.model")

-- Define User configurable Properties of the plugin
require("properties.properties")

-- Optional function to define pins on the plugin that are not connected to a Control
require("properties.pins")

-- Optional function to update available properties when properties are altered by the user
require("properties.rectify_properties")

require("control_components.components")

require("control_components.wiring")

-- Defines the Controls used within the plugin
require("control_components.controls")

--Layout of controls and graphics for the plugin UI to display
require("layout.layout")

require("runtime.runtime")
