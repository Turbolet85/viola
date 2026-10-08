-- The config of this chunk's own compositor for the live work: no autostart, no bind, no XWayland, no shell,
-- no locker; one output at scale 1, no gaps and no border, so a terminal window gets the whole output.
hl.monitor({ output = "", mode = "preferred", position = "auto", scale = 1 })
hl.config({
  xwayland = { enabled = false },
  general = { gaps_in = 0, gaps_out = 0, border_size = 0 },
  misc = { disable_hyprland_logo = true, disable_splash_rendering = true },
  ecosystem = { no_update_news = true, no_donation_nag = true },
  animations = { enabled = false },
  debug = { disable_logs = false },
})
