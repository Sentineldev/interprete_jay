
use crate::HashMap;






pub struct Lexer<'a>{
    pub grammar_tokens : HashMap<&'a str,&'a str>
}


impl<'a> Lexer<'_>{
    pub fn build(grammar_tokens: &HashMap<&'a str,&'a str>) -> Lexer<'a>{

        let grm_tokens = grammar_tokens.clone();

        Lexer { grammar_tokens: grm_tokens } 
    }


    fn is_keyword(&self,keyword : &String) -> bool{
        if let Some(value) = self.grammar_tokens.get(keyword.as_str()){
            if value.contains("KEYWORD") { return true }
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

    fn is_separator(&self,character : char) -> bool{
        if let Some(element) = self.grammar_tokens.get(character.to_string().as_str()){
            if element.contains("SEPARATOR"){ return true }
        }
        false
    }
    fn is_operator(&self,character : char) -> bool {
        if let Some(element) = self.grammar_tokens.get(character.to_string().as_str()){
            if element.contains("OPERATOR"){ return true }
        }
        false
    }

    pub fn read_file_lines(&self,file_content : &String) -> Result<Vec<String>,String>{

        //Lee linea por linea y convierte en tokens los elementos que se encuentren en la linea.
        //Elimina los espacios atras y adelante de la linea.
        //Convierte la linea en un vector separado por espacios.

        let mut tokenized_vector : Vec<String> = Vec::new();

        for line in file_content.trim().lines(){
            if line.trim().starts_with("//") { continue }
            if line.trim().len() == 0 { continue }
            let splited_line : Vec<&str> = line.trim().split_whitespace().collect();
            if let Ok(val) = self.scanner(&splited_line){
                tokenized_vector.append(&mut val.clone());
            }

        }

        Ok(tokenized_vector)
    }

    pub fn scanner(&self,vectorized_line : &Vec<&str>) -> Result<Vec<String>,String>{

        //Lee linea por linea y convierte en tokens los elementos que se encuentren en la linea.
        //Elimina los espacios atras y adelante de la linea.
        //Convierte la linea en un vector separado por espacios.

        let mut tokenized_vector = Vec::new();
        for token in vectorized_line{
            if let Ok(val) = self.tokenizer(token){
                tokenized_vector.append(&mut val.clone());
                continue;
            }

            return Err(format!("Error en la lectura de los tokens."));
            
        }


        Ok(tokenized_vector)

    }

    fn tokenizer(&self,token : &str) -> Result<Vec<String>,String>{
        //Genera un vector con los tokens leidos.

        //En el primer caso verifica si es una palabra clave.
        //En el segundo recorre cada caracter de la cadena
        //arma los tokens y los va separando por medio de 'Separators'
        // o tambien operadores
        let mut tokenized_vector  : Vec<String> = Vec::new(); 
        
        if self.is_keyword(&token.to_string()){
            tokenized_vector.push(token.to_string());
            return Ok(tokenized_vector);
        }

        
        let mut builded_token = String::new();

        let mut check_next = false;
        
        for current in token.chars(){    
            
            if self.is_letter_or_dig(current) && !check_next{
                builded_token.push(current);
            }
            else if self.is_letter_or_dig(current) && check_next{
                check_next = false;
                tokenized_vector.push(builded_token.clone());
                builded_token.clear();
                builded_token.push(current);
            }
            else if self.is_separator(current) && !check_next{
                if !builded_token.is_empty(){
                    tokenized_vector.push(builded_token.clone());
                    builded_token.clear();
                }
                tokenized_vector.push(current.to_string());
            }
            else if self.is_separator(current) && check_next{
                tokenized_vector.push(builded_token.clone());
                builded_token.clear();
                builded_token.push(current);
            }
            else if self.is_operator(current){
                
                if check_next{
                    builded_token.push(current);
                }
                else{
                    if !builded_token.is_empty(){
                        tokenized_vector.push(builded_token.clone());
                        builded_token.clear();
                    }
                    builded_token.push(current);
                    check_next = true;
                }   
            }
        }

        if !builded_token.is_empty(){
            tokenized_vector.push(builded_token);
        }

        Ok(tokenized_vector)
    }
}