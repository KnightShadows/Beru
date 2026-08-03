# `beru index update`

Updates the local copy of the Beru package registry.

## Usage
```bash
beru index update
```

## Description
Beru uses a decentralized Git repository to store package recipes. By default, a clone of this repository is stored at `~/.beru/index`. 

When you run `beru index update`, Beru performs a `git pull` in this directory to fetch the latest package versions and recipes submitted by the community. You should run this periodically if you want to use the latest versions of libraries.

## Options
*None.*

