# Passing inputs to Nix modules

Today I learned of basically three patterns
for passing flake inputs (and other arguments) into
home-manager modules.

The examples bellow assume that
you added a new input named `foo-bar` into your main `flake.nix`,
for example: `inputs.foo-bar.url = "..."`.

## 1. Passing packages via overlays

This is the "classic" pattern, harking back to non-flakes NixOS.
Assuming that the newly imported `foo-bar` flake defines _a package_,
you can add it into an overlay like so:

```diff
...
 outputs = { self, ... }@inputs: {
   homeConfigurations = {
     "user@host" = let
       system = "aarch64-darwin";
     in inputs.home-manager.lib.homeManagerConfiguration rec {
-      pkgs = inputs.nixpkgs.legacyPackages.${system};
+      pkgs = import inputs.nixpkgs {
+        inherit system;
+        overlays = [
+          (new: old: { foo-bar = inputs.foo-bar.packages.${system}.foo-bar; })
+        ];
+      };
       modules = [
...
```
Now, a module will have `foo-bar` in its `pkgs`.

## 2. Passing inputs via `extraSpecialArgs`

This is reportedly based on a similar mechanism in NixOS, called `specialArgs`.
I haven't tried it yet, but from what I understand,
it should allow to pass extra stuff as arguments to modules.
The stuff passed can be anything, doesn't have to be a package.
In the main `flake.nix`, the following needs to be added:
```diff
...
 outputs = { self, ... }@inputs: {
   homeConfigurations = {
     "user@host" = let
       system = "aarch64-darwin";
     in inputs.home-manager.lib.homeManagerConfiguration rec {
       pkgs = inputs.nixpkgs.legacyPackages.${system};
+      extraSpecialArgs = { inherit inputs; };  # make inputs available to modules via arg
       modules = [
...
```
Any other arguments can also be added -
the example above adds the whole `inputs`,
it's basically equivalent to: `{ extraSpecialArgs = { inputs = inputs; } }`.

The change above now makes `inputs` available
as argument to any modules:
```diff
-{ pkgs, config, ... }:
+{ pkgs, config, inputs, ... }:
...
```

## 3. Passing inputs via `_module.args`

I also haven't tried it yet.
This pattern seems the newest, and the same in
[NixOS][] and [home-manager][].
It's somewhat similar to variant 2 above
(i.e. `extraSpecialArgs`), but more "local".
I think it may be also "order dependant",
and I'm not sure in what way things get overridden if they're repeated.

[NixOS]: https://nixos.org/manual/nixos/stable/options#opt-_module.args
[home-manager]: https://nix-community.github.io/home-manager/options.xhtml#opt-_module.args

This has the benefit that it allows
communication _between modules_.
That's because it's defined (partly?) within the
structure of the regular "option/config" system.

To pass something into `_module.args`,
you need to _define a new module_ like below:
```diff
...
 outputs = { self, ... }@inputs: {
   homeConfigurations = {
     "user@host" = let
       system = "aarch64-darwin";
     in inputs.home-manager.lib.homeManagerConfiguration rec {
       pkgs = inputs.nixpkgs.legacyPackages.${system};
       modules = [
+        { _module.args = { inherit foo-bar; }; }  # make foo-bar available to modules via arg
...
```
(Note: above is I think a shortcut for
`{ config = { _module.args = { inherit foo-bar; }; }; }`).

This should make `foo-bar` available to (subsequent?) modules,
again as a new named argument:
```diff
-{ pkgs, config, ... }:
+{ pkgs, config, foo-bar, ... }:
...
```
