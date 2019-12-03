package com.gildedrose;

import org.approvaltests.Approvals;
import org.approvaltests.combinations.CombinationApprovals;
import org.junit.jupiter.api.Test;

class GildedRoseTest {

    @Test
    void goldenMaster() {
        String[] names = {
                "foo",
                "Aged Brie",
                "Backstage passes to a TAFKAL80ETC concert",
                "Sulfuras, Hand of Ragnaros"
        };
        Integer[] sellIns = {-1, 0, 10, 11, 12};
        Integer[] qualities = {-1, 0, 1, 49, 50, 51};
        CombinationApprovals.verifyAllCombinations(
                this::updateQuality,
                names,
                sellIns,
                qualities
        );
    }

    @Test
    void conjuredItem() {
        Approvals.verify(updateQuality("Conjured Pot", 2, 10));
    }

    private Item updateQuality(String name, int sellIn, int quality) {
        Item item = new Item(name, sellIn, quality);
        Item[] items = new Item[]{item};
        GildedRose app = new GildedRose(items);
        app.updateQuality();
        return item;
    }

}
