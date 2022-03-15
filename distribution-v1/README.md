# Donations & Prizes distribution

This folder contains:

- the list of donors in `donors.json`,
- the donations breakdown in `donationsBreakdown.txt`,
- the list of prizes in `prizes.json`,
- the prizes distribution in `prizesDistribution.json`.

If you want to regenerate by yourself these files, install [Node.js](https://nodejs.org) and [Yarn](https://yarnpkg.com), then run `yarn`.

## Regenerate the list of donors

Run:

```
yarn download-donors
```

## Regenerate donations breakdown

Run:

```
yarn breakdown-donations
```

## Regenerate the list of prizes

Run:

```
yarn download-prizes
```

## Regenerate prizes distribution

Run:

```
yarn distribute-prizes
```
