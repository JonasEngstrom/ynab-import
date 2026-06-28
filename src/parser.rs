use regex;
use std::error;
use std::process::exit;
use crate::*;

/// Parses different bank input formats.
pub enum Parser {
    /// Parses data copied and pasted from the Swedish branch of Danske Bank’s Hembanken’s web interface.
    DanskeBank,
}

impl Parser {
    /// Returns the regex required to extract data from the desired bank.
    fn regex(&self) -> Result<regex::Regex, regex::Error> {
        match self {
            Self::DanskeBank => regex::Regex::new(r"(?m)^(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})[ \t]*\r?\n[ \t]*(?<merchant>[^\r\n]+?)(?:[ \t]+\){4})?[ \t]*\r?\n[ \t]*(?<amount>[+-]?(?:\d+|\d{1,3}(?:\.\d{3})+),\d{2})"),
        }
    }

    /// Parse data from the desired bank. Returns a string in CSV format, for import into YNAB.
    pub fn parse(&self, text_to_parse: &str, memo: Option<&str>) -> Result<String, Box<dyn error::Error>> {
        let regex = match self.regex() {
            Ok(regex) => regex,
            Err(_) => {
                eprintln!("{}", messages::UNABLE_TO_PARSE_REGEX);
                exit(1);
            },
        };

        let mut transaction_list = csv::TransactionList::new();

        for capture in regex.captures_iter(text_to_parse) {
            let year = &capture["year"].parse::<u16>()?;
            let month = &capture["month"].parse::<u8>()?;
            let day = &capture["day"].parse::<u8>()?;
            let merchant = &capture["merchant"];
            let amount = &capture["amount"]
                .replace(".", "")
                .replace(",", ".")
                .parse::<f64>()?;

            let date = date::Date::new(*year, *month, *day)?;
            let payee = payee::Payee::new(merchant.to_string());
            // let memo = memo::Memo::new(optional_memo);
            let memo = match memo {
                Some(memo) => memo::Memo::new(memo.to_string()),
                None => None,
            };
            let flow = flow::Flow::from_amount(*amount);
    
            let transaction = csv::Transaction::new(date, payee, memo, flow);

            transaction_list.push(transaction);
        }


        Ok(transaction_list.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn danske_bank_parsing_works_with_memo() -> Result<(), String> {
        let danske_bank_text_one = r#"Enter start date yyyy.mm.dd REB1637Enter end date yyyy.mm.dd REB1638
Framtida
JuniMajAprilMarsFebruariJanuari2026DecemberNovemberOktoberSeptemberAugustiJuliJuniMaj
Tidigare
Kontoutdrag


Visa kategorier
Juni 2026

Datum		 	Text		Belopp	 	 		Saldo		Avstämt
2026-06-25			
Foo Bar ))))
-41,45
 
 		62.930,69		

2026-06-24			
bar work
7.650,02
 
 		62.972,14		

2026-06-22			
Foo ))))
-170,00
 
 		55.322,12		

2026-06-22			
GIRAFFE
-322,05
 
 		55.492,12		

2026-06-08			
Överföring
11.524,85
 
 		55.814,17		

2026-06-08			
City ))))
-69,50
 
 		44.289,32		

2026-06-08			
Virta EV Charge Proc
-200,00
 
 		44.358,82		

2026-06-08			
Sl App
-43,00
 
 		44.558,82		

2026-06-08			
Sl App
-43,00
 
 		44.601,82		

2026-06-08			
GENERIC.SE
-566,26
 
 		44.644,82		

2026-06-05			
Sl App
-43,00
 
 		45.211,08		

2026-06-05			
Sl App
-43,00
 
 		45.254,08		

2026-06-02		 	
ENERGI
-393,00
 
 		45.297,08		

2026-06-01			
Kronans AB ))))
-195,00
 
 		45.690,08		

Maj 2026
2026-05-29		 	
232
-13.439,00
 
 		45.885,08		

2026-05-28		 	
SVERKER
-609,00
 
 		59.324,08		

2026-05-28		 	
BER AB
-247,00
 
 		59.933,08		

2026-05-26			
Virta EV Charge Proc
-200,00
 
 		60.180,08		

2026-05-25			
Överföring
12.239,68
 
 		60.380,08		

2026-05-25			
Sl App
-40,00
 
 		48.140,40		

