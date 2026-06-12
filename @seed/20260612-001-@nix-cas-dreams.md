# Nix content-addressing dreams

Interestingly, although Nix does hash the _inputs_ (ideally[^1] meaning source code),
it doesn't actually protect too well the _outputs_ against subversion...
because it regularly downloads the outputs (i.e. compiled binaries) from a cache,
and thus requires extending the trust to the cache server...
The core packages presumably download from the official cache,
but some thirdparty packages
[do like to try and happily mandate extending global trust permanently towards external caches](https://github.com/microvm-nix/microvm.nix/issues/491),
posing [a concrete security concern](https://garnix.io/blog/stop-trusting-nix-caches/).
AFAIU there's a (slow going, [maybe even stuck](https://github.com/NixOS/nix/issues/859))
effort to try and support content-addressed fetching of binaries (i.e. outputs) as well;
I kinda hope for a future when the nixpkgs repo includes expected hashes of the build outputs,
and the `nix` command downloads them via DHT from torrents or somewhere such,
with the official Nix Hydra server only working as a torrent seed.
(See also: [related](https://www.tweag.io/blog/2020-09-10-nix-cas/),
[related](http://www.chriswarbo.net/blog/2025-03-16-nix_ipfs.html)/[archive](https://web.archive.org/web/20260216061959/http://www.chriswarbo.net/blog/2025-03-16-nix_ipfs.html),
[related](https://discourse.nixos.org/t/pre-rfc-generic-content-addressed-fetchers-ipfs-radicle-etc/75367/21),
[related](https://github.com/NixOS/rfcs/pull/17),
[related](https://news.ycombinator.com/item?id=38064308).)

[^1]: Quite often, packages definitions unfortunately use prebuilt binary blobs, i.e. "releases" from github, as "inputs". At least they have to hash them, thus pinning/"content-addressing", but then again the trust is moved onto the person who uploaded the compiled binary to the github releases page, thus introducing disconnect from the actual source code.

_([PESOS](https://indieweb.org/PESOS): https://lobste.rs/c/2t6tya)_
