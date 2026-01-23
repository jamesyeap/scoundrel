# Scoundrel
A solo card game of risk and survival played with a standard deck of cards, now playable in your terminal.

![images](./resources/images/game_screen_1.gif)

## Installation
For macOS and Linux users:
```bash
brew install jamesyeap/jamesyeap/scoundrel
```

For Windows users:
* download the `.exe` file from the [Releases](https://github.com/jamesyeap/scoundrel/releases) section.

## How To Play
In Scoundrel, you play the role of a lone rogue descending through a deadly dungeon.
* Each suit represents either:
   * a monster to fight, or
   * a weapon to wield, or
   * potions to heal
* Watch this [tutorial](https://www.youtube.com/watch?v=Gt2tYzM93h4) (credits to [@Rulies](https://www.youtube.com/@Rulies))

## Roadmap
- [x] Implement game engine
- [x] Implement UI in Ratatui 
- [ ] Improve splash screen
- [ ] Implement an in-game tutorial
- [ ] Track which cards have already been drawn
- [ ] Improve end-of-game screens (won/lost)
- [ ] Add persistence (can view past runs)
- [ ] Publish it as a web-app with [Ratzilla](https://github.com/orhun/ratzilla)
- [ ] Add feature where at the end of the run, you can compare your score against a bot (that is also playing the same run)
   - You should be able to see the steps that the bot has taken. 
- [ ] Automatically publish new releases to brew - my brew repo is [here](https://github.com/jamesyeap/homebrew-jamesyeap)