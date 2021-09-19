# uggo

CLI tool to pull builds from https://u.gg/.

## Install

You can install from `crates.io`:

```zsh
cargo install uggo
```

Or, by installing the GitHub repo directly:

```zsh
cargo install --git https://github.com/kade-robertson/uggo
```

Finally, you can always find the latest release builds in the [Releases](https://github.com/kade-robertson/uggo/releases/latest) tab.

## Usage

Just run the executable. There are no command-line options.

Once running, you'll be presented with a prompt:

```
query>
```

You can change the mode you want to retrieve builds from by using the `mode` command:

```
query> mode normal
```

A list of valid modes can be seen from using the `modes` command. If you want to see the currently active mode, use `mode` without any arguments. The default is `normal`.

Once you have the mode you want selected, you can make queries in the form `<champion>[,<role>][,<region]`. If `<champion>` is the only item provided, `<role>` defaults to Automatic (whichever has the highest sample size), and `<region>` defaults to `World`. The champion field does it's best to match the intended champion, so an exact match isn't required. For more details on the optional fields, refer to [Roles](#roles) and [Regions](#regions).

Here's some examples:

```
query> Lux
query> Seraphine,mid
query> Ornn,top,na1
query> Sivir,kr
```

Here's an example output, which details runes, shards, spells, ability order, items and best/worst matchups:

```
 --------------------------------
 Build for Ornn, playing Top lane
 --------------------------------
 Resolve                      Inspiration
 Grasp of the Undying  [●··]  Magical Footwear (Row 1)  [·●·]
 Demolish              [●··]  Biscuit Delivery (Row 2)  [··●]
 Conditioning          [●··]
 Overgrowth            [●··]

 Shards:
 - Offense: +9 Adaptive Force
 - Flex: +6 Armor
 - Defense: +6 Armor

 Spells: Flash, Teleport

 Ability Order: W -> Q -> E

 Starting:  Corrupting Potion
     Core:  Frostfire Gauntlet, Plated Steelcaps, Thornmail
      4th:  Abyssal Mask, Anathema's Chains
      5th:  Warmog's Armor, Abyssal Mask, Gargoyle Stoneplate
      6th:  Frozen Heart, Gargoyle Stoneplate, Abyssal Mask

  Best Matchups:  Jayce, Darius, Kled, Tahm Kench, Sion
 Worst Matchups:  Cho'Gath, Illaoi, Gangplank, Irelia, Warwick
```

For runes, the larger dots indicate the position in the particular row that option is in.

By default, when `uggo` is first run it will attempt to connect to the Game Client API, and if able to enables automatically creating rune pages. For this to work, the following needs to be true:

- League of Legends is already running,
- You have at least one editable rune page, and
- An editable rune page must be the current page.

### Roles

`<role>` can be 1 of 7 options:

- Jungle
- Support
- ADCarry
- Top
- Mid
- None
- Automatic

`None` is only used for ARAM, and is the default in this case. Otherwise, `Automatic` is the default.

### Regions

`<region>` can be 1 of 12 options:

- NA1
- EUW1
- KR
- EUN1
- BR1
- LA1
- LA2
- OC1
- RU
- TR1
- JP1
- World

`World` is the default.
