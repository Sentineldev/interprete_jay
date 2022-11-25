


use std::collections::HashMap;
use std::process;



#[derive(Debug)]
pub struct Parser<'a>{

    //Estructura hacer el analisis sintactico
    //Obtiene los tokens definidos en la gramatica
    //Ademas tambien receibe un vector con todos los tokens leidos en el archivo .jay

    pub grammar_tokens : HashMap<&'a str,&'a str>,
    pub file_tokens  : Vec<String>
}



impl<'a> Parser<'_>{
    
    pub fn build(grammar_tokens: &HashMap<&'a str,&'a str>,file_tokens: &Vec<String>) -> Parser<'a>{

        let gram = grammar_tokens.clone();

        let fil_tokens = file_tokens.clone();

        Parser { grammar_tokens : gram, file_tokens: file_tokens.to_vec() }
    }

    pub fn run(&mut self){
        //Inicia el analisis sintactico por la primera regla de produccion.
        self.program();
    }

    fn end_of_file(&self) -> bool{
        //Verifica si queda o no un solo caracter.
        self.file_tokens.len()  == 1
    }

    fn consume(&mut self){
        //Remueve el elemento que se encuentre al inicio del vector.
        self.file_tokens.remove(0);
    }

    fn compare_to_top(&self,token : &str) -> bool{
        //Compara el token con lo que se encuentre al inicio del vector.
        if self.file_tokens.get(0) == Some(&token.to_string()){
            return true;
        }
        false
    }
    fn is_letter(&self,character : char) -> bool{
        
        //Valida si el caracter es una letra.
        if let Some(element) = self.grammar_tokens.get(character.to_string().as_str()){
            if element == &"LETTER"{
                return true;
            }
        }


        false
    }
    fn is_digit(&self,character : char) -> bool{
        //Valida si el caracter es una letra.
        if let Some(element) = self.grammar_tokens.get(character.to_string().as_str()){
            if element == &"NUMBER"{
                return true;
            }
        }
        false
    }

    fn is_keyword(&self,keyword : &String)-> bool{

        if let Some(value) = self.grammar_tokens.get(keyword.as_str()){
            if value.contains(&"KEYWORD") { return true }
        }
        false
    }
   

    fn program(&mut self){

         //Verifica la regla de produccion de un programa
         //<Program> ::= void main() '{' <Declarations> <Statements> '}'

        if self.compare_to_top("void"){
            self.consume();
            if self.compare_to_top("main"){
                self.consume();
                if self.compare_to_top("("){
                    self.consume();
                    if self.compare_to_top(")"){
                        self.consume();
                        if self.compare_to_top("{"){
                            self.consume();
                            while self.declarations() || self.statements(){
                                continue;
                            }
                            if self.compare_to_top("}"){
                                self.consume();
                            }
        
                        }
                    }
                }
                
            }
        }   
    }

    fn declarations(&mut self) -> bool{

        //Verifica las declaraciones
        //formato en ebnf <Declarations> ::= < {<Declaration>}*
        //self.declaration()
        
        if self.declaration(){
            self.declarations();
        }
        else{
            println!("{:?}",self.file_tokens);
            return false;
        }

        true
    }

    fn declaration(&mut self) -> bool{

        //Verifica si es una declaracion
        //1. Verifica que el primer elemento en el vector de tokens sea un type
        //2. Verifica los identificadores  que siguen luego del tipo de declaracion
        //Formato <Declaration> ::= <Type> <Identifiers>';'
        if self.declaration_type(){
            println!("enter declaration...");
            if self.identifiers(){
                println!("identifiers devuelve true");
                if self.compare_to_top(";"){
                    self.consume();
                    return true;
                }
            }
            
        }


        false
    }

    fn declaration_type(&mut self) -> bool{

        //Verifica si el elemento en el vector contiene un 'INT' o 'Boolean'
        //Formato <Type> ::= int | boolean

        println!("enter type...");
        if self.end_of_file() { return false }
        if self.compare_to_top("int") { 
            self.consume();
            return true
        }
        if self.compare_to_top("boolean") { 
            self.consume();
            return true 
        }

        false
    }

    fn identifiers(&mut self) -> bool{
        //Verifica que tenga forma de identificadores
        //Formato <Identifiers> ::= <Identifier> {','<identifier>}*
        println!("enter identifiers...");
        if self.end_of_file() { return false }
        if !self.identifier(){
            return false;
        }

        self.consume();
        
        while self.compare_to_top(","){
            self.consume();
            if !self.identifier(){
                return false;
            }
            self.consume();
        }
        true

    }
    fn identifier(&mut self) -> bool{
        //Verifica que el identificador inicie con  una letra
        //y tenga una secuencia de letras y numero despues del primer caracter.
        //Formato <Identifier> ::= <Letter> | <Identifier> <Letter> | <Identifier> <Digit>
        println!("enter identifier...");
        if self.end_of_file() { return false}
        if let Some(value) = self.file_tokens.get(0){
            
            if self.is_keyword(&value){ 
                println!("value: {value}");
                return false }


            if self.is_letter(value.chars().next().unwrap()){
                let sliced_string = &value[1..];
                for chr in sliced_string.chars(){
                    if self.is_letter(chr) { continue }
                    else if self.is_digit(chr) { continue }
                    return false;
                }
                return true;
            }
                
                
        }

    
        false
    }

    fn statements(&mut self) -> bool{
        //Inicia la verificacion de statement.
        //<Statements> ::= {<Statement>}*
        if self.statement(){
            self.statements();
        }
        else{
            return false;
        }

        true

    }
    fn statement(&mut self) -> bool{
        //Verifica que sea un statement
        //Formato <Statement> ::= <Block> | <Assignment> | <IfStatement> | <WhileStatement>
        if self.block(){  return true }
        else if self.assignment() { return true } 
        else if  self.if_statement() { return true } 
        else if self.while_statement() { return true }


        false
    }

    fn block(&mut self) -> bool{
        //Verifica si es un bloque 
        //Si el primer elemento del vector es un {
        //entonces se trata de un bloque y verifica si hay statments dentro del bloque
        //debe encontrar un } para estar valido.
        //Formato <Block> ::= '{'<Statements>'}'
        if self.compare_to_top("{"){
            self.consume();
            if self.statements() {
                println!(" en block statement {:?}",self.file_tokens);
                if self.compare_to_top("}"){ 
                    println!(" en block statement clsoe bracket {:?}",self.file_tokens);
                    self.consume();
                    return true 
                }
            }
        }

        false
    }

    pub fn assignment(&mut self) -> bool{
        //Verifica si se trata de una asignacion
        //Primero verifica que sea un identificador valido
        //Que contengan el signo = que indica asignacion
        //y por ultimo verifica la expresion y concluya con ';'
        //Formato <Assignment> ::= <Identifier> '=' <Expression> ';'
        if self.identifier(){
            self.consume();
            if self.compare_to_top("="){
                self.consume();
                if self.expression(){
                    if self.compare_to_top(";"){
                        self.consume();
                        return true;
                    }
                    eprintln!("Error sintactico missing ';'");
                }
            }
        }

        false
    }

    pub fn if_statement(&mut self) -> bool{
        //Verifica la sintaxis de un IF
        //Formato EBNF <IfStatement> ::= if (<Expression>) <Statement> [else <Statement>]

        if self.compare_to_top("if"){
            println!("consuming if.");
            self.consume();
            if self.compare_to_top("("){
                self.consume();
                if self.expression(){
                    if self.compare_to_top(")"){
                        self.consume();
                        if self.statement(){
                            if self.compare_to_top("else"){
                                self.consume();
                                if self.statement(){
                                    println!(" en if statement {:?}",self.file_tokens);
                                    return true;
                                }
                                return false;
                            }
                            return true;
                        }
                    }
                }
            }
        }
        

        false
    }


    pub fn while_statement(&mut self) -> bool {
        //Verifica si es un While
        //Formato EBNF <WhileStatement> ::= while (<Expression>) <Statement>
        if self.compare_to_top("while"){
            self.consume();
            if self.compare_to_top("("){
                self.consume();
                if self.expression(){
                    if self.compare_to_top(")"){
                        self.consume();
                        if self.statement(){
                            return true;
                        }
                    }
                }
                
            }
        }
        false 
    }


    pub fn expression(&mut self) -> bool{
        //Verifica si es una expresion
        //Formato EBNF <Expression> ::= <Conjunction> {'||' <Conjunction> }*
        if self.conjunction() {
            while self.compare_to_top("||"){
                self.consume();
                if self.conjunction() { continue }
                return false;
            }
            return true;
        }

        false
    }

    pub fn conjunction(&mut self) -> bool{

        //Verifica la sintaxis de una relacion.
        //Formato EBNF <Conjunction> ::= <Relation> {'&&' <Relation>}*
        if self.relation(){
            while self.compare_to_top("&&"){
                self.consume();
                if self.relation() { continue }

                return false;
            }

            return true;
        }

        false
    }


    pub fn relation(&mut self) -> bool{
        //Verifica la sintaxis de una addition
        //Formato EBNF <Relation> ::= <Addition> {['>','>=','==','!=','<','<='] <Addition>}*
        if self.addition(){
            while self.compare_to_top(">") || self.compare_to_top(">=") || self.compare_to_top("==") ||
                self.compare_to_top("!=")  || self.compare_to_top("<") || self.compare_to_top("<="){
                    self.consume();
                    if self.addition() { continue }
                    return false;
                }

                return true;
        }

        false
    }


    pub fn addition(&mut self) -> bool{
        //Verifica la sintaxis de una addition

        //Formato <Addition> ::= <Term> {['+','-'] <Term>}*
        if self.term(){ 
            while self.compare_to_top("+") || self.compare_to_top("-"){
                self.consume();
                if self.term() { continue }
                return false;
            }
            return true;
        }
        false
    }

    pub fn term(&mut self) -> bool{
        //verifica la sintaxis de un term

        //Formato EBNF <Term>::= <Negation> {['*','/'] <Negation>}*
        if self.negation(){
            while self.compare_to_top("*") || self.compare_to_top("/"){
                self.consume();
                if self.negation() { continue }
                return false;
            }
            return true;
        }
        false
    }

    pub fn negation(&mut self) -> bool {
        //Validando la negation
        //Formato en ebnf <Negation> ::= [!] <Factor>
        if self.compare_to_top("!"){
            self.consume();
        }

        if self.factor(){ return true; }

        false
    }

    pub fn factor(&mut self) -> bool{

        //Verifica si es un factor valido
        //1. Verifica que sea un identificador o un literal, en caso de serlo lo consume.
        //2. Verifica que sea una expresion.
        //Formato EBNF <Factor> ::= <Identifier> | <Literal> | (<Expression>)
        if self.identifier() || self.literal(){
            self.consume();
            return true;
        }
        else if self.compare_to_top("("){
            self.consume();
            if self.expression(){
                if self.compare_to_top(")"){
                    self.consume();
                    return true;
                }
            }
        }

        false
    }


    pub fn literal(&mut self) -> bool{
        //Verifica si es un litera valido
        //Formato EBNF <Literal> ::= <Boolean> | <Integer>
        if self.boolean(){ return true }
        else if self.integer() { return true }

        false
    }

    pub fn boolean(&mut self)-> bool{
        //Verifica si es un boolean
        //Formato EBNF <Boolean> ::= true | false
        
        self.compare_to_top("true") || self.compare_to_top("false")
    }

    pub fn integer(&mut self) -> bool{
        //Verifica si es un integer
        //Formato EBNF <Integer> ::= <Digit> | <Integer> <Digit>
        if let Some(value) = self.file_tokens.get(0){
            for chr in value.chars(){
                if self.is_digit(chr) { continue }
                return false;
            }

            return true;
        }

        false
    }


    pub fn show(&self){

        println!("{:?}",self.file_tokens);
    }
    
}