use scraper::Html;

fn main() {
    let html = reqwest::blocking::get("https://www.puzzle-nurikabe.com/?v=0&size=6")
        .unwrap()
        .text()
        .unwrap();

    let document = Html::parse_document(&html);

    let puzzle_info_string = get_puzzle_info(&document);
    let (_, puzzle_info) = parse::parse_puzzle_info(&puzzle_info_string).unwrap();

    let puzzle_copyright = "Puzzle data from https://www.puzzle-nurikabe.com/".to_string();

    let puzzle_data = get_puzzle_data(&document);

    let filename = puzzle_info.to_filename();
    let contents = format!(
        "# {}\n# {}\n{}",
        puzzle_info_string, puzzle_copyright, puzzle_data
    );

    std::fs::write(
        format!("data/puzzles/puzzle-nurikabe-com/{}", filename),
        contents,
    )
    .unwrap();
}

fn get_puzzle_info(document: &Html) -> String {
    let selector = scraper::Selector::parse(".puzzleInfo").unwrap();
    let puzzle_info = document.select(&selector).next().unwrap();
    puzzle_info.text().collect()
}

fn get_puzzle_data(document: &Html) -> String {
    let table_selector = scraper::Selector::parse("#NurikabeTable").unwrap();
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let table = document.select(&table_selector).next().unwrap();
    let mut puzzle_data = String::new();

    for row in table.select(&tr_selector) {
        for cell in row.select(&td_selector) {
            let cell_text = cell.text().collect::<String>();
            match cell_text.parse::<u32>() {
                Ok(number) => puzzle_data.push_str(&number.to_string()),
                Err(_) => puzzle_data.push_str("."),
            }
        }
        puzzle_data.push_str("\n");
    }

    puzzle_data
}

mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, digit1, space1, u32 as nom_u32};
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair};
    use nom::IResult;

    #[derive(Debug)]
    pub(crate) struct PuzzleInfo {
        num_cols: u32,
        num_rows: u32,
        difficulty: String,
        puzzle_id: u32,
    }

    impl PuzzleInfo {
        pub(crate) fn to_filename(&self) -> String {
            format!(
                "{}x{}-{}-{}.txt",
                self.num_cols,
                self.num_rows,
                self.difficulty.to_ascii_lowercase(),
                self.puzzle_id
            )
        }
    }

    pub(crate) fn parse_puzzle_info(puzzle_info: &str) -> IResult<&str, PuzzleInfo> {
        let (input, (num_cols, num_rows)) =
            separated_pair(nom_u32, char('x'), nom_u32)(puzzle_info)?;
        let (input, difficulty) = preceded(space1, alpha1)(input)?;
        let (input, _) = tag(" Nurikabe Puzzle ID: ")(input)?;
        let (input, puzzle_id) = parse_puzzle_id(input)?;

        Ok((
            input,
            PuzzleInfo {
                num_cols,
                num_rows,
                difficulty: difficulty.to_string(),
                puzzle_id,
            },
        ))
    }

    fn parse_puzzle_id(input: &str) -> IResult<&str, u32> {
        let (input, chunks) = separated_list1(char(','), digit1)(input)?;
        let string = chunks.join("");
        let puzzle_id = string.parse().unwrap();
        Ok((input, puzzle_id))
    }
}
