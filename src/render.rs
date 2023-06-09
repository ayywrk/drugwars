use ircie::format::{Color, Msg};
use itertools::Itertools;
use rand::{seq::SliceRandom, RngCore};

use crate::{
    dealer::Dealer,
    location_data::{PriceTrend, SingleLocationData},
    renderer::{RenderBox, RenderBoxContent, Renderer},
    resources::{Location, Locations, Messages},
    utils::{get_flight_price, PrettyAmount, PrettyMoney, StringManips},
};

pub fn render_info(dealer: &Dealer) -> Vec<String> {
    Renderer::new(50)
        .add_box(
            &RenderBox::new()
                .headers(["Dealer Info".to_owned()])
                .add_content([&RenderBoxContent::new()
                    .sizes([18, 25])
                    .add_row(["nick".to_owned(), dealer.nick.to_owned()])
                    .add_row(["health".to_owned(), format!("{:.2} hp", dealer.health)])
                    .add_row(["dirty money".to_owned(), dealer.money.pretty_money()])
                    .add_row([
                        "money laundered".to_owned(),
                        dealer.laundered_money.pretty_money(),
                    ])
                    .add_row(["location".to_owned(), dealer.location.name.clone()])
                    .add_row(["capacity".to_owned(), dealer.capacity.pretty_amount()])
                    .add_row(["status".to_owned(), dealer.status.pretty()])
                    .get()])
                .get(),
        )
        .build()
}

pub fn render_help() -> Vec<String> {
    Renderer::new(90)
        .add_box(
            &RenderBox::new()
                .headers(["Command list".to_owned()])
                .add_content([&RenderBoxContent::new()
                    .add_row(["register".to_owned(), "join the game".to_owned()])
                    .add_row(["h".to_owned(), "print this list".to_owned()])
                    .add_row(["ha".to_owned(), "print the admin command list".to_owned()])
                    .add_row(["i".to_owned(), "print your info".to_owned()])
                    .add_row(["m".to_owned(), "print the market".to_owned()])
                    .add_row([
                        "p".to_owned(),
                        "show the people at your location".to_owned(),
                    ])
                    .add_row(["t".to_owned(), "print the date and time".to_owned()])
                    .add_row([
                        "a <target> <weapon>".to_owned(),
                        "attack someone".to_owned(),
                    ])
                    .add_row(["l <target>".to_owned(), "loot a dead player".to_owned()])
                    .add_row(["lm <money>".to_owned(), "launder your money".to_owned()])
                    .add_row([
                        "leaderboard".to_owned(),
                        "show the hardest dealers".to_owned(),
                    ])
                    .add_row([
                        "heal".to_owned(),
                        "heal completely for a third of your money".to_owned(),
                    ])
                    .add_row([
                        "bt <amount>".to_owned(),
                        "buy thugs (cost 10,000 / day)".to_owned(),
                    ])
                    .add_row(["st <amount>".to_owned(), "sell thugs".to_owned()])
                    .add_row([
                        "bd <drug> <amount>".to_owned(),
                        "buy drug from market".to_owned(),
                    ])
                    .add_row([
                        "sd <drug> <amount>".to_owned(),
                        "sell drug to market".to_owned(),
                    ])
                    .add_row([
                        "bi <drug> <amount>".to_owned(),
                        "buy item from market".to_owned(),
                    ])
                    .add_row([
                        "si <drug> <amount>".to_owned(),
                        "sell item to market".to_owned(),
                    ])
                    .add_row(["bc <amount>".to_owned(), "buy inventory slots".to_owned()])
                    .add_row([
                        "cc <amount>".to_owned(),
                        "check price to add <amount> inventory slots".to_owned(),
                    ])
                    .add_row(["cf ".to_owned(), "check flight prices".to_owned()])
                    .add_row([
                        "f <destination>".to_owned(),
                        "fly to destination".to_owned(),
                    ])
                    .add_row([
                        "cshd <drug> <amount> <destination>".to_owned(),
                        "check drug shipping price".to_owned(),
                    ])
                    .add_row([
                        "cshi <drug> <amount> <destination>".to_owned(),
                        "check item shipping price".to_owned(),
                    ])
                    .add_row([
                        "shd <drug> <amount> <destination>".to_owned(),
                        "ship drug to destination".to_owned(),
                    ])
                    .add_row([
                        "shi <item> <amount> <destination>".to_owned(),
                        "ship item to destination".to_owned(),
                    ])
                    .add_row([
                        "gm <bloke> <amount>".to_owned(),
                        "give money to some bloke".to_owned(),
                    ])
                    .add_row([
                        "gd <bloke> <drug> <amount>".to_owned(),
                        "give drugs to some bloke".to_owned(),
                    ])
                    .add_row([
                        "gi <bloke> <item> <amount>".to_owned(),
                        "give items to some bloke".to_owned(),
                    ])
                    .get()])
                .get(),
        )
        .build()
}

