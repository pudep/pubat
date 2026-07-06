# Pubat 

Pubat is my own tui version of bat but only for file viewing. 

# History 

I was working on Pude text editor and pubat was previously was a text editor. I failed to make a real text editor due to many complexities so I ended up giving final touch to the project and made it shift into a file viewer.

Yes, In this project I used ai as I was learning rust via making this project so use of ai was  either only for complex task or for learning new topics like Ropes, textwrap etc. 

# Selling point.

Well I dont know that this will be what you like because `pubat` uses :

1. Ropes data structure (via `Ropey crate`) to load file in one go.
2. Uses `Ratatui` to draw a alternate tui i.e it **it never prints to your terminal screen**
3. Uses `textwrap crate` to display NoHyphenation ascii space text wrapping like `Helix`.
4. `Crossterm` for `Up`/`Down`/`<C-q>` keys.
5. Displays line numbers that respect wrapping.
6. Uses `Syntect` crate for smart extension guided bat like highlighting.

7. Uses `shellexpand` to expand your path variables.

8. Scrolls one line up and down and never remembers your last cursor position as it dont have any cursor at all.

9. Displays a file well in `Tokyonight moon` theme sometimes better than `bat` or `cat`.

# How to download

This shell command will find you shell and append the Path variable to your shell. This is a safe independent of any shell script.

```sh
cargo install --git https://github.com/pudep/pude

sh -c '
  case "$(basename "$SHELL")" in
    fish)
      rc="$HOME/.config/fish/config.fish"
      mkdir -p "$HOME/.config/fish"
      line="set -gx PATH \$HOME/.cargo/bin \$PATH"
      ;;
    zsh)
      rc="$HOME/.zshrc"
      line="export PATH=\"\$HOME/.cargo/bin:\$PATH\""
      ;;
    *)
      rc="$HOME/.bashrc"
      line="export PATH=\"\$HOME/.cargo/bin:\$PATH\""
      ;;
  esac

  grep -qxF "$line" "$rc" 2>/dev/null || echo "$line" >> "$rc"

  exec $SHELL
'
```
