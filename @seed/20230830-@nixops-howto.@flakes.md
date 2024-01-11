# First steps in NixOps, with Flakes

I use [Nix] a lot where I can,
managing my dotfiles through *[home-manager]*,
and dabble with [NixOS] on a secondary, "non-critical" personal laptop[^nixos-slackware].
I absolutely love its premise,
especially with its [determinism improved even further][flake-det] thanks to [Nix Flakes][flakes].
Recently, for a number of reasons, I decided to pull the trigger
to try and migrate my (currently barebones) [personal website][akavel]
from a shared, PHP-only hosting service to a VPS,
which would hopefully let me experiment with more cool apps.
I thought it would be super cool if I could provision the new website Nix style:
controlling a [NixOS-based VPS][nixos-hosting] remotely
by the means of a local [NixOps] description[^nixops-alt].
With, obviously, Flakes wherever possible!
(See also Xe Iaso's [excellent writeup of their own adventures with NixOps][xe-nixops]).

[^nixos-slackware]:
    Unfortunately, I'm not convinced that going all-in on NixOS on a
    critical personal machine is feasible (yet?...), even for a rather tech-savvy person.
    Using it constantly floods me with a kind of "nostalgia",
    reminding me of the painful, neverending struggles of trying to do anything
    at all with Linux sometime in the '90s, until Ubuntu came around.
    Specifically, the name "Slackware" is the key word,
    which seems to spring to my mind almost magically...
    As of yet, the NixOS experience is not like Ubuntu.
    The NixOS experience today is more like the '90s Slackware and Gentoo
    smashed together. But with some *crazy amazing* pink pony unicorns farting
    rainbows of declarative reproducibility and ultimate control
    smeared all over. Yes, with all the blood, gore, and a thousand papercuts.
    No, *not* the ["Equoid" kind of unicorns](https://www.tor.com/2013/09/24/equoid/).
    I think. Right?

[^nixops-alt]:
    Since originally doing & writing this,
    I stumbled upon [a post on r/NixOS about various alternatives to NixOps](
    https://old.reddit.com/r/NixOS/comments/vnajkg/lollypops_simple_parallel_stateless_nixos/ie7afdo/).
    For now I plan to stick with NixOps, given its official capacity
    plus a look through the lens of the [Lindy Effect](https://en.wikipedia.org/wiki/Lindy_effect).
    Also, at least [some](https://colmena.cli.rs/unstable/tutorial/migration.html) of them
    seem to try and provide some backwards-compatibility with NixOps.

[Nix]: https://en.wikipedia.org/wiki/Nix_(package_manager)
[NixOS]: https://en.wikipedia.org/wiki/NixOS
[home-manager]: https://github.com/nix-community/home-manager
[flakes]: https://nixos.wiki/wiki/Flakes
[flake-det]: https://www.tweag.io/blog/2020-05-25-flakes/
[akavel]: http://akavel.com
[nixos-hosting]: https://nixos.wiki/wiki/NixOS_friendly_hosters
[NixOps]: https://nixos.org/nixops
[xe-nixops]: https://xeiaso.net/blog/backslash-kubernetes-2021-01-03

<<TODO[LATER]: howto for NixOS on RackNerd hosting - nix-infect, lowendbox.com>>

```pikchr:render
Ops: box radius 5px fill white "NixOps" fit
     arrow right 150% "SSH" above
OS:  box same "NixOS"

Local: box thin \
 height 2.5 * Ops.height \
 color 0xcccc55 fill 0xffffdd \
 at Ops behind Ops
line invisible from Local.nw to Local.ne \
 "👨‍💻 local" above

Remote: box same at OS
line invisible from Remote.nw to Remote.ne \
 "🖥 remote VPS " above
```


## Installing NixOps on a local machine with Nix

Unfortunately, the recommended `nix-env -i nixops`
(or rather, on my all-in experimental Nix tech machine: `nix profile install nixpkgs#nixops`)
incantation did not work for me.
It failed with a [fairly cryptic message about python-2.7.18.6 being "insecure", or EOLed][py2insecure].
What I eventually somehow found, is that a newer version of NixOps is being developed,
IIUC dubbed "NixOps 2.0", and it installed successfully on my machine
— which presumably also means that it is maybe using Python 3.x already?

[py2insecure]: https://github.com/NixOS/nixops/issues/1564

    $ nix profile install nixpkgs#nixopsUnstable

After this went fine, I removed it...

    $ nix profile list | grep nixopsUnstable
    27 flake:nixpkgs#legacyPackages.x86_64-linux.nixopsUnstable [...]
    $ nix profile remove 27

...so that I could put it in my main, [home-manager]-based declarative config instead:

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
and then the [NixOps 2.0 documentation][nixops2-flakes] is currently more in what I'd call [vague hints territory][nixops2-nodoc]...
***UPDATE:*** I just stumbled upon [a very helpful comment][nixops17-note]
(on a NixOps issue #1452, aptly named *"Documentation (...) is incomplete"*...).
This comment contains [a link to NixOps 1.7 documentation](https://releases.nixos.org/nixops/nixops-1.7/manual/manual.html),
which seems to provide **the last most complete docs of NixOps in the 1.x version line**.
I recommend taking a look, and juggling that together with the provisional 2.0 docs,
plus possibly Flakes docs, NixOS/Nixpkgs docs, and Nix expression language docs
to achieve what you want with NixOps... 😅

[nixops-doc]: https://nixos.org/nixops/manual
[nixops-doc-today]: https://hydra.nixos.org/build/115931128/download/1/manual/manual.html
[nixops2-nodoc]: https://github.com/NixOS/nixops/issues/1553
[nixops17-note]: https://github.com/NixOS/nixops/issues/1452#issue-862860327

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

**<<FIXME: convert to use `pkgs.lib.recursiveUpdate`, [as recommended in nix.dev/recipes/best-practices](https://nix.dev/recipes/best-practices#updating-nested-attribute-sets).>>**

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
+        # Allow SSH through the firewall - TODO: is it required or automatic?
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


### Phase 3: Nginx "hello world"

To make Nginx run with its default *"Welcome to Nginx!"* page, the following changes were needed:

```diff 
         networking.hostName = "my-hostname";
         networking.domain = "";
-        # Allow SSH through the firewall - TODO: is it required or automatic?
-        networking.firewall.allowedTCPPorts = [ 22 ];
+        # Allow nginx (and ssh) through the firewall
+        networking.firewall.allowedTCPPorts = [ 80 22 ];
 
+        services.nginx.enable = true;
+
         services.openssh.enable = true;
         users.users.root.openssh.authorizedKeys.keys = [
```

Then, to make it display something more interesting and custom,
I grabbed the following snippet from somewhere over the InterWebs:

```diff
         # Allow nginx (and ssh) through the firewall
         networking.firewall.allowedTCPPorts = [ 80 22 ];
 
-        services.nginx.enable = true;
+        services.nginx = {
+          enable = true;
+          virtualHosts.vhost1 = {
+            default = true;
+            locations."/" = {
+              root = pkgs.writeTextDir "index.html" "Hello akavel's world!";
+            };
+          };
+        };
 
         services.openssh.enable = true;
         users.users.root.openssh.authorizedKeys.keys = [
```

Each of those two steps should be deployable with:

    $ nixops deploy
