# shell.nix for Smithay Anvil
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    
    # Wayland dependencies
    wayland
    wayland-protocols
    wayland-scanner
    
    # Input and display dependencies
    libxkbcommon
    libinput
    libudev
    libseat
    libdrm
    libgbm
    mesa
    
    # X11 dependencies (for X11 backend and XWayland support)
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xwayland
    
    # Lua for configuration
    lua5_4
    luajit
    
    # Other dependencies
    pkg-config
    fontconfig
    freetype
    vulkan-loader
    vulkan-headers
    renderdoc
    
    # For image handling
    libpng
  ];
  
  # Environment variables
  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [pkgs.vulkan-loader pkgs.mesa]}:$LD_LIBRARY_PATH"
    export PKG_CONFIG_PATH="${pkgs.libxkbcommon}/lib/pkgconfig:${pkgs.libinput}/lib/pkgconfig:${pkgs.libseat}/lib/pkgconfig:${pkgs.libdrm}/lib/pkgconfig:${pkgs.mesa}/lib/pkgconfig:${pkgs.wayland}/lib/pkgconfig:$PKG_CONFIG_PATH"
    export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
    
    # For Vulkan support
    export VK_LAYER_PATH="${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d"
    
    echo "Smithay Anvil development environment loaded!"
    echo "Available backends: x11, winit, tty-udev"
    echo "Run with: cargo run -- --<backend>"
  '';
}