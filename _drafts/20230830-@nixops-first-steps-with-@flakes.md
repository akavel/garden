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
and the [NixOps 2.0 documentation][nixops2-flakes] is then more in some vague hints territory...

[nixops-doc]: https://nixos.org/nixops/manual
[nixops-doc-today]: https://hydra.nixos.org/build/115931128/download/1/manual/manual.html
