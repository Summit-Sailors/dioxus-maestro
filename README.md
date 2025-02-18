## Notes

### Structure

This repo contains crates which may be used in dioxus apps (client and server sides). `clients` help to establish connection with database and make operations, run background tasks etc. `frontend` contains crates for client side - there are reusable ui components (like inputs, buttons, forms, etc), hooks and tools for plotting the data.

## Notes

1. `clients` : you may need to run some additional system dependencies such as:

   - postgresql-devel,
   - postgresql-server,
   - postgresql-contrib,
   - gtk3-devel,
   - gdk-pixbuf2-devel,
   - pango-devel,
   - atk-devel,
   - webkit2gtk4.1-devel,
   - cairo-gobject-devel,
   - libxdo-devel,
   - chromium,
   - chromium-headless,
   - chromedriver,
   - xorg-x11-server-devel,
   - libX11-devel,
   - libXi-devel,
   - libXrandr-devel,
   - libXinerama-devel,
   - libXcursor-devel
     with `sudo` or `brew`

2. `frontend`: in root folder need to have `tailwind.config.js` where in `content` block must be maestro-frontend \*.{html, rs, css} files (thus provided styles will be applied).

Some of the crates have list of features - don't forget specify those you need - keep in mind, that some crates (especially from `clients`) won't compile on client side without features specification.
