# Divicards

Desktop Application for Path of Exile divination cards

## Demo

![Application Demo Workflow](assets/demo.gif)

### Features

-   Pulling your stash contents from GGG's API to create a list of divination cards you have in your tabs. You can select which tabs you want to export and can select several at once. This uses OAuth and does not rely on poesessid, login credentials, or any other information that could harm the user;
-   Download the resulting spreadsheet locally or post it to Google Sheets with further formatting options
-   Convert other table formats to the one used by Divicards. You can drag a file into it, and as long as it resembles a table with a list of names and amounts, it should work. It can auto-correct typos if the input table was created by hand and contains mistakes;
-   Merge several tables into one;
-   Automatically pull and calculate other useful information, such as prices according to the current poe.ninja rates, and weight estimates assuming you got the cards from stacked decks (it's difficult to get a large sample of generic cards from any other source);

### Website

Also, check out [Website](https://divicards-site.pages.dev/) [repo](https://github.com/shonya3/divicards-site)

## Install

Install the latest release (https://github.com/shonya3/divicards/releases)

## Notable crates in repo:

-   [Divi](https://github.com/shonya3/divicards/tree/main/divi) - parse divination cards sets and calculate real weight
-   [Divcord](https://github.com/shonya3/divicards/tree/main/divcord) - parse [divcord spreadsheet](https://docs.google.com/spreadsheets/d/1Pf2KNuGguZLyf6eu_R0E503U0QNyfMZqaRETsN5g6kU/edit?pli=1#gid=0)

## Platform

-   [Tauri](https://tauri.app/)

-   Web Components with [Lit](https://lit.dev)

-   [Vue3](https://vuejs.org/)

## Recommended IDE Setup

-   [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Lit](https://marketplace.visualstudio.com/items?itemName=runem.lit-plugin) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
