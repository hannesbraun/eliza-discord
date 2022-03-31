# ELIZA Discord Bot

This is a small Discord bot containing the ELIZA chatbot (powered by [eliza-rs](https://github.com/arosspope/eliza-rs)).

## Building

```bash
cargo build --release
```

If you want to reduce the size of the resulting binary, try using `strip` and `upx`.

## Usage

Write your token for the bot into a file. Supply the path to the file as the first argument to the program.
If no path is supplied, the bot tries to read the token from the file `ELIZA_DISCORD_TOKEN` in your current directory.

A new instance of ELIZA is created (lazily) for each channel the bot has access to.
Within a channel, ELIZA will act as if she's communicating with only one person, even if it's a channel inside a guild.
Just try it out.

## License
This software is licensed unter the Boost Software License. For more information see [LICENSE](LICENSE).
