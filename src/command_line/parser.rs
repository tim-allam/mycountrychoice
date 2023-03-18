use crate::filtering::params::FilterParams;
use clap::Parser;

/// Country choice application based on Nomad Capitalist data
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Input file that contains data about countries
   #[arg(short, long)]
   input_filename: Option<String>,

   /// Tax value from 20 (the worst) and 50 (tax-free)
   #[arg(short, long)]
   tax: Option<u8>,

   /// Dual citizenship: a value from 10 (forbidden) to 50 (allowed)
   #[arg(short, long)] //default_value_t = 10
   citizenship: Option<u8>,
   
   /// Personal freedom: a value from 10 to 50
   #[arg(short, long)]
   freedom: Option<u8>,
}


pub struct InputDataParser {
    arguments: Args
}

impl InputDataParser {
    pub fn new() -> Self {
        InputDataParser { arguments: Args::parse() }
    }

    pub fn get_csv_filename(&self) -> String {
        self.arguments.input_filename
            .clone()
            .unwrap_or_else(|| "countries.csv".to_string())
    }

    pub fn generate_filter_params(&self) -> Option<FilterParams> {
        let freedom = self.arguments.freedom.as_ref();        
        let tax = self.arguments.tax.as_ref();
        let citizenship = self.arguments.citizenship.as_ref();

        match (freedom, tax, citizenship) {
            (Some(f), Some(t), Some(dc)) => {
                Some( FilterParams::FreedomTaxCitizenship(*f, *t, *dc) )
            },
            (None, Some(t), Some(dc)) => {
                Some( FilterParams::DualCitizenshipTax(*t, *dc) )
            },
            (None, Some(t), None) => {
                Some( FilterParams::BetterTax(*t) )
            },
            _ => { None }
        } 
    }
}

