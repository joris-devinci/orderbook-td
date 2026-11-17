// ============================================================================
// REFERENCE IMPLEMENTATION (Naive, for correctness comparison)
// ============================================================================

use crate::interfaces::{OrderBook, Price, Quantity, Side, Update, Item};
use min_heap::MinHeap;
use std::collections::BinaryHeap;

pub struct OrderBookImpl {
    bid_heap: BinaryHeap<Item>,
    ask_heap: MinHeap<Item>,
    total_ask_quantity: Quantity,
    total_bid_quantity: Quantity,
}

impl OrderBook for OrderBookImpl {
    fn new() -> Self {
        Self {
            bid_heap: BinaryHeap::new(),
            ask_heap: MinHeap::new(),
            total_ask_quantity: 0,
            total_bid_quantity: 0
        }
    }

    fn apply_update(&mut self, update: Update) {
        match update {
            Update::Set { price, quantity, side } => {
                if side == Side::Bid {
                    self.bid_heap.push(Item { price, quantity });
                    self.total_bid_quantity += quantity;
                }
                else {
                    self.ask_heap.push(Item { price, quantity });
                    self.total_ask_quantity += quantity;
                }
            }
            Update::Remove { price, side } => {
                match side {
                    Side::Bid => {
                        let mut temp_heap = BinaryHeap::new();

                        while self.bid_heap.len() > 0 {
                            let item: Item = self.bid_heap.pop().expect("msg");

                            if item.price == price {
                                self.total_bid_quantity -= item.quantity;

                                self.bid_heap.append(&mut temp_heap);
                                return;
                            }
                            temp_heap.push(item);
                        }
                        self.bid_heap.append(&mut temp_heap);
                    }
                    Side::Ask => {
                        let mut temp_heap = MinHeap::new();

                        while self.ask_heap.len() > 0 {
                            let item: Item = self.ask_heap.pop().expect("msg");

                            if item.price == price {
                                self.total_bid_quantity -= item.quantity;

                                self.ask_heap.append(&mut temp_heap);
                                return;
                            }
                            temp_heap.push(item);
                        }
                        self.ask_heap.append(&mut temp_heap);
                    }
                }
            }
        }
    }

    fn get_spread(&self) -> Option<Price> {
        let ask_price: Price = self.get_best_ask().expect("Get best ask error");
        let bid_price: Price = self.get_best_bid().expect("Get best bid error");

        return Some(ask_price - bid_price);
    }

    fn get_best_bid(&self) -> Option<Price> {
        let best_bid = self.bid_heap.peek().expect("Peek went wrong").price;
        return Some(best_bid);
    }

    fn get_best_ask(&self) -> Option<Price> {
        let best_ask = self.ask_heap.peek().expect("Peak went wrong").price;
        return Some(best_ask);
    }

    fn get_quantity_at(&self, price: Price, side: Side) -> Option<Quantity> {
        match side {
        Side::Ask => {
                let mut cloned_heap = self.ask_heap.clone();

                while let Some(item) = cloned_heap.pop() {
                    if item.price == price {
                        return Some(item.quantity);
                    }
                }
            }
            Side::Bid => {
                let mut cloned_heap = self.bid_heap.clone();

                while let Some(item) = cloned_heap.pop() {
                    if item.price == price {
                        return Some(item.quantity);
                    }
                }
            }
        }
        None
    }

 fn get_top_levels(&self, side: Side, n: usize) -> Vec<(Price, Quantity)> {
    let mut top_levels: Vec<(Price, Quantity)> = vec![];

    match side {
        Side::Ask => {
            let mut cloned_heap = self.ask_heap.clone();

            for _ in 0..n {
                match cloned_heap.pop() {
                    Some(item) => top_levels.push((item.price, item.quantity)),
                    None => break,
                }
            }
        }
        Side::Bid => {
            let mut cloned_heap = self.bid_heap.clone();

            for _ in 0..n {
                match cloned_heap.pop() {
                    Some(item) => top_levels.push((item.price, item.quantity)),
                    None => break,
                }
            }
        }
    }
    top_levels
}

    fn get_total_quantity(&self, side: Side) -> Quantity {
        match side {
            Side::Ask => self.total_ask_quantity,
            Side::Bid => self.total_bid_quantity,
        }
    }
}
