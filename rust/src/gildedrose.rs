use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in self.items.iter_mut() {
            match item.name.as_str() {
                "Aged Brie" => {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;
                    }

                    item.sell_in = item.sell_in - 1;

                    if item.sell_in < 0 {
                        if item.quality < 50 {
                            item.quality = item.quality + 1;
                        }
                    }
                },
                "Backstage passes to a TAFKAL80ETC concert" => {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;

                        if item.sell_in < 11 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }

                        if item.sell_in < 6 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }
                    }
                    item.sell_in = item.sell_in - 1;

                    if item.sell_in < 0 {
                        item.quality = item.quality - item.quality;
                    }
                },
                "Sulfuras, Hand of Ragnaros" => (),
                _ => {
                    if item.quality > 0 {
                        item.quality = item.quality - 1;
                    }

                    item.sell_in = item.sell_in - 1;

                    if item.sell_in < 0 {
                        if item.quality > 0 {
                            item.quality = item.quality - 1;
                        }   
                    }
                },
            }
        }
    }


}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn test_update_quality() {
        let items = vec![
            Item::new("foo", -1, -1),
            Item::new("another_item", -1, 1),
            Item::new("Aged Brie", -1, 1),
            Item::new("Backstage passes to a TAFKAL80ETC concert", -1, 1),
            Item::new("another_item", -1, 49),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 12, 1),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 12, 50),
            Item::new("Sulfuras, Hand of Ragnaros", -1, 1),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 9, 49),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 49),
            Item::new("Aged Brie", -1, 51),
            Item::new("foo", -1, 0),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 11, 11),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 9, 2),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 6, 11),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 11),
            Item::new("Aged Brie", 1, 1),
            Item::new("Aged Brie", 0, 50),
            Item::new("foo", 2, 10),
        ];
        let mut rose = GildedRose::new(items.clone());
        rose.update_quality();

        insta::assert_snapshot!(items
            .iter()
            .zip(rose.items.iter())
            .map(|(before, after)| { format!("{before} => {after}") })
            .collect::<Vec<_>>()
            .join("\n"));
    }
}