2026-05-25			
Tag
-120,00
 
 		48.180,40		
"#;
        let expected_danske_bank_result_one = r#""Date","Payee","Memo","Outflow","Inflow"
"2026-06-25","Foo Bar","A short memo","41.45",""
"2026-06-24","bar work","A short memo","","7650.02"
"2026-06-22","Foo","A short memo","170.00",""
"2026-06-22","GIRAFFE","A short memo","322.05",""
"2026-06-08","Överföring","A short memo","","11524.85"
"2026-06-08","City","A short memo","69.50",""
"2026-06-08","Virta EV Charge Proc","A short memo","200.00",""
"2026-06-08","Sl App","A short memo","43.00",""
"2026-06-08","Sl App","A short memo","43.00",""
"2026-06-08","GENERIC.SE","A short memo","566.26",""
"2026-06-05","Sl App","A short memo","43.00",""
"2026-06-05","Sl App","A short memo","43.00",""
"2026-06-02","ENERGI","A short memo","393.00",""
"2026-06-01","Kronans AB","A short memo","195.00",""
"2026-05-29","232","A short memo","13439.00",""
"2026-05-28","SVERKER","A short memo","609.00",""
"2026-05-28","BER AB","A short memo","247.00",""
"2026-05-26","Virta EV Charge Proc","A short memo","200.00",""
"2026-05-25","Överföring","A short memo","","12239.68"
"2026-05-25","Sl App","A short memo","40.00",""
"2026-05-25","Tag","A short memo","120.00","""#;
        let danske_bank_text_two = r#"2026-06-22			
Bar ))))
-170,00
 
 		55.322,12		

2026-06-22			
GIRAFFE
-322,05
 
 		55.492,12		

2026-06-08			
Överföring
11.524,85
 
 		55.814,17		

2026-06-08			
City ))))
-69,50
 
 		44.289,32		

2026-06-08			
Virta EV Charge Proc
-200,00
 
 		44.358,82		

2026-06-08			
Sl App
-43,00
 
 		44.558,82		

2026-06-08			
Sl App
-43,00
 
 		44.601,82		

2026-06-08			
GENERIC.SE
-566,26
 
 		44.644,82		

2026-06-05			
Sl App
-43,00
 
 		45.211,08		

2026-06-05			
Sl App
-43,00
 
 		45.254,08		

2026-06-02		 	
ENERGI
-393,00
 
 		45.297,08		
"#;
        let expected_danske_bank_result_two = r#""Date","Payee","Memo","Outflow","Inflow"
"2026-06-22","Bar","A short memo","170.00",""
"2026-06-22","GIRAFFE","A short memo","322.05",""
"2026-06-08","Överföring","A short memo","","11524.85"
"2026-06-08","City","A short memo","69.50",""
"2026-06-08","Virta EV Charge Proc","A short memo","200.00",""
"2026-06-08","Sl App","A short memo","43.00",""
"2026-06-08","Sl App","A short memo","43.00",""
"2026-06-08","GENERIC.SE","A short memo","566.26",""
"2026-06-05","Sl App","A short memo","43.00",""
"2026-06-05","Sl App","A short memo","43.00",""
"2026-06-02","ENERGI","A short memo","393.00","""#;
        let danske_bank_text_three = r#"Juni 2026

Datum		 	Text		Belopp	 	 		Saldo		Avstämt
2026-06-25			
Foo Bar ))))
-41,45
 
 		62.930,69		

2026-06-24			
bar work
7.650,02
 
 		62.972,14		

2026-06-22			
Bar ))))
-170,00
 
 		55.322,12		

2026-06-22			
GIRAFFE
-322,05
 
 		55.492,12		

2026-06-08			
Överföring
11.524,85
 
 		55.814,17		

2026-06-08			
City ))))
-69,50
 
 		44.289,32		

2026-06-08			
Virta EV Charge Proc
-200,00
 
 		44.358,82		

2026-06-08			
Sl App
-43,00
 
 		44.558,82		

2026-06-08			
Sl App
-43,00
 
 		44.601,82		

2026-06-08			
GENERIC.SE
-566,26
 
 		44.644,82		
"#;
        let expected_danske_bank_result_three = r#""Date","Payee","Memo","Outflow","Inflow"
