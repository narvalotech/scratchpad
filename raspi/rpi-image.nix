{ config, pkgs, lib, ... }:

{
  imports = [
    # Basic SD image setup
    <nixpkgs/nixos/modules/installer/sd-card/sd-image-aarch64.nix>
  ];

  # Raspberry Pi 4 Specifics
  boot.kernelPackages = pkgs.linuxKernel.packages.linux_rpi4;

  boot.initrd.availableKernelModules = lib.mkForce [
    "mmc_block"       # Essential for reading the SD card
    "sdhci_pci"       # SD card controller
    "usbhid"
    "usb_storage"
    "vc4"             # Video driver
    "pcie_brcmstb"    # Pi 4 PCIe (for USB)
    "reset-raspberrypi"
    "xhci_pci"        # USB 3.0 controller
  ];

  hardware.enableRedistributableFirmware = true;

  # 3. Console Output
  # If you don't see anything on HDMI, the Pi might be outputting to Serial.
  # This ensures it goes to the monitor.
  boot.kernelParams = [
    "console=ttyS1,115200n8"
    "console=tty0"
    "earlyprintk"
  ];

  # 4. Standard RPi4 Bootloader settings
  boot.loader.grub.enable = false;
  boot.loader.generic-extlinux-compatible.enable = true;

  # Enable SSH
  services.openssh = {
    enable = true;
    settings.PermitRootLogin = "yes"; # Change this to "no" after setting up a user
  };
  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPQ5Q78OSdxA4ALVKy5fPlHW2BucSVhqhF/gIKLrWue3"
  ];

  users.users.root.initialPassword = "l33t";

  systemd.sleep.extraConfig = ''
    AllowSuspend=no
    AllowHibernation=no
    AllowHybridSleep=no
    AllowSuspendThenHibernate=no
    '';

  users.users.jon = {
    isNormalUser = true;
    description = "jon";
    extraGroups = [ "networkmanager" "wheel" "docker" "usb-serial"];
  };

  # Our user doesn't have a password, so we let them
  # do sudo without one
  security.sudo.wheelNeedsPassword = false;

  environment.systemPackages = with pkgs; [
    libraspberrypi
    raspberrypi-eeprom
    xorriso
    vim
    wget
    git
    sbcl
  ];

  networking.firewall.enable = false;

  services.displayManager = {
    autoLogin.enable = true;
    autoLogin.user = "jon";
  };

  # Enable Docker
  virtualisation.docker.enable = true;

  # Tmpfs Logging (Log to RAM instead of SD Card)
  # This significantly extends the life of your SD card.
  services.journald.extraConfig = ''
    Storage=volatile
    RuntimeMaxUse=64M
  '';

  # Basic System Setup
  networking.hostName = "a-ha";
  nixpkgs.hostPlatform = "aarch64-linux";

  systemd.services.home-lisp = {
    # [Unit] section attributes
    description = "Run home.lisp script at boot";
    after = [ "multi-user.target" ];

    path = [ 
      pkgs.bash 
      pkgs.sbcl
      pkgs.coreutils # provides 'env'
    ];

    # [Service] section attributes
    serviceConfig = {
      Type = "simple";
      User = "jon";
      WorkingDirectory = "/home/jon/hal9k";
      ExecStart = "/home/jon/hal9k/support/home.sh";
      Restart = "always";
      RestartSec = "2s";
      StandardOutput = "journal";
      StandardError = "journal";
    };

    # [Install] section attributes
    wantedBy = [ "multi-user.target" ];
  };

  systemd.services.web-lisp = {
    # [Unit] section attributes
    description = "Run web.lisp script at boot";
    after = [ "multi-user.target" ];

    path = [ 
      pkgs.bash 
      pkgs.sbcl
      pkgs.coreutils # provides 'env'
    ];

    # [Service] section attributes
    serviceConfig = {
      Type = "simple";
      User = "jon";
      WorkingDirectory = "/home/jon/hal9k";
      ExecStart = "/home/jon/hal9k/support/web.sh"; 
      Restart = "always";
      RestartSec = "2s";
      StandardOutput = "journal";
      StandardError = "journal";
    };

    # [Install] section attributes
    wantedBy = [ "multi-user.target" ];
  };

  # Base system
  system.stateVersion = "25.11";

  # Enable cross-compilation if building on x86_64
  # nixpkgs.buildPlatform = "x86_64-linux";
}
