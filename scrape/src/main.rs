use scraper::Html;

fn main() {
    for _ in 0..10 {
        scrape_random_puzzle(PuzzleType::Hard15x15);
    }
}

enum PuzzleType {
    Easy5x5,
    Easy7x7,
    Easy10x10,
    Easy12x12,
    Easy15x15,
    Easy20x20,
    Hard5x5,
    Hard7x7,
    Hard10x10,
    Hard12x12,
    Hard15x15,
}

impl PuzzleType {
    fn size_param(&self) -> u32 {
        match self {
            PuzzleType::Easy5x5 => 0,
            PuzzleType::Easy7x7 => 1,
            PuzzleType::Easy10x10 => 2,
            PuzzleType::Easy12x12 => 5,
            PuzzleType::Easy15x15 => 3,
            PuzzleType::Easy20x20 => 4,
            PuzzleType::Hard5x5 => 6,
            PuzzleType::Hard7x7 => 7,
            PuzzleType::Hard10x10 => 8,
            PuzzleType::Hard12x12 => 9,
            PuzzleType::Hard15x15 => 10,
        }
    }
}

fn scrape_random_puzzle(puzzle_type: PuzzleType) {
    let url = format!(
        "https://www.puzzle-nurikabe.com/?v=0&size={}",
        puzzle_type.size_param()
    );

    let html = reqwest::blocking::get(url).unwrap().text().unwrap();

    let document = Html::parse_document(&html);

    let puzzle_info_string = get_puzzle_info(&document);
    let (_, puzzle_info) = parse::parse_puzzle_info(&puzzle_info_string).unwrap();

    let puzzle_copyright = "Puzzle data from https://www.puzzle-nurikabe.com/".to_string();

    let puzzle_data = get_puzzle_data(&document);

    let contents = format!(
        "# {}\n# {}\n{}",
        puzzle_info_string, puzzle_copyright, puzzle_data
    );

    let dir = format!(
        "data/puzzles/puzzle-nurikabe-com/{}x{}-{}",
        puzzle_info.num_cols,
        puzzle_info.num_rows,
        puzzle_info.difficulty.to_ascii_lowercase()
    );

    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(format!("{}/{}.txt", dir, puzzle_info.puzzle_id), contents).unwrap();

    println!(
        "Scraped puzzle: {}x{}-{} (ID: {})",
        puzzle_info.num_cols, puzzle_info.num_rows, puzzle_info.difficulty, puzzle_info.puzzle_id
    );
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
        pub(crate) num_cols: u32,
        pub(crate) num_rows: u32,
        pub(crate) difficulty: String,
        pub(crate) puzzle_id: u32,
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
