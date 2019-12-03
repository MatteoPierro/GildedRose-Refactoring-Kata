package com.gildedrose;

public class NormalItemUpdater implements ItemUpdater {

    private final int degrade;

    public NormalItemUpdater() {
        this(1);
    }

    protected NormalItemUpdater(int degrade) {
        this.degrade = degrade;
    }

    @Override
    public void update(Item item) {
        if (item.quality > 0) {
            item.quality = item.quality - degrade;
        }

        item.sellIn = item.sellIn - 1;

        if (item.sellIn < 0) {
            if (item.quality > 0) {
                item.quality = item.quality - degrade;
            }
        }
    }
}