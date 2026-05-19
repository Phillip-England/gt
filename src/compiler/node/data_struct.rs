use crate::{app_err, compiler::{node::DataField, parser::{ParserErr, DataType}, tokenizer::AdvancedToken}, err::{AppErr, AppErrKind}};




#[derive(Clone, Debug)]
pub struct DataStruct {
	pub name: String,
	pub node_fields: Vec<DataField>
}

impl DataStruct {

	pub fn new(toks: Vec<AdvancedToken>) -> Result<DataStruct, AppErr> {
	   
		// extracting data type name
		let second_tok_opt = toks.get(1).clone();
		if second_tok_opt.is_none() {
			return Err(app_err!(AppErrKind::Ast(ParserErr::MalformedDataType(String::from("attempted to access token containing data type name but could not locate it")))))
		}
		let second_tok = second_tok_opt.unwrap();
		let data_type_name: String;
		if let AdvancedToken::Indicator(s) = second_tok {
			data_type_name = s.clone();
		} else {
			return Err(app_err!(AppErrKind::Ast(ParserErr::ExpectedIndicatorToken(String::from("attempted to access tokens for ast generation, and expected an indicator token, but could not find one")))) )  
		}


		let mut count = 0;
		let mut field_names: Vec<String> = vec![];
		let mut field_types: Vec<DataType> = vec![];
		for tok in toks {
			if count > 2 {
				if matches!(tok, AdvancedToken::ClosedCurlyBrace) {
					count = count + 1;
					continue
				}
				// odd should be field name
				if count % 2 == 1 {
					if matches!(tok, AdvancedToken::Indicator(_)) {
						match tok {
							AdvancedToken::Indicator(s) => {
								let field_name = s.to_owned();
								field_names.push(field_name);
							},
							_ => {}
						}
					}
					count = count + 1;
					continue
				}
				// even should be field data type
				if matches!(tok, AdvancedToken::KeywordNum) {
					field_types.push(DataType::Num)
				}
				if matches!(tok, AdvancedToken::KeywordStr) {
					field_types.push(DataType::Str)
				}
				if matches!(tok, AdvancedToken::KeywordBool) {
					field_types.push(DataType::Bool)
				}
				if matches!(tok, AdvancedToken::Indicator(_)) {
					field_types.push(DataType::Custom);
				}

			}
			count = count + 1;
		}

		// our field names and data types should be same len
		if field_names.len() != field_types.len() {
			return Err(app_err!(AppErrKind::Ast(ParserErr::MalformedDataType(String::from("expected our field names and field types to be the same length but they were not")))));
		}

		let mut fields: Vec<DataField> = vec![];
		let mut count: usize = 0;
		loop {
			if count > field_names.len() - 1 || count > field_types.len() - 1 {
				break;
			}
			let name_opt = field_names.get(count);
			if name_opt.is_none() {
				return Err(app_err!(AppErrKind::Ast(ParserErr::MalformedDataType(String::from("could not find field name in expected location")))))
			}
			let name = name_opt.unwrap();
			let t_opt = field_types.get(count);
			if t_opt.is_none() {
				return Err(app_err!(AppErrKind::Ast(ParserErr::MalformedDataType(String::from("could not find field type in expected location")))))
			}        
			let t = t_opt.unwrap();
			let field = DataField::new(name.to_owned(), t.clone());
			fields.push(field);
			count = count + 1
		}
		return Ok(DataStruct {
			name: data_type_name,
			node_fields: fields,
		})
	}

}

