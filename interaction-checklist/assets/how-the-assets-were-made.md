# How the assets were made

## Initial tiles

For the inital template I wanted arrow tiles and a a little character to move around the screen. [This CC0 tileset by surt/vk](https://opengameart.org/content/roblocks) fit the bill. So I clipped some of the tiles from there.

## Non-initial tiles

I decided that a solid colour for checked cells, where the edges would connect each other, was good for this use case, since the checked cells and the unchecked cells are conceptually disjoint sets. We want the checked cells to overtake the unchecked ones. So the blob expanding seems appropriate.

The checkmark itself was hand-drawn by me. I drew it with 2-by-2 blocks of pixels. First was a diagonally-down line where the blocks did not overlap, then a second "45 degree" line upward, which overlapped the first, where the blocks consistently overlaped the previous block by one pixel.