
use crate::HashMap;






#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum TokenType{
    Identifier,
    KeyWord,
    Separator,
    Operator
}
#[derive(Debug)]
#[derive(Clone)]
pub struct Token{
    pub value : String,
    pub column : u32,
    pub line : u32,
    pub token_type : TokenType
}

impl Token{
    pub fn build(token : &String,token_type : TokenType,current_line : u32) -> Token{

        let new_token = token.clone();
        Token {  value: new_token,token_type: token_type,line:current_line,column:0  }
    }

    pub fn get_value(&self) -> String{
        self.value.clone()
    }
    pub fn get_line(&self) -> u32{
        self.line
    }
}







pub struct Lexer<'a>{
    pub grammar_tokens : HashMap<&'a str,&'a str>
}


impl<'a> Lexer<'_>{
    pub fn build(grammar_tokens: &HashMap<&'a str,&'a str>) -> Lexer<'a>{

        let grm_tokens = grammar_tokens.clone();

        Lexer { grammar_tokens: grm_tokens } 
    }


    fn is_of_type(&self,element : &String,type_of : &str) -> bool{
        if let Some(result) = self.grammar_tokens.get(element.as_str()){
            if result.contains(type_of) { return true }
        }
        false
    }

    fn is_letter_or_dig(&self,character : char) -> bool{
        if let Some(element) = self.grammar_tokens.get(character.to_string().as_str()){
            if element == &"LETTER" { return true }
            else if element == &"NUMBER" { return true }
        }
        false
    }


    

    pub fn read_file_lines(&self,file_content : &String) -> Result<Vec<Token>,String>{

        //Lee linea por linea y convierte en tokens los elementos que se encuentren en la linea.
        //Elimina los espacios atras y adelante de la linea.
        //Convierte la linea en un vector separado por espacios.

        let mut tokenized_vector : Vec<Token> = Vec::new();

        let mut current_line : u32 =  1;

        for line in file_content.trim().lines(){
            if line.trim().starts_with("//") { continue }
            if line.trim().len() == 0 { continue }
            

            let split_by_comment : Vec<&str> = line.trim().split("//").collect(); 
            let split_by_spaces : Vec<&str> = split_by_comment[0].trim().split_whitespace().collect();
            
            //let splited_line : Vec<&str> = line.trim().split_whitespace().collect();

            if let Ok(mut val) = self.scanner(&split_by_spaces,current_line){
                tokenized_vector.append(&mut val);
            }

            current_line+=1;

        }

        Ok(tokenized_vector)
    }

    pub fn scanner(&self,vectorized_line : &Vec<&str>,current_line : u32) -> Result<Vec<Token>,String>{

        //Recibe una linea separado por espacios en un vector
        //convierte cada elemento del vector en un token.

        //Retorna un vector  de tokens.

        let mut tokenized_vector = Vec::new();
        for token in vectorized_line{
            if let Ok(mut val) = self.tokenizer(token,current_line){
                tokenized_vector.append(&mut val);
                continue;
            }

            return Err(format!("Error en la lectura de los tokens."));
            
        }


        Ok(tokenized_vector)

    }

    fn tokenizer(&self,token : &str,current_line : u32) -> Result<Vec<Token>,String>{
        //Genera un vector con los tokens leidos.

        //En el primer caso verifica si es una palabra clave.
        //En el segundo recorre cada caracter de la cadena
        //arma los tokens y los va separando por medio de 'Separators'
        // o tambien operadores

        let mut tokenized_vector_aux : Vec<Token> = Vec::new();
        
        if self.is_of_type(&token.to_string(),"KEYWORD"){


            tokenized_vector_aux.push(Token::build(
                &token.to_string(),
                TokenType::KeyWord,
                current_line
            ));

            return Ok(tokenized_vector_aux);
        }

        
        let mut builded_token = String::new();

        let mut check_next = false;
        
        for current in token.chars(){    
            
            if self.is_letter_or_dig(current) && !check_next{
                builded_token.push(current);
            }
            else if self.is_letter_or_dig(current) && check_next{
                check_next = false;

                tokenized_vector_aux.push(Token::build(
                    &builded_token,
                    TokenType::Operator,
                    current_line
                ));

                builded_token.clear();
                builded_token.push(current);
            }
            else if self.is_of_type(&current.to_string(),"SEPARATOR") && !check_next{
                if !builded_token.is_empty(){

                    if self.is_of_type(&builded_token,"KEYWORD"){
                        tokenized_vector_aux.push(Token::build(
                            &builded_token,
                            TokenType::KeyWord,
                            current_line
                        ));
                    }
                    else{
                        tokenized_vector_aux.push(Token::build(
                            &builded_token,
                            TokenType::Identifier,
                            current_line
                        ));
                    }
                    
                    builded_token.clear();
                }
                tokenized_vector_aux.push(Token::build(
                    &current.to_string(),
                    TokenType::Separator,
                    current_line
                ));
            }
            else if self.is_of_type(&current.to_string(),"SEPARATOR") && check_next{
                check_next = false;
                
                tokenized_vector_aux.push(Token::build(
                    &builded_token,
                    TokenType::Operator,
                    current_line
                ));

                
                builded_token.clear();
                tokenized_vector_aux.push(Token::build(
                    &current.to_string(),
                    TokenType::Separator,
                    current_line
                ));
            }
            else if self.is_of_type(&current.to_string(),"OPERATOR"){
                
                if check_next{
                    builded_token.push(current);
                }
                else{
                    if !builded_token.is_empty(){
                        
                        if self.is_of_type(&builded_token,"KEYWORD"){
                            tokenized_vector_aux.push(Token::build(
                                &builded_token,
                                TokenType::KeyWord,
                                current_line
                            ));
                        }
                        else{
                            tokenized_vector_aux.push(Token::build(
                                &builded_token,
                                TokenType::Identifier,
                                current_line
                            ));
                        }

                        builded_token.clear();
                    }

                    builded_token.push(current);
                    check_next = true;
                }   
            }
        }

        if !builded_token.is_empty(){
            if self.is_of_type(&builded_token,"KEYWORD"){
                tokenized_vector_aux.push(Token::build(
                    &builded_token,
                    TokenType::KeyWord,
                    current_line
                ));
            }
            else{
                tokenized_vector_aux.push(Token::build(
                    &builded_token,
                    TokenType::Identifier,
                    current_line
                ));
            }
        }


        Ok(tokenized_vector_aux)
    }
}