"2026-06-25","Foo Bar","A short memo","41.45",""
"2026-06-24","bar work","A short memo","","7650.02"
"2026-06-22","Bar","A short memo","170.00",""
"2026-06-22","GIRAFFE","A short memo","322.05",""
"2026-06-08","Överföring","A short memo","","11524.85"
"2026-06-08","City","A short memo","69.50",""
"2026-06-08","Virta EV Charge Proc","A short memo","200.00",""
"2026-06-08","Sl App","A short memo","43.00",""
"2026-06-08","Sl App","A short memo","43.00",""
"2026-06-08","GENERIC.SE","A short memo","566.26","""#;

        let danske_bank_parser = Parser::DanskeBank;

        let danske_bank_parsed_text_one = danske_bank_parser
            .parse(danske_bank_text_one, Some("A short memo"))
            .unwrap();
        let danske_bank_parsed_text_two = danske_bank_parser
            .parse(danske_bank_text_two, Some("A short memo"))
            .unwrap();
        let danske_bank_parsed_text_three = danske_bank_parser
            .parse(danske_bank_text_three, Some("A short memo"))
            .unwrap();
        
        assert_eq!(danske_bank_parsed_text_one, expected_danske_bank_result_one);
        assert_eq!(danske_bank_parsed_text_two, expected_danske_bank_result_two);
        assert_eq!(danske_bank_parsed_text_three, expected_danske_bank_result_three);

        Ok(())
    }

    #[test]
    fn danske_bank_parsing_works_without_memo() -> Result<(), String> {
        let danske_bank_text_one = r#"Enter start date yyyy.mm.dd REB1637Enter end date yyyy.mm.dd REB1638
Framtida
JuniMajAprilMarsFebruariJanuari2026DecemberNovemberOktoberSeptemberAugustiJuliJuniMaj
Tidigare
Kontoutdrag


Visa kategorier
Juni 2026

Datum		 	Text		Belopp	 	 		Saldo		Avstämt
2026-06-25			
Foo Bar ))))
-41,45
 
 		62.930,69		

2026-06-24			
bar work
7.650,02
 
 		62.972,14		

2026-06-22			
Foo ))))
-170,00
 
 		55.322,12		

2026-06-22			
GIRAFFE
-322,05
 
 		55.492,12		

2026-06-08			
Överföring
11.524,85
 
 		55.814,17		

2026-06-08			
City ))))
-69,50
 
 		44.289,32		

2026-06-08			
Virta EV Charge Proc
-200,00
 
 		44.358,82		

2026-06-08			
Sl App
-43,00
 
 		44.558,82		

2026-06-08			
Sl App
-43,00
 
 		44.601,82		

2026-06-08			
GENERIC.SE
-566,26
 
 		44.644,82		

2026-06-05			
Sl App
-43,00
 
 		45.211,08		

2026-06-05			
Sl App
-43,00
 
 		45.254,08		

2026-06-02		 	
ENERGI
-393,00
 
 		45.297,08		

2026-06-01			
Kronans AB ))))
-195,00
 
 		45.690,08		

Maj 2026
2026-05-29		 	
232
-13.439,00
 
 		45.885,08		

2026-05-28		 	
SVERKER
-609,00
 
 		59.324,08		

2026-05-28		 	
BER AB
-247,00
 
 		59.933,08		

2026-05-26			
Virta EV Charge Proc
-200,00
 
 		60.180,08		

2026-05-25			
Överföring
12.239,68
 
 		60.380,08		

2026-05-25			
Sl App
-40,00
 
 		48.140,40		

2026-05-25			
Tag
-120,00
 
 		48.180,40		
"#;
        let expected_danske_bank_result_one = r#""Date","Payee","Memo","Outflow","Inflow"
"2026-06-25","Foo Bar","","41.45",""
"2026-06-24","bar work","","","7650.02"
"2026-06-22","Foo","","170.00",""
"2026-06-22","GIRAFFE","","322.05",""
"2026-06-08","Överföring","","","11524.85"
"2026-06-08","City","","69.50",""
"2026-06-08","Virta EV Charge Proc","","200.00",""
"2026-06-08","Sl App","","43.00",""
"2026-06-08","Sl App","","43.00",""
"2026-06-08","GENERIC.SE","","566.26",""
"2026-06-05","Sl App","","43.00",""
"2026-06-05","Sl App","","43.00",""
"2026-06-02","ENERGI","","393.00",""
"2026-06-01","Kronans AB","","195.00",""
"2026-05-29","232","","13439.00",""
"2026-05-28","SVERKER","","609.00",""
"2026-05-28","BER AB","","247.00",""
"2026-05-26","Virta EV Charge Proc","","200.00",""
"2026-05-25","Överföring","","","12239.68"
"2026-05-25","Sl App","","40.00",""
"2026-05-25","Tag","","120.00","""#;
        let danske_bank_text_two = r#"2026-06-22			
