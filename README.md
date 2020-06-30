# Santa Rustavia

Rule a city-state. Conquer your foes!

In 1978 the Reverend (later Professor) George Blank wrote a strategy game for the TRS-80 called *Santa Paravia en Fiumaccio*. It was a spiritual successor to *Hamurabi* (see below) and a predecessor of games like Sid Meier's *Civilization*.

The BASIC code for *Santa Paravia en Fiumaccio* was published in the December 1978 issue of *SoftSide* magazine, which you can read at the Internet Archive [here](https://archive.org/details/softside-magazine-03).

The MobyGames page for the game can be found [here](https://www.mobygames.com/game/browser/santa-paravia-and-fiumaccio) and a short bio of Professor George Blank is [here](https://www.mobygames.com/developer/sheet/view/developerId,134909/).

The game was ported to C by Thomas Knox. You can find that version [here](https://github.com/DNSGeek/Random-Stuff/blob/master/paravia.c). That port was modernised to C99 by [darkf](https://github.com/darkf), you can find that [here](https://github.com/darkf/paravia).

There is an article about the development of the game at [Games Nostalgia](https://gamesnostalgia.com/story/166/the-fascinating-story-of-santa-paravia-and-fiumaccio) and there is more information at [Wikipedia](https://en.wikipedia.org/wiki/Santa_Paravia_en_Fiumaccio).

Santa Rustavia is a homage of the original game. It is a port of the modernised C code to Rust with a few tweaks.

Like the C ports, this port lacks the map routine of the original BASIC, which you can read for yourself [on page 43 of SoftSide December 1978](https://archive.org/details/softside-magazine-03/page/n41/mode/2up), but is otherwise fully playable.

The game supports an early form of multiplayer called "hot seat". Before internet gaming and LAN parties, you could still play with your friends by taking turns to sit at the computer to play your turn, then getting out of the chair for the next player. More exercise than today! If you select more than 1 player, you will each be asked to pick a name and take turns managing your cities. There is also a computer player called "Baron Peppone" to challenge you.

You will need Rust installed. The easiest way to do that is [rustup](https://rustup.rs/).

Compile the program with `cargo run` or `cargo build`.

This is a text-based game that runs in the terminal. The game will ask you questions that you must answer to make decisions about your kingdom.

Press <kbd>Ctrl</kbd> + <kbd>c</kbd> to quit at any time.

If you enjoy this sort of thing, you may also like [Hamurusti](https://github.com/candolle/hamurusti), a similar port of 1968's [Hamurabi](https://en.wikipedia.org/wiki/Hamurabi_%28video_game%29) to Rust.
