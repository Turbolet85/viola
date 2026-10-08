-- The minimal config of this chunk's own compositor: no autostart, no bind, no XWayland, no shell, no locker.
hl.config({
  xwayland = { enabled = false },
  misc = { disable_hyprland_logo = true, disable_splash_rendering = true },
  ecosystem = { no_update_news = true, no_donation_nag = true },
  animations = { enabled = false },
  debug = { disable_logs = false },
})
