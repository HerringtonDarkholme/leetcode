#[derive(Clone, Debug)]
struct Transaction<'a> {
    amount: i32,
    time: i32,
    name: &'a str,
    city: &'a str,
}

fn parse<'a>(source: &'a str) -> Option<Transaction<'a>> {
    let mut splits = source.split(',');
    let name = splits.next()?;
    let time = splits.next()?.parse().ok()?;
    let amount = splits.next()?.parse().ok()?;
    let city = splits.next()?;
    Some(Transaction {
        name,
        time,
        amount,
        city,
    })
}

fn to_string(t: Transaction) -> String {
    format!("{},{},{},{}", t.name, t.time, t.amount, t.city)
}
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut invalid = vec![];
        let mut group_by_name = HashMap::new();
        for i in 0..transactions.len() {
            let transaction = parse(&transactions[i]).unwrap();
            group_by_name.entry(transaction.name).or_insert_with(|| vec![]).push(transaction);
        }
        for trans in group_by_name.into_values() {
            let mut set = HashSet::new();
            for i in 0..trans.len() {
                if trans[i].amount > 1000 {
                    set.insert(i);
                }
                for j in i+1..trans.len() {
                    if (trans[i].time - trans[j].time).abs() <= 60 && trans[i].city != trans[j].city {
                        set.insert(i);
                        set.insert(j);
                    }
                }
            }
            for i in set {
                invalid.push(trans[i].clone());
            }
        }
        invalid.into_iter().map(to_string).collect()
    }
}
