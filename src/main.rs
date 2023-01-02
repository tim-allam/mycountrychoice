mod command_line;
mod filtering;
mod data_reader;
mod cdata;

use command_line::parser::InputDataParser;
use data_reader::interface::CountryRecordReader;
use data_reader::csv_reader::CSVReader;

use filtering::factory::FilterFactory;

fn main() {

    let command_line_parser = InputDataParser::new();
    let input_filename = command_line_parser.get_csv_filename();
    let csv_reader = CSVReader::new(&input_filename);

    if let Ok(valid_records) = csv_reader.extract_records() {
        // build filter parameters based on user's input
        let filter_params = command_line_parser.generate_filter_params();

        // generate suitable filter to process data records 
        let filter = FilterFactory::generate_filter(&filter_params);
        let mut filtered_records = filter.process(&valid_records);

        filtered_records.sort_by(|a, b| b.travel_power.cmp(&a.travel_power));

        if !filtered_records.is_empty() {
            for r in &filtered_records {
                println!("Country: {:>20} [traveling_power: {}, taxation: {}, dual_citizenship: {}, freedom: {}, perception: {}]",
                r.country, r.travel_power, r.taxation, r.dual_citizenship, r.freedom, r.perception
                );
            }
        } else {
            println!("No records found!!!");
        }
    }
}