pub fn render_admin_help() -> Vec<String> {
    Renderer::new(90)
        .add_box(
            &RenderBox::new()
                .headers(["Command list".to_owned()])
                .add_content([&RenderBoxContent::new()
                    .add_row(["save".to_owned(), "save the game".to_owned()])
                    .add_row(["dealers".to_owned(), "show all dealers".to_owned()])
                    .add_row(["ff".to_owned(), "advance to next day".to_owned()])
                    .get()])
                .get(),
        )
        .build()
}

pub fn render_market(
    width: usize,
    mut rng: &mut dyn RngCore,
    nick: &str,
    dealer: &Dealer,
    location: &SingleLocationData,
    messages: &Messages,
) -> Vec<String> {
    let mut renderer = Renderer::new(width);

    let drugs_owned = dealer.owned_drugs.get(&dealer.location).unwrap();
    let items_owned = dealer.owned_items.get(&dealer.location).unwrap();

    let mut rumor_content = RenderBoxContent::<1>::new();

    for rumor in &location.rumors {
        if rumor.confirmed.is_none() {
            let mut msg = Msg::new()
                .color(Color::Cyan)
                .text("You hear a rumor that ")
                .color(Color::Yellow)
                .text(&rumor.drug.name)
                .color(Color::Cyan);
            msg = match rumor.trend {
                PriceTrend::Up => msg.text(" will be abundant in "),
                PriceTrend::Down => msg.text(" will be scarce in "),
            };

            msg = msg
                .color(Color::Purple)
                .text(&rumor.location.name)
                .color(Color::Cyan)
                .text(" tomorrow.");

            rumor_content.add_row([msg.to_string()]);
        }
    }

    for price_mod in &location.price_mods {
        match price_mod.trend {
            PriceTrend::Up => {
                let mut message = messages
                    .get("price_up")
                    .unwrap()
                    .choose(&mut rng)
                    .unwrap()
                    .to_owned()
                    + " "
                    + messages
                        .get("price_up_end")
                        .unwrap()
                        .choose(&mut rng)
                        .unwrap()
                        .as_str();

                let colored_drug = Msg::new()
                    .color(Color::Yellow)
                    .text(&price_mod.drug.name)
                    .color(Color::Green);

                message = message.replace("%DRUG", &colored_drug.to_string());

                let msg = Msg::new().color(Color::Green).text(&message).reset();
                rumor_content.add_row([msg.to_string()]);
            }

            PriceTrend::Down => {
                let mut message = messages
                    .get("price_down")
                    .unwrap()
                    .choose(&mut rng)
                    .unwrap()
                    .to_owned()
                    + " "
                    + messages
                        .get("price_down_end")
                        .unwrap()
                        .choose(&mut rng)
                        .unwrap()
                        .as_str();

                let colored_drug = Msg::new()
                    .color(Color::Yellow)
                    .text(&price_mod.drug.name)
                    .color(Color::Orange);

                message = message.replace("%DRUG", &colored_drug.to_string());

                let msg = Msg::new().color(Color::Orange).text(&message).reset();
                rumor_content.add_row([msg.to_string()]);
            }
        };
    }
    let rumor_content = rumor_content.get();

    let mut drugs_market_content = RenderBoxContent::new();
    drugs_market_content
        .header([
            "Drug".to_owned(),
            "Supply".to_owned(),
            "Demand".to_owned(),
            "Price".to_owned(),
        ])
        .sizes([18, 10, 10, 19]);

    let mut drugs_owned_content = RenderBoxContent::new();
    drugs_owned_content
        .header([
            "Drug".to_owned(),
            "Amount".to_owned(),
            "Bought at".to_owned(),
        ])
        .sizes([18, 10, 25]);

    for pair in location.drug_market.iter().zip_longest(drugs_owned.iter()) {
        match pair {
            itertools::EitherOrBoth::Both(market, owned) => {
                let market_drug_name = match drugs_owned.contains_key(market.0) {
                    true => Msg::new()
                        .color(Color::Cyan)
                        .text(&market.0.name)
                        .reset()
                        .to_string(),
                    false => market.0.name.to_owned(),
                };

                let owned_drug_name = match location.drug_market.contains_key(owned.0) {
                    true => Msg::new()
                        .color(Color::Cyan)
                        .text(&owned.0.name)
                        .reset()
                        .to_string(),
                    false => owned.0.name.to_owned(),
                };

                let mut msg = Msg::new();
                msg = if market.1.price >= market.0.nominal_price {
                    msg.color(Color::Green)
                        .text("↗ ")
                        .text(market.1.price.pretty_money())
                } else {
                    msg.color(Color::Red)
                        .text("↘ ")
                        .text(market.1.price.pretty_money())
                };

                msg = msg.reset();

                drugs_market_content.add_row([
                    market_drug_name,
                    market.1.supply.pretty_amount(),
                    market.1.demand.pretty_amount(),
                    msg.to_string(),
                ]);

                drugs_owned_content.add_row([
                    owned_drug_name,
                    owned.1.amount.pretty_amount(),
                    owned.1.bought_at.pretty_money(),
                ]);
            }
            itertools::EitherOrBoth::Left(market) => {
                let market_drug_name = match drugs_owned.contains_key(market.0) {
                    true => Msg::new()
                        .color(Color::Cyan)
                        .text(&market.0.name)
                        .reset()
                        .to_string(),
                    false => market.0.name.to_owned(),
                };

                let mut msg = Msg::new();
                msg = if market.1.price >= market.0.nominal_price {
                    msg.color(Color::Green)
                        .text("↗ ")
                        .text(market.1.price.pretty_money())
                } else {
                    msg.color(Color::Red)
                        .text("↘ ")
                        .text(market.1.price.pretty_money())
                };

                msg = msg.reset();

                drugs_market_content.add_row([
                    market_drug_name,
                    market.1.supply.pretty_amount(),
                    market.1.demand.pretty_amount(),
                    msg.to_string(),
                ]);
            }
            itertools::EitherOrBoth::Right(owned) => {
                let owned_drug_name = match location.drug_market.contains_key(owned.0) {
                    true => Msg::new()
                        .color(Color::Cyan)
                        .text(&owned.0.name)
                        .reset()
                        .to_string(),
                    false => owned.0.name.to_owned(),
                };

                drugs_owned_content.add_row([
                    owned_drug_name,
                    owned.1.amount.pretty_amount(),
                    owned.1.bought_at.pretty_money(),
                ]);
            }
        }
    }
    let drugs_market_content = drugs_market_content.get();
    let drugs_owned_content = drugs_owned_content.get();

    let mut items_market_content = RenderBoxContent::new();
    items_market_content
        .header([
            "Item".to_owned(),
            "Supply".to_owned(),
            "Demand".to_owned(),
            "Price".to_owned(),
        ])
        .sizes([18, 10, 10, 19]);

    let mut items_owned_content = RenderBoxContent::new();
    items_owned_content
        .header([
            "Item".to_owned(),
            "Amount".to_owned(),
            "Bought at".to_owned(),
        ])
        .sizes([18, 10, 25]);

    for pair in location.item_market.iter().zip_longest(items_owned.iter()) {
        match pair {
            itertools::EitherOrBoth::Both(market, owned) => {
                let market_item_name = match items_owned.contains_key(market.0) {
                    true => Msg::new()
                        .color(Color::Cyan)
                        .text(&market.0.name)
                        .reset()
                        .to_string(),
                    false => market.0.name.to_owned(),
                };

                let owned_item_name = match location.item_market.contains_key(owned.0) {
                    true => Msg::new()
                        .color(Color::Cyan)
                        .text(&owned.0.name)
                        .reset()
                        .to_string(),
                    false => owned.0.name.to_owned(),
                };

                let mut msg = Msg::new();
                msg = if market.1.price >= market.0.nominal_price {
                    msg.color(Color::Green)
                        .text("↗ ")
                        .text(market.1.price.pretty_money())
                } else {
                    msg.color(Color::Red)
                        .text("↘ ")
                        .text(market.1.price.pretty_money())
                };

                msg = msg.reset();

                items_market_content.add_row([
                    market_item_name,
                    market.1.supply.pretty_amount(),
                    market.1.demand.pretty_amount(),
                    msg.to_string(),
                ]);

                items_owned_content.add_row([
                    owned_item_name,
                    owned.1.amount.pretty_amount(),
                    owned.1.bought_at.pretty_money(),
                ]);
            }
            itertools::EitherOrBoth::Left(market) => {
                let market_item_name = match items_owned.contains_key(market.0) {
                    true => Msg::new()
                        .color(Color::Cyan)
                        .text(&market.0.name)
                        .reset()
                        .to_string(),
                    false => market.0.name.to_owned(),
                };

                let mut msg = Msg::new();
                msg = if market.1.price >= market.0.nominal_price {
                    msg.color(Color::Green)
                        .text("↗ ")
                        .text(market.1.price.pretty_money())
                } else {
                    msg.color(Color::Red)
                        .text("↘ ")
                        .text(market.1.price.pretty_money())
                };

                msg = msg.reset();

                items_market_content.add_row([
                    market_item_name,
                    market.1.supply.pretty_amount(),
                    market.1.demand.pretty_amount(),
                    msg.to_string(),
                ]);
            }
            itertools::EitherOrBoth::Right(owned) => {
                let owned_item_name = match location.item_market.contains_key(owned.0) {
                    true => Msg::new()
                        .color(Color::Cyan)
                        .text(&owned.0.name)
                        .reset()
                        .to_string(),
                    false => owned.0.name.to_owned(),
                };

                items_owned_content.add_row([
                    owned_item_name,
                    owned.1.amount.pretty_amount(),
                    owned.1.bought_at.pretty_money(),
                ]);
            }
        }
    }
    let items_market_content = items_market_content.get();
    let items_owned_content = items_owned_content.get();

    let rumor_box = RenderBox::new()
        .headers([format!(
            "{} ─ {} ─ {} ─ {} ─ {}",
            nick,
            format!("{:.2} hp", dealer.health),
            dealer.money.pretty_money(),
            dealer.location.name,
            dealer.status.pretty()
        )])
        .add_content([&rumor_content])
        .get();

    let drugs_box = RenderBox::new()
        .headers([
            "Drug market".to_owned(),
            format!(
                "Owned drugs ({}/{})",
                //pretty_print_amount(dealer.get_total_owned_local::<Drug>()),
                0.pretty_amount(),
                dealer.capacity.pretty_amount(),
            ),
        ])
        .add_content([&drugs_market_content, &drugs_owned_content])
        .get();

    let items_box = RenderBox::new()
        .headers([
            "Item market".to_owned(),
            format!(
                "Owned items ({}/{})",
                //pretty_print_amount(dealer.get_total_owned_local::<Item>()),
                0.pretty_amount(),
                dealer.capacity.pretty_amount(),
            ),
        ])
        .add_content([&items_market_content, &items_owned_content])
        .get();

    renderer
        .add_box(&rumor_box)
        .add_box(&drugs_box)
        .add_box(&items_box);

    renderer.build()
}

