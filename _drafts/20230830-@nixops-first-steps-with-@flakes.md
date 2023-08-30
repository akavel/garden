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

[py2insecure]: https://github.com/NixOS/nixops/issues/1564


