use std::fmt::{self, Display};
/// Item
/// `sell_in` value is the number of days we have to sell the item
#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: &str, sell_in: i32, quality: i32) -> Item {
        assert!(
            quality > 0,
            "An item with negative quality cannot be created!"
        );
        if name == SULFURAS {
            assert!(quality == 80, "Sulfuras can only have quality equal to 80");
        } else {
            assert!(
                quality <= 50,
                "Normal item cannot be created with quality greater than 50"
            );
        }

        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Item {
    pub fn can_decrease_quality(self) -> bool {
        self.quality > 0
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

const SULFURAS: &str = "Sulfuras, Hand of Ragnaros";
const BRIE: &str = "Aged Brie";
const CONCERT: &str = "Backstage passes to a TAFKAL80ETC concert";

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name != BRIE && item.name != CONCERT {
                if item.quality > 0 && item.name != SULFURAS && item.clone().can_decrease_quality()
                {
                    item.quality -= 1;
                }
            } else if item.quality < 50 {
                item.quality += 1;

                if item.name == CONCERT {
                    if item.sell_in < 11 && item.quality < 50 {
                        item.quality += 1;
                    }

                    if item.sell_in < 6 && item.quality < 50 {
                        item.quality += 1;
                    }
                }
            }

            if item.name != SULFURAS {
                item.sell_in -= 1;
            }

            if item.sell_in < 0 {
                if item.name == BRIE {
                    if item.quality < 50 {
                        item.quality += 1;
                    }
                } else if item.name != CONCERT {
                    if item.quality > 0 && item.name != SULFURAS {
                        item.quality -= 1;
                    }
                } else {
                    item.quality = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gildedrose::{BRIE, CONCERT, SULFURAS};

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
        let items = vec![Item::new("foo", 0, 1)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("foo", rose.items[0].name);
    }

    #[test]
    fn test_sulfuras_name() {
        let sulfuras = vec![Item::new(SULFURAS, 0, 80)];
        let mut rose = GildedRose::new(sulfuras);
        rose.update_quality();

        assert_eq!(SULFURAS, rose.items[0].name);
    }
    // #[test_case("Aged Brie", 10, 0; "Aged Brie, 10, 0")]

    #[test_case("Not Aged Brie", 10, 1; "Not aged brie, 10, 1")]
    #[test_case("Not Backstage", 10, 1; "Not Backstage, 10, 1")]
    #[test_case("Not Backstage", 10, 10; "Not Backstage, 10, 10")]
    #[test_case("Not Backstage", 10, 100; "Not Backstage, 10, 100")]
    fn test_items_with_qlty_gt_0(name: &str, sell_in: i32, quality: i32) {
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
    fn test_items_with_qlty_eq_0(name: &str, sell_in: i32, quality: i32) {
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

    #[test_case(SULFURAS, 10, 0; "Sulfuras, 0")]
    #[test_case(SULFURAS, 10, 10; "Sulfuras, 10")]
    #[test_case(SULFURAS, 10, 80; "Sulfuras, 80")]
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

    #[test_case(CONCERT, 10, 0; "Backstage, 10, 0")]
    fn test_backstage_qlty_increases_below_10(name: &str, sell_in: i32, quality: i32) {
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

    #[test_case(CONCERT, 5, 0; "Backstage, 5, 0")]
    fn test_backstage_qlty_increases_below_5(name: &str, sell_in: i32, quality: i32) {
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

    #[ignore = "this gives 4 but should be quality 0"]
    #[test_case(CONCERT, 5, 10; "Backstage, 5, 0")]
    fn test_backstage_qlty_drops_to_0(name: &str, sell_in: i32, quality: i32) {
        let test_item = test_update_single_item(
            Item {
                name: name.to_string(),
                sell_in,
                quality,
            },
            5,
        );
        dbg!(&test_item.name);
        dbg!(&test_item.sell_in);
        dbg!(&test_item.quality);
        assert_eq!(test_item.name, name.to_string());
        assert_eq!(test_item.sell_in, sell_in - 1);
        assert_eq!(test_item.quality, 0);
    }

    // this doesn't work
    #[test_case(BRIE, 5, 0; "Aged Brie, 5, 0")]
    #[test_case(BRIE, 5, 1; "Aged Brie, 5, 1")]
    #[test_case(BRIE, 5, 20; "Aged Brie, 5, 20")]
    #[test_case(BRIE, 5, 30; "Aged Brie, 5, 30")]
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

    #[test_case("Old Item", 5, 0, 0; "Aged Brie, 5, 0")]
    #[test_case("Old Item", 5, 1, 0; "Aged Brie, 5, 1")]
    #[test_case("Old Item", 5, 20, 5; "Aged Brie, 5, 20")]
    #[test_case("Old Item", 5, 30, 15; "Aged Brie, 5, 30")]
    fn test_drop_quality_faster_with_sell_in_lt_0(
        name: &str,
        sell_in: i32,
        quality: i32,
        result: i32,
    ) {
        let test_item = test_update_single_item(
            Item {
                name: name.to_string(),
                sell_in,
                quality,
            },
            10,
        );
        assert_eq!(test_item.name, "Old Item".to_string());
        assert_eq!(test_item.sell_in, -5);
        assert_eq!(test_item.quality, result);
    }

    #[test]
    fn test_drop_quality_below_0() {
        let test_item = test_update_single_item(
            Item {
                name: "Broken Item".to_string(),
                sell_in: 10,
                quality: 3,
            },
            10,
        );
        assert_eq!(test_item.name, "Broken Item".to_string());
        assert_eq!(test_item.sell_in, 0);
        assert_eq!(test_item.quality, 0);
    }

    #[test]
    #[should_panic(expected = "An item with negative quality cannot be created!")]
    fn test_create_item_with_neg_quality() {
        // quality of an item can be never negative
        Item::new("Broken item", 100, -10);
    }

    #[test]
    #[should_panic(expected = "Normal item cannot be created with quality greater than 50")]
    fn test_create_item_with_quality_gt_50() {
        // no item should have a quality above 50 (see below)
        Item::new("something", 100, 51);
    }

    #[test]
    #[should_panic(expected = "Sulfuras can only have quality equal to 80")]
    fn test_create_sulfuras_with_qlty_gt_50() {
        // except Sulfuras, which is exactly 80
        Item::new(SULFURAS, 100, 81);
    }
}