pub fn render_people(width: usize, location: &SingleLocationData) -> Vec<String> {
    let mut blokes = location.people.iter().collect::<Vec<_>>();
    let mut line = String::new();

    let mut blokes_content = RenderBoxContent::new();

    while blokes.len() > 0 {
        let to_append = format!("{}, ", blokes[blokes.len() - 1]);

        if line.irc_safe_len() + to_append.irc_safe_len() > width - 2 {
            line.truncate(line.len() - 2);
            blokes_content.add_row([line]);

            line = String::new();
        }

        line += &to_append;
        blokes.pop();
    }

    if line.irc_safe_len() > 0 {
        line.truncate(line.len() - 2);
        blokes_content.add_row([line]);
    }
    let blokes_content = blokes_content.get();

    Renderer::new(width)
        .add_box(
            &RenderBox::new()
                .headers(["People in town".to_owned()])
                .add_content([&blokes_content])
                .get(),
        )
        .build()
}

pub fn render_prices_from(current_location: &Location, locations: &Locations) -> Vec<String> {
    let mut flight_prices_content = RenderBoxContent::new();

    flight_prices_content
        .header(["To".to_owned(), "Price".to_owned()])
        .sizes([30, 15]);

    for location in locations.iter() {
        if location.name == current_location.name {
            continue;
        }

        let price = get_flight_price(current_location, location);

        let to = Msg::new()
            .color(Color::Yellow)
            .text(&location.name)
            .reset()
            .to_string();
        let p_price = Msg::new()
            .color(Color::Green)
            .text(price.pretty_money())
            .reset()
            .to_string();

        flight_prices_content.add_row([to, p_price]);
    }

    Renderer::new(50)
        .add_box(
            &RenderBox::new()
                .headers([format!("Flight prices from {}", &current_location.name)])
                .add_content([&flight_prices_content.get()])
                .get(),
        )
        .build()
}

/*
impl DrugWars {

    pub fn render_leaderboard(&self) -> Vec<String> {
        let dealers = &self
            .dealers
            .iter()
            .sorted_by_key(|(_, k)| k.laundered_money)
            .rev()
            .enumerate()
            .collect::<Vec<_>>();

        let min = dealers.len().min(5);

        let dealers = &dealers[0..min];

        let mut leaderboard_content = RenderBoxContent::new();
        leaderboard_content
            .header([
                "Dealer".to_owned(),
                "Place".to_owned(),
                "Laundered money".to_owned(),
            ])
            .sizes([12, 8, 25]);

        for (idx, (name, dealer)) in dealers {
            let mut msg = PrivMsg::new();
            let msg = msg
                .color(IrcColor::Green)
                .text(&pretty_print_money(dealer.laundered_money))
                .reset()
                .get();

            leaderboard_content.add_row([
                name.to_owned().clone(),
                (idx + 1).to_string(),
                msg.to_owned(),
            ]);
        }

        Renderer::new(50)
            .add_box(
                &RenderBox::new()
                    .headers(["Top 5 hardest dealers".to_owned()])
                    .add_content([&leaderboard_content])
                    .get(),
            )
            .build()
    }
}
 */