Bar ))))
-170,00
 
 		55.322,12		

2026-06-22			
GIRAFFE
-322,05
 
 		55.492,12		

2026-06-08			
Överföring
11.524,85
 
 		55.814,17		

2026-06-08			
City ))))
-69,50
 
 		44.289,32		

2026-06-08			
Virta EV Charge Proc
-200,00
 
 		44.358,82		

2026-06-08			
Sl App
-43,00
 
 		44.558,82		

2026-06-08			
Sl App
-43,00
 
 		44.601,82		

2026-06-08			
GENERIC.SE
-566,26
 
 		44.644,82		

2026-06-05			
Sl App
-43,00
 
 		45.211,08		

2026-06-05			
Sl App
-43,00
 
 		45.254,08		

2026-06-02		 	
ENERGI
-393,00
 
 		45.297,08		
"#;
        let expected_danske_bank_result_two = r#""Date","Payee","Memo","Outflow","Inflow"
"2026-06-22","Bar","","170.00",""
"2026-06-22","GIRAFFE","","322.05",""
"2026-06-08","Överföring","","","11524.85"
"2026-06-08","City","","69.50",""
"2026-06-08","Virta EV Charge Proc","","200.00",""
"2026-06-08","Sl App","","43.00",""
"2026-06-08","Sl App","","43.00",""
"2026-06-08","GENERIC.SE","","566.26",""
"2026-06-05","Sl App","","43.00",""
"2026-06-05","Sl App","","43.00",""
"2026-06-02","ENERGI","","393.00","""#;
        let danske_bank_text_three = r#"Juni 2026

Datum		 	Text		Belopp	 	 		Saldo		Avstämt
2026-06-25			
Foo Bar ))))
-41,45
 
 		62.930,69		

2026-06-24			
bar work
7.650,02
 
 		62.972,14		

2026-06-22			
Bar ))))
-170,00
 
 		55.322,12		

2026-06-22			
GIRAFFE
-322,05
 
 		55.492,12		

2026-06-08			
Överföring
11.524,85
 
 		55.814,17		

2026-06-08			
City ))))
-69,50
 
 		44.289,32		

2026-06-08			
Virta EV Charge Proc
-200,00
 
 		44.358,82		

2026-06-08			
Sl App
-43,00
 
 		44.558,82		

2026-06-08			
Sl App
-43,00
 
 		44.601,82		

2026-06-08			
GENERIC.SE
-566,26
 
 		44.644,82		
"#;
        let expected_danske_bank_result_three = r#""Date","Payee","Memo","Outflow","Inflow"
"2026-06-25","Foo Bar","","41.45",""
"2026-06-24","bar work","","","7650.02"
"2026-06-22","Bar","","170.00",""
"2026-06-22","GIRAFFE","","322.05",""
"2026-06-08","Överföring","","","11524.85"
"2026-06-08","City","","69.50",""
"2026-06-08","Virta EV Charge Proc","","200.00",""
"2026-06-08","Sl App","","43.00",""
"2026-06-08","Sl App","","43.00",""
"2026-06-08","GENERIC.SE","","566.26","""#;

        let danske_bank_parser = Parser::DanskeBank;

        let danske_bank_parsed_text_one = danske_bank_parser
            .parse(danske_bank_text_one, None)
            .unwrap();
        let danske_bank_parsed_text_two = danske_bank_parser
            .parse(danske_bank_text_two, None)
            .unwrap();
        let danske_bank_parsed_text_three = danske_bank_parser
            .parse(danske_bank_text_three, None)
            .unwrap();
        
        assert_eq!(danske_bank_parsed_text_one, expected_danske_bank_result_one);
        assert_eq!(danske_bank_parsed_text_two, expected_danske_bank_result_two);
        assert_eq!(danske_bank_parsed_text_three, expected_danske_bank_result_three);

        Ok(())
    }
}