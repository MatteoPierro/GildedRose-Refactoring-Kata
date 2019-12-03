package com.gildedrose;

class GildedRose {
    Item[] items;

    public GildedRose(Item[] items) {
        this.items = items;
    }

    public void updateQuality() {
        for (Item item : items) {
            update(item);
        }
    }

    private void update(Item item) {
        if (item.name.equals("Aged Brie")) {
            new AgedBrieItemUpdater().update(item);
            return;
        }

        if (item.name.equals("Backstage passes to a TAFKAL80ETC concert")) {
            new BackstageItemUpdater().update(item);
            return;
        }

        if (item.name.equals("Sulfuras, Hand of Ragnaros")) {
            new SulfurasItemUpdater().update(item);
            return;
        }

        new NormalItemUpdater().update(item);
    }
}