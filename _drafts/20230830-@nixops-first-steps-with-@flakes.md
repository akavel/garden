I use Nix a lot where I can, and dabble with NixOS on a secondary, "non-critical" personal laptop.
I like the idea a lot, especially with the improved determinism thanks to Nix Flakes.
For a number of reasons, I decided to try and migrate my (currently barebones) personal website
from a shared, PHP-only host to a VPS, which would hopefully let me experiment with more cool apps.
I thought it would be super cool if I could provision the new website Nix style, with NixOps.

<<TODO: LINKIFY ABOVE>>
<<TODO[LATER]: howto for NixOS on RackNerd hosting - nix-infect, lowendbox.com>>
<<TODO[LATER]: maybe more fluff>>

## Installing NixOps on local NixOs machine

Unfortunately, the recommended `nix-env -i nixops`
(or rather, on my all-in experimental Nix tech machine: `nix profile install nixpkgs#nixops`)
incantation did not work for me.
It failed with a [fairly cryptic message about python-2.7.18.6 being "insecure", or EOLed][py2insecure].
What I eventually somehow found, is that a newer version of NixOps is being developed,
IIUC dubbed "NixOps 2.0", and seemingly using python3, given that it installed successfully: 

[py2insecure]: https://github.com/NixOS/nixops/issues/1564

    $ nix profile install nixpkgs#nixopsUnstable

After this went fine, I removed it...

    $ nix profile list | grep nixopsUnstable
    27 flake:nixpkgs#legacyPackages.x86_64-linux.nixopsUnstable [...]
    $ nix profile remove 27

...so that I could put it in my regular declarative config instead:

```diff
diff --git a/modules/host-ux305c.nix b/modules/host-ux305c.nix
index 464d6d1..ac1cab4 100644
--- a/modules/host-ux305c.nix
+++ b/modules/host-ux305c.nix
@@ -16,6 +16,10 @@ with pkgs;
       up
       zettlr
       zip
+
+      # [2023.08] Regular 'nixops' fails, because it depends on python2, which
+      # is now marked as "vulnerable"/insecure.
+      nixopsUnstable
     ];
 
     home.username = "akavel";
```


## Deploying "hello world" nginx website with NixOps 2.0 + Flakes

The nice added benefit of NixOps 2.0
(making me really happy that I actually was forced to use it)
is that it [apparently adds support for Nix Flakes][nixops2-flakes]
&mdash; yay for all-in Nix experimentality! 🥳

[nixops2-flakes]: https://nixops.readthedocs.io/en/latest/manual/migrating.html

The not-so-nice thing is,
that with lacking and chaotic (but improving! 💖) documentation being the norm in the Nix core toolset,
the [NixOps documentation][nixops-doc] seems comparably *very* lacking ([state at time of writing][nixops-doc-today]),
and the [NixOps 2.0 documentation][nixops2-flakes] is then more in some [vague hints territory][nixops2-nodoc]...

[nixops-doc]: https://nixos.org/nixops/manual
[nixops-doc-today]: https://hydra.nixos.org/build/115931128/download/1/manual/manual.html
[nixops2-nodoc]: https://github.com/NixOS/nixops/issues/1553

Fortunately, with some previous experience with Flakes, as well as luck and determination,
I was [able to piece together][nixops2-nodoc] a (not so) simple `flake.nix`
that I managed to get to work and deploy on my remote machine without making it inaccessible.
To break it down into steps for easier understanding,
we can start with...

### Phase 1: Barebones Flake scaffolding for NixOps 2.0

```nix
{
  description = "NixOps config of my servers";

  inputs = {
    # I used NixOS 22.11, as this matches what was recommended by the
    # nix-infect usage guide at the time of writing. And nix-infect was
    # what I used to install NixOS on my remote machine. 
    nixpkgs.url = "github:nixos/nixpkgs/release-22.11";
  };

  outputs = { self, ... }@inputs: {
    nixopsConfigurations.default = {
      inherit (inputs) nixpkgs;     # required! nixops complains if not present
      network.storage.legacy = {};  # required! nixops complains if not present

      ## TODO: here we will specify all the "regular" NixOps properties,
      ## like network.description, machine definitions, etc.
      ## ...

    };
  };
}
```

Interestingly, already at this point you can start playing with `nixops` subcommands,
even though there's not really even any machine defined yet.
The naming here is somewhat weird and confusing to me,
as seems quite typical usually in the core Nix world, unfortunately...
(though Nix 2.0 fortunately improves on this,
although not without its own new quirks...)

