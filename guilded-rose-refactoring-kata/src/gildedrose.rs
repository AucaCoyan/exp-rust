use std::fmt::{self, Display};
/// Item
/// sell_in value is the number of days we have to sell the item
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
        for i in 0..self.items.len() {
            if self.items[i].name != "Aged Brie"
                && self.items[i].name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if self.items[i].quality > 0 {
                    if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                        self.items[i].quality = self.items[i].quality - 1;
                    }
                }
            } else {
                if self.items[i].quality < 50 {
                    self.items[i].quality = self.items[i].quality + 1;

                    if self.items[i].name == "Backstage passes to a TAFKAL80ETC concert" {
                        if self.items[i].sell_in < 11 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }

                        if self.items[i].sell_in < 6 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }
                    }
                }
            }

            if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                self.items[i].sell_in = self.items[i].sell_in - 1;
            }

            if self.items[i].sell_in < 0 {
                if self.items[i].name != "Aged Brie" {
                    if self.items[i].name != "Backstage passes to a TAFKAL80ETC concert" {
                        if self.items[i].quality > 0 {
                            if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                                self.items[i].quality = self.items[i].quality - 1;
                            }
                        }
                    } else {
                        self.items[i].quality = self.items[i].quality - self.items[i].quality;
                    }
                } else {
                    if self.items[i].quality < 50 {
                        self.items[i].quality = self.items[i].quality + 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};
    use test_case::test_case;

    fn test_update_single_item(item: Item, days: i32) -> Item {
        let single_item = vec![item];
        let mut rose = GildedRose::new(single_item);
        for _ in 1..=days {
            rose.update_quality();
        }
        rose.items.pop().unwrap()
    }

    #[test]
    pub fn new_item_doesnt_change_name() {
        let items = vec![Item::new("foo", 0, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("foo", rose.items[0].name);
    }

    #[test]
    fn test_sulfuras() {
        let sulfuras = vec![Item::new("Sulfuras, Hand of Ragnaros", 0, 0)];
        let mut rose = GildedRose::new(sulfuras);
        rose.update_quality();

        assert_eq!("Sulfuras, Hand of Ragnaros", rose.items[0].name);
    }
    // #[test_case("Aged Brie", 10, 0; "Aged Brie, 10, 0")]

    #[test_case("Not Aged Brie", 10, 1; "Not aged brie, 10, 1")]
    #[test_case("Not Backstage", 10, 1; "Not Backstage, 10, 1")]
    #[test_case("Not Backstage", 10, 10; "Not Backstage, 10, 10")]
    #[test_case("Not Backstage", 10, 100; "Not Backstage, 10, 100")]
    fn test_multiple_cases_qlty_gt_0(name: &str, sell_in: i32, quality: i32) {
        let test_item = test_update_single_item(
            Item {
                name: name.to_string(),
                sell_in,
                quality,
            },
            1,
        );
        dbg!(&test_item.name);
        dbg!(&test_item.sell_in);
        dbg!(&test_item.quality);
        assert_eq!(test_item.name, name.to_string());
        assert_eq!(test_item.sell_in, sell_in - 1);
        assert_eq!(test_item.quality, quality - 1);
    }

    #[test_case("Not Aged Brie", 10, 0; "Not Aged Brie, 10, 0")]
    #[test_case("Not Backstage", 10, 0; "Not Backstage, 10, 0")]
    fn test_multiple_cases_qlty_is_0(name: &str, sell_in: i32, quality: i32) {
        let test_item = test_update_single_item(
            Item {
                name: name.to_string(),
                sell_in,
                quality,
            },
            1,
        );
        dbg!(&test_item.name);
        dbg!(&test_item.sell_in);
        dbg!(&test_item.quality);
        assert_eq!(test_item.name, name.to_string());
        assert_eq!(test_item.sell_in, sell_in - 1);
        assert_eq!(test_item.quality, 0);
    }

    #[test_case("Sulfuras, Hand of Ragnaros", 10, 0; "Sulfuras, 0")]
    #[test_case("Sulfuras, Hand of Ragnaros", 10, 10; "Sulfuras, 10")]
    #[test_case("Sulfuras, Hand of Ragnaros", 10, 80; "Sulfuras, 80")]
    fn test_sulfuras_doesnt_decrease_quality(name: &str, sell_in: i32, quality: i32) {
        let test_item = test_update_single_item(
            Item {
                name: name.to_string(),
                sell_in,
                quality,
            },
            1,
        );
        dbg!(&test_item.name);
        dbg!(&test_item.sell_in);
        dbg!(&test_item.quality);
        assert_eq!(test_item.name, name.to_string());
        assert_eq!(test_item.sell_in, sell_in);
        assert_eq!(test_item.quality, quality);
    }

    // this doesn't work
    //#[test_case("Aged Brie", 10, 0; "Aged Brie, 10, 0")]
    #[test_case("Backstage passes to a TAFKAL80ETC concert", 10, 0; "Backstage, 10, 0")]
    fn test_multiple_cases_items_qlty_increases_below_10(name: &str, sell_in: i32, quality: i32) {
        let test_item = test_update_single_item(
            Item {
                name: name.to_string(),
                sell_in,
                quality,
            },
            1,
        );
        dbg!(&test_item.name);
        dbg!(&test_item.sell_in);
        dbg!(&test_item.quality);
        assert_eq!(test_item.name, name.to_string());
        assert_eq!(test_item.sell_in, sell_in - 1);
        assert_eq!(test_item.quality, quality + 2);
    }

    // this doesn't work
    #[test_case("Aged Brie", 5, 0; "Aged Brie, 5, 0")]
    #[test_case("Aged Brie", 5, 1; "Aged Brie, 5, 1")]
    #[test_case("Aged Brie", 5, 20; "Aged Brie, 5, 20")]
    #[test_case("Aged Brie", 5, 30; "Aged Brie, 5, 30")]
    fn test_brie_qlty_increases_with_time(name: &str, sell_in: i32, quality: i32) {
        let test_item = test_update_single_item(
            Item {
                name: name.to_string(),
                sell_in,
                quality,
            },
            1,
        );
        dbg!(&test_item.name);
        dbg!(&test_item.sell_in);
        dbg!(&test_item.quality);
        assert_eq!(test_item.name, name.to_string());
        assert_eq!(test_item.sell_in, sell_in - 1);
        assert_eq!(test_item.quality, quality + 1);
    }

    // this doesn't work
    //#[test_case("Aged Brie", 5, 0; "Aged Brie, 5, 0")]
    #[test_case("Backstage passes to a TAFKAL80ETC concert", 5, 0; "Backstage, 5, 0")]
    fn test_multiple_cases_items_qlty_increases_below_5(name: &str, sell_in: i32, quality: i32) {
        let test_item = test_update_single_item(
            Item {
                name: name.to_string(),
                sell_in,
                quality,
            },
            1,
        );
        dbg!(&test_item.name);
        dbg!(&test_item.sell_in);
        dbg!(&test_item.quality);
        assert_eq!(test_item.name, name.to_string());
        assert_eq!(test_item.sell_in, sell_in - 1);
        assert_eq!(test_item.quality, quality + 3);
    }

    #[test]
    fn test_not_backstage_qlty_gt_0() {
        let test_item = test_update_single_item(
            Item {
                name: "Not backstage".to_string(),
                sell_in: 10,
                quality: 1,
            },
            1,
        );
        assert_eq!(test_item.name, "Not backstage".to_string());
        assert_eq!(test_item.sell_in, 9);
        assert_eq!(test_item.quality, 0);
    }

    #[ignore = "this doesn't work"]
    #[test]
    #[should_panic]
    fn test_creation_item() {
        // no item should have a quality above 50 (see below)
        Item::new("something", 100, 51);
    }

    #[ignore = "this doesn't work"]
    #[test]
    #[should_panic]
    fn test_creation_sulfuras() {
        // except Sulfuras, which is exactly 80
        Item::new("Sulfuras, Hand of Ragnaros", 100, 81);
    }
}
