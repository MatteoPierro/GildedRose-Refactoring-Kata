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
        updaterFor(item).update(item);
    }

    private ItemUpdater updaterFor(Item item) {
        if (item.name.equals("Aged Brie")) {
            return new AgedBrieItemUpdater();
        }

        if (item.name.equals("Backstage passes to a TAFKAL80ETC concert")) {
            return new BackstageItemUpdater();
        }

        if (item.name.equals("Sulfuras, Hand of Ragnaros")) {
            return new SulfurasItemUpdater();
        }

        if (item.name.contains("Conjured")) {
            return new ConjuredItemUpdater();
        }

        return new NormalItemUpdater();
    }
}