<<TODO: VERIFY BELOW TRUE>>
<<TODO: git add & commit? + nix flake update>>

Specifically, the following will now work:

    $ nixops create

Weirdly named as it is, this doesn't really do much practically useful,
but is still required.
Personally, at my current (lack of) understanding level of NixOps,
I see it more as a kind of *"nixops init"* command.
What it seems to do,
is initialize some data in a local SQLite database
used by NixOps to store cached state data and metadata about machines etc.

With that said,
please note I don't yet know nor understand
why NixOps allows calling `nixops create` multiple times
and thus create multiple so-called "deployments" in its database.
Maybe I'll learn some day.
For now, after I called it twice,
I had to then call `nixops delete` to keep just one "deployment" in the DB
and save myself some trouble
(otherwise NixOps doesn't know which "deployment" to use as the default one).

### Phase 2: Adding the first machine, without losing SSH connectivity

Ok, from the title you may have guessed what happened to me...
After I happily added some basic config for my machine and managed to deploy it,
I promptly lost SSH connectivity to it.

Shows up, NixOps seems to like to *completely* take over a remote NixOS machine's config.
This means that you apparently need to fully reproduce whole `/etc/nixos/configuration.nix`
(with all included files)
on your local (controlling) machine.
In my case, the original `/etc/nixos/configuration.nix` and `/etc/nixos/hardware-configuration.nix`
were created by the *nixos-infect* tool that I used on the remote machine
to convert it from an Ubuntu deployment to a NixOS one.
I copied them (with `scp`) verbatim from the remote machine to the local one
into a subdirectory.
I then had to tweak them further,
as the `//` merge operator of Nix shows up to be **non-recursive**
(a.k.a. shallow merge, not deep merge).

<<TODO[LATER]: lib.recursivelyMerge/Update or something>>
<<TODO[LATER]: linkify nixos-infect>>

<details>
    <summary>File <code>racknerd/configuration.nix</code></summary>

```nix
# Initially created by `nix-infest`, on 2023-08-29.
# Afterwards edited & tweaked for nixops.
{ ... }: {
  imports = [
    ./hardware-configuration.nix
  ];
  boot.cleanTmpDir = true;
  zramSwap.enable = false;
  system.stateVersion = "22.11";
}
```
</details>

<details>
    <summary>File <code>racknerd/hardware-configuration.nix</code></summary>

```nix
{ modulesPath, ... }:
{
  imports = [ (modulesPath + "/profiles/qemu-guest.nix") ];
  boot.loader.grub.device = "/dev/vda";
  boot.initrd.availableKernelModules = [ "ata_piix" "uhci_hcd" "xen_blkfront" "vmw_pvscsi" ];
  boot.initrd.kernelModules = [ "nvme" ];
  fileSystems."/" = { device = "/dev/vda1"; fsType = "ext4"; };
  swapDevices = [ { device = "/dev/vda2"; } ];
}
```
</details>

```diff
     nixpkgs.url = "github:nixos/nixpkgs/release-22.11";
   };
 
   outputs = { self, ... }@inputs: {
     nixopsConfigurations.default = {
       inherit (inputs) nixpkgs;     # required! nixops complains if not present
+
+      network.description = "akavel's servers";
       network.storage.legacy = {};  # required! nixops complains if not present
 
-      ## TODO: here we will specify all the "regular" NixOps properties,
-      ## like network.description, machine definitions, etc.
-      ## ...
-
+      ### Machines ###
+
+      my-machine = { pkgs, ... }:
+        (import ./racknerd/configuration.nix {}) // {
+        deployment.targetHost = "1.2.3.4";   # replace with your machine's IP
+
+        networking.hostName = "my-hostname";
+        networking.domain = "";
+        # Allow nginx SSH through the firewall - TODO: is it required or automatic?
+        networking.firewall.allowedTCPPorts = [ 22 ];
+
+        services.openssh.enable = true;
+        users.users.root.openssh.authorizedKeys.keys = [
+          ''ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIP85cjGLDUGWsYWzlJbr/r6Bsdi30ZGZb5/5IzQipYpS me@local-machine''
+        ];
+      };
     };
   };
```

<<TODO: VERIFY>>

Now, I could finally run it:

    $ nixops deploy

