pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = vec![];

    if !list.is_empty() {
        for index in 0..list.len() - 1 {
            proverb.push(format!(
                "For want of a {item} the {next} was lost.",
                item = list[index],
                next = list[index + 1]
            ));
        }
        proverb.push(format!("And all for the want of a {item}.", item = list[0]));
    }
    proverb.join("\n")
}
