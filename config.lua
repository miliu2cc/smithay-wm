-- Anvil Window Manager Configuration
-- This is a sample configuration file for the Anvil window manager

-- Configuration table
config = {
    -- Keybindings configuration
    keybindings = {
        -- Launch terminal with Logo+Return
        {
            modifiers = {"Logo"},
            key = "Return",
            action = "spawn",
            args = { command = "alacritty" }
        },
        
        -- Quit the compositor with Logo+q
        {
            modifiers = {"Logo"},
            key = "q",
            action = "quit"
        },
        
        -- Close focused window with Logo+c
        {
            modifiers = {"Logo"},
            key = "c",
            action = "close_window"
        },
        
        -- Launch application launcher with Logo+d
        {
            modifiers = {"Logo"},
            key = "d",
            action = "spawn",
            args = { command = "rofi -show drun" }
        },
        
        -- Toggle fullscreen for focused window with Logo+f
        {
            modifiers = {"Logo"},
            key = "f",
            action = "toggle_fullscreen"
        },
        
        -- Move focus with Logo+arrow keys
        {
            modifiers = {"Logo"},
            key = "Left",
            action = "focus",
            args = { direction = "left" }
        },
        {
            modifiers = {"Logo"},
            key = "Right",
            action = "focus",
            args = { direction = "right" }
        },
        {
            modifiers = {"Logo"},
            key = "Up",
            action = "focus",
            args = { direction = "up" }
        },
        {
            modifiers = {"Logo"},
            key = "Down",
            action = "focus",
            args = { direction = "down" }
        },
    }
}

-- You can add custom functions here that can be called from keybindings
function custom_action(args)
    -- Your custom action logic here
    print("Custom action called with args: " .. tostring(args))
    return true
end