use crate::{format_quote::convert_to_tweet, 
            file_io::select_random_quote};

// #[test]
fn quote_short() {
    let quote = "This is a short quote".to_string();
    let converted_tweet = convert_to_tweet(quote.clone());
    assert_eq!(vec![quote], converted_tweet);
}

// #[test]
fn quote_280_chars() {
    let quote = "This is a quote which has exactly 280 characters. This is an edge case which doesn't really deserve a separate unit test but i'm doing it anyway for the sake of completeness and readability. This still hasn't hit 280 characters but it is about to very soon right at the end of thi".to_string();
    let converted_tweet = convert_to_tweet(quote.clone());
    assert_eq!(vec![quote], converted_tweet);
}

// Edge cases
// There are no full stops
// #[test]
fn quote_long() {
    // Original quote
    let quote = "This is a long quote which needs to be broken into two tweets because there is a 281 character limit per tweet that has been set by Twitter. Once posted we can't edit the tweet either so we need to make sure that the formatting is proper before creating the thread. If you think about it 280 characters is actually a fairly long limit. Earlier Twitter had 140 characters which was tiny. Wonder what data they looked at to make such a major decision of increasing the default character limit to 280.".to_string();

    // Expected tweets 
    let quote_split_one = "This is a long quote which needs to be broken into two tweets because there is a 280 character limit per tweet that has been set by Twitter. Once posted we can't edit the tweet either so we need to make sure that the formatting is proper before creating the thread. ".to_string();
    let quote_split_two = "If you think about it 280 characters is actually a fairly long limit. Earlier Twitter had 140 characters which was tiny. Wonder what data they looked at to make such a major decision of increasing the default character limit to 280.".to_string();

    let converted_tweet = convert_to_tweet(quote.clone());

    assert_eq!(vec![quote_split_one, quote_split_two], converted_tweet);
}

// #[test]
fn quote_long_2() {
    // Original quote
    let quote = "This is a long quote which needs to be broken into two tweets because there is a 280 character limit per tweet that has been set by Twitter. Once posted we can't edit the tweet either so we need to make sure that the formatting is proper before creating the thread. If you think about it 280 characters is actually a fairly long limit. Earlier Twitter had 140 characters which was tiny. Wonder what data they looked at to make such a major decision of increasing the default character limit to 280. Mr. John Johnson Jr. was born in the U.S.A but earned his Ph.D. in Israel before joining Nike Inc. as an engineer. He also worked at craigslist.org as a business analyst.".to_string();

    // Expected tweets 
    let quote_split_one = "This is a long quote which needs to be broken into two tweets because there is a 280 character limit per tweet that has been set by Twitter. Once posted we can't edit the tweet either so we need to make sure that the formatting is proper before creating the thread. ".to_string();
    let quote_split_two = "If you think about it 280 characters is actually a fairly long limit. Earlier Twitter had 140 characters which was tiny. Wonder what data they looked at to make such a major decision of increasing the default character limit to 280. Mr. ".to_string();
    let quote_split_three = "John Johnson Jr. was born in the U.S.A but earned his Ph.D. in Israel before joining Nike Inc. as an engineer. He also worked at craigslist.org as a business analyst.".to_string();

    let converted_tweet = convert_to_tweet(quote.clone());

    assert_eq!(vec![quote_split_one, quote_split_two, quote_split_three], converted_tweet);
}

// Sentence is longer than 280 characters
// #[test]
fn quote_actual() {
    let quote = "A modest understanding of the dynamics of climatic change in past societies could well prove useful in the event that climates continue to fluctuate. If you know that a drop of one degree Centigrade on average reduces the growing season by three to four weeks and shaves five hundred feet off the maximum elevation at which crops can be grown, then you know something about the boundary conditions that will confine people's action in the future. You can use this knowledge to forecast changes in everything from grain prices to land values. You may even be able to draw informed conclusions about the likely impact of falling temperatures on real incomes and political stability. In the past, governments have been overthrown when crop failures extending over several years raised food prices and shrank disposable incomes. For example, it is no coincidence that the seventeenth century, the coldest in the modern period, was also a period of revolution worldwide.".to_string();

    let quote_split_one = "A modest understanding of the dynamics of climatic change in past societies could well prove useful in the event that climates continue to fluctuate. ".to_string();
    let quote_split_two = "If you know that a drop of one degree Centigrade on average reduces the growing season by three to four weeks and shaves five hundred feet off the maximum elevation at which crops can be grown, then you know something about the boundary conditions that will confine people's ".to_string();
    let quote_split_three = "action in the future. You can use this knowledge to forecast changes in everything from grain prices to land values. You may even be able to draw informed conclusions about the likely impact of falling temperatures on real incomes and political stability. ".to_string();
    let quote_split_four = "In the past, governments have been overthrown when crop failures extending over several years raised food prices and shrank disposable incomes. ".to_string();
    let quote_split_five = "For example, it is no coincidence that the seventeenth century, the coldest in the modern period, was also a period of revolution worldwide.".to_string();

    let converted_tweet = convert_to_tweet(quote.clone());

    assert_eq!(vec![quote_split_one, quote_split_two, quote_split_three, 
                    quote_split_four, quote_split_five], converted_tweet);
}

// #[test]
fn quote_actual_2() {
    let quote = "...the more widely dispersed key technologies, the more widely dispersed power will tend to be, and the smaller the optimum scale of government.".to_string();
    let converted_tweet = convert_to_tweet(quote.clone());
    assert_eq!(vec![quote], converted_tweet);
}

// #[test]
#[should_panic]
fn long_word() {
    let quote = "thisisalongwordwhichwontexistinrealtextsbutimjustwritingthistestforthesakeofcompletenessabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefhijklmnopqrstuvwxyzabcdefhijklmnopqrstuvwxyzabcdefhijklmnopqrstuvwxyzabcdefhijklmnopqrstuvwxyzabcdefhijklmnopqrstuvwxyzabcdefhijklmnopqrstuvwxyz".to_string();
    let converted_tweet = convert_to_tweet(quote.clone());
}

#[test]
fn quote_numbered_bullets_file() {
    let path = "testing-data/test5.md";

    let quote_split_one = "Here are some summary points that you should keep in mind as you seek to understand the Information Revolution:\n1. A shift in the megapolitical foundations of power normally unfolds far in advance of the actual revolutions in the use of power.\n".to_string();
    let quote_split_two = "2. Incomes are usually falling when a major transition begins, often because a society has rendered itself crisis-prone by marginalizing resources due to population pressures.\n3. Seeing \"outside\" a system is usually taboo. ".to_string();
    let quote_split_three = "People are frequently blind to the logic of violence in the existing society; therefore, they are almost always blind to changes in that logic, latent or overt. Megapolitical transitions are seldom recognized before they happen.\n".to_string();
    let quote_split_four = "4. Major transitions always involve cultural revolutions, and usually entail clashes between adherents of the old and new values.\n".to_string();
    let quote_split_five = "5. Megapolitical transitions are never popular, because they antiquate painstakingly acquired intellectual capital and confound established moral imperatives. ".to_string();
    let quote_split_six = "They are not undertaken by popular demand, but in response to changes in the external conditions that alter the logic of violence in the local setting.".to_string();

    let quotes = select_random_quote(path);
    let tweet = convert_to_tweet(quotes.to_owned());

    assert_eq!(vec![quote_split_one, quote_split_two, quote_split_three, 
                    quote_split_four, quote_split_five, quote_split_six], tweet);
}
