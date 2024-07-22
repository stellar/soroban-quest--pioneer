# Soroban Pioneer Quest <!-- omit in toc -->

![Stellar-Quest-email](https://user-images.githubusercontent.com/4383610/200077219-de8e1f20-9878-4705-bec6-ced9a3904694.jpg)

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)][gitpod]

## Table of Contents <!-- omit in toc -->

- [Welcome](#welcome)
- [Video Walkthrough](#video-walkthrough)
- [Gitpod](#gitpod)
  - [VS Code](#vs-code)
  - [Terminal and Ports](#terminal-and-ports)
  - [Docker Container'd](#docker-containerd)
  - [Gitpod CLI](#gitpod-cli)
- [Stellar Quest CLI](#stellar-quest-cli)
- [Rust Environment](#rust-environment)
- [Stellar CLI](#stellar-cli)
- [Testnet](#testnet)
- [Good Luck](#good-luck)
- [Join us on Discord](#join-us-on-discord)

## Welcome

Welcome to the Pioneer Quest for our upcoming Soroban Quest! We are beyond
excited you've joined us. These quests are going to be fun, exciting, and
_interesting_ to be sure! There's a lot to go through so you are up to speed, so
let's jump in!

## Video Walkthrough

The rest of this README is quite heavily skewed toward written information.
That's not everybody's primary way of learning, so we've created a video
walkthrough you can check out, as well. It doesn't contain everything in this
README, but it's a start. If that kind of content sounds like it's more up your
alley, you can [watch the video right here][video]!

[![Soroban Pioneer Quest Walkthrough][thumbnail]][video]

## Gitpod

You are reading this _from inside_ a Gitpod development environment. You heard
right! A freshly baked, automated environment built just for you! A couple
things you should know about Gitpod:

### VS Code

Gitpod is built around a fully-functional copy of the VS Code IDE. Entirely in
your browser! All the things you can do in the local version of VS Code, you can
do here. Just about any extension you have in your local copy you can install
here. You get the idea.

### Terminal and Ports

In your Gitpod workspace, you'll notice a panel at the bottom that is open to a
few terminal shells. There are a few methods to show/hide this panel, but my
favorite is the ``Ctrl + ` `` keyboard shortcut. There is a lot of information
in the [VS Code docs][vsc-docs] about how to use these integrated terminals.

This panel will actually be pretty important to your questing success, so let's
take a second to examine this a bit more in-depth.

![Terminal Panel][terminals]

#### **Terminals** <!-- omit in toc -->

On the right-hand side of this panel, you'll notice there are 4 (four) shells
open. Each of them is designed for a specific purpose:

- `Albedo Signer` - A simple webapp that facilitates claiming rewards for
  completed quests. (You won't be required to actually _do_ anything in this
  shell.)
- `Testnet: docker` - An instance of the Testnet node that is running inside
  your Gitpod workspace. (You won't be required to actually _do_ anything in
  this shell either.)
- `CLI - Testnet` - This shell is designed for interacting with the Testnet
  network. It has some environment variables customized for this purpose, and
  will make it easier and quicker for you to work with the network. When we say
  something like "you need to deploy a contract to the Testnet," you'll want
  to do that from this shell.

Above this list of terminals, you could use the **+** icon to open another
terminal, if you closed one of yours.

#### **Ports** <!-- omit in toc -->

There are also a couple open ports that can be useful during your quests. You
can see these ports by clicking on **Ports** in the top part of the terminal
panel (see above screenshot). You could also click on **Ports: 8000, 3000** in
the lower-right-hand corner of the Gitpod workspace (see below screenshot).

- **Port 3000** - You can ignore the **Albedo Signer** port. That's open so we
  can use it along with the `sq` CLI to get your earned rewards to you.
- **Port 8000** - On port 8000 of your Gitpod workspace lives various tools to
  interact with the Testnet network. There is a running instance of JSON-RPC
  (which you will need to use) and a Horizon API server (which can be _very_
  useful).

Both of these ports are publicly available on the web, at the listed addresses.
You could even use this RPC endpoint to interact with the Testnet network from
your local computer!

![Open Gitpod Ports][ports]

### Docker Container'd

That's right, Gitpod is running on a Docker-ized container so that we can be
certain _your_ setup for these Soroban Quests is **exactly** the same as _our_
setup! We have a few tasks configured to run on your Gitpod's startup. They're
briefly explained below, but you can also read through `.gitpod.yml` and
`.gitpod.Dockerfile` to get a sense of what is happening during the build
process.

### Gitpod CLI

Gitpod has its very own CLI that you can use to manage a running Gitpod
workspace. It's got loads of useful features, and tons of ways you can use it.
Preat neat, huh!? You can [learn all about it here][gp-cli]!

## Stellar Quest CLI

Would you belive that we've made, specifically for our Soroban Quest, a Stellar
Quest CLI?! No joke! It's _super awesome_, and **absolutely essential** for you
to understand, if you want to succeed in this live series. It's (awesomely)
called "Squirtle" but you'll become more familiar with invoking it as `sq`. The
code for it lives in the `_squirtle/` directory, but you won't need to bother
with anything in there.

It exists as a command line tool that can connect your Gitpod instance with the
SQ backend. You can use it to:

- login to your Stellar Quest account using Discord (or logout),
- get information about the currently logged in user,
- visit the Stellar Quest website,
- fetch new quests when they become available,
- generate a keypair to play a particular quest,
- fund Quest Keypairs on the Testnet,
- check and/or verify the quests you've completed,
- get rewards for completing quests successfully

You can invoke `sq` from within any of the bash shells in this Gitpod workspace.
The output of `sq --help` is shown below, for your convenience. You can also
invoke `sq <command> --help` to get more information on how to use a particular
command.

```bash
$ sq help
sq

Commands:
  sq login          Connect your Stellar Quest account to Gitpod
  sq logout         Disconnect your Stellar Quest account from Gitpod
  sq user           Print out information about yourself           [aliases: me]
  sq open           Open the Stellar Quest website
  sq pull           Pull any new or missing Quests into the /quests directory
  sq play [index]   Generate a Quest Keypair to play a Quest
  sq fund [key]     Create and fund an account on the Testnet
  sq check [index]  Check your Quest answer
  sq submit [xdr]   Submit a signed award XDR to the Stellar Quest backend
  sq rpc            Check the status of your local RPC endpoint
                                                              [aliases: horizon]
  sq                                                                   [default]

Options:
      --version  Show version number                                   [boolean]
  -h, --help     Show help                                             [boolean]
```

## Rust Environment

Crucially toward the goal of writing smart contracts for Stellar, your workspace
contains a fully configured, ready to go Rust development environment. We have
all the tooling, compilers, build processes, and anything else you'll need to
hit the ground running. This includes:

- A specified version of the [Rust][rust] programming language
- This pre-configured [VS Code][vscode] editor, with some essential extensions
- The [Cargo][cargo] package manager for Rust crates
- The `wasm32-unknown-unknown` target for compiling your contracts

You have enough pre-installed to write, debug, test, build, and deploy Stellar
smart contracts from right within this Gitpod workspace. We even have an example
contract ready for you to look through in the `quests/0-hello-world/` directory.

## Stellar CLI

Your workspace includes the Stellar CLI, as well. The Stellar CLI is an
essential tool to interact with Stellar's JSON-RPC servers and execute smart
contracts on the network. You can always find the latest about the [Stellar CLI
here][stellar-cli]!

## Testnet

Did you know you have access to a Stellar network node right now!? Yeah, right
here in your browser, in this workspace there's a Stellar node connected to the
"Testnet" testing network. We've taken care of all the work so you don't have
to worry about docker images, starting the service, or anything besides learning
to make contracts!

This will come in very handy when you are ready to deploy and share your
contracts.

## Good Luck

Now that you're (at least somewhat) familiar with the lay of the land, you're
ready to get questing! Open up that Gitpod, and get to work! Most importantly,
have fun!!

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)][gitpod]

## Join us on Discord

In the [Stellar Developers Discord server][dev-discord], you will find a large,
active, and helpful community! We have recently announced a $100M Soroban
Adoption Fund, which SDF created to support the growth and development of the
Soroban ecosystem. We'll be sharing more about additional programs on the
Stellar Dev Discord in the not-too-distant future, so make sure to join today to
be the among the first to hear those announcements. This is yet another way for
you to **Tinker and Earn** XLM with Stellar smart contracts! Many of the people
who are _creating_ the Soroban platform are there, and willing to answer
questions, too! Talk about "straight from the horse's mouth"!!

[gitpod]: https://gitpod.io/#ENV=prod/https://github.com/stellar/soroban-quest--pioneer
[gp-cli]: https://www.gitpod.io/docs/configure/workspaces/gitpod-cli
[rust]: https://www.rust-lang.org/
[cargo]: https://doc.rust-lang.org/cargo/
[vscode]: https://code.visualstudio.com/
[stellar-cli]: https://developers.stellar.org/docs/tools/stellar-cli
[video]: https://youtu.be/6_tgpth6U5Y
[thumbnail]: https://user-images.githubusercontent.com/2024293/201189898-dd9ae16e-698c-4b2d-b442-fec7d7222f3f.jpg
[terminals]: https://user-images.githubusercontent.com/2024293/201201300-f86bbc98-6c0a-4189-b92c-fe4145c95f0d.png
[vsc-docs]: https://code.visualstudio.com/docs/terminal/basics
[ports]: https://user-images.githubusercontent.com/2024293/201206484-f69f9123-3550-49be-97f2-f6f39fe9aa2f.png
[dev-discord]: https://discord.gg/stellardev
