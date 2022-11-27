


use std::collections::HashMap;
use std::process;

use crate::lexer::Token;

#[derive(Debug)]
pub struct Parser<'a>{

    //Estructura hacer el analisis sintactico
    //Obtiene los tokens definidos en la gramatica
    //Ademas tambien receibe un vector con todos los tokens leidos en el archivo .jay

    pub grammar_tokens : HashMap<&'a str,&'a str>,
    pub file_tokens  : Vec<Token>,
    pub line_control : u32,
    
}



impl<'a> Parser<'_>{
    
    pub fn build(grammar_tokens: &HashMap<&'a str,&'a str>,file_tokens: &Vec<Token>) -> Parser<'a>{

        let gram = grammar_tokens.clone();

        let fil_tokens = file_tokens.clone();

        Parser { grammar_tokens : gram, file_tokens: fil_tokens,line_control : 1 }
    }

    pub fn run(&mut self){
        //Inicia el analisis sintactico por la primera regla de produccion.
        self.program();
    }

    fn consume(&mut self){
        //Remueve el elemento que se encuentre al inicio del vector.
        self.file_tokens.remove(0);
    }

    fn compare_to_top(&self,token : &str) -> bool{
        //Compara el token con lo que se encuentre al inicio del vector.
        if let Some(value) = self.file_tokens.get(0){
            return value.get_value().to_string() == token.to_string()
        }
        false
    }

    fn is_of_type(&self,element : &String,type_of : &str) -> bool {
        if let Some(result) = self.grammar_tokens.get(element.as_str()){
            if result.contains(type_of) { return true }
        }

        false
    }

    
    fn update_line(&mut self){
        if let Some(element) = self.file_tokens.get(0){
            self.line_control = element.get_line();
        }
    }

    fn end_process(&self){
        eprintln!("Linea {}: Error de sintaxis",self.line_control);
        process::exit(0);
    }

    fn program(&mut self){

         //Verifica la regla de produccion de un programa
         //<Program> ::= void main() '{' <Declarations> <Statements> '}'


        if !self.compare_to_top("void"){
            self.end_process();
        }
        self.update_line();
        self.consume();
        if !self.compare_to_top("main"){
            self.end_process();
        }

        
        self.update_line();
        self.consume();


        if !self.compare_to_top("("){
            self.end_process();
        }
        self.update_line();
        self.consume();
        if !self.compare_to_top(")"){
            self.end_process();
        }
        self.update_line();
        self.consume();
        if !self.compare_to_top("{"){
            self.end_process();
        }
        self.update_line();
        self.consume();

        self.declarations();
        self.statements();

        if !self.compare_to_top("}"){
            self.end_process();
        }
        self.update_line();
        self.consume();
        println!("after declarations:");
        for el in &self.file_tokens{
            print!(" asd{} ",el.get_value());
        }
        println!(" ");

    }

    

    fn declarations(&mut self) -> bool{

        //Verifica las declaraciones
        //formato en ebnf <Declarations> ::= < {<Declaration>}*

        if self.declaration(){
            return self.declarations();
        }
        false
    }

    fn declaration(&mut self) -> bool{

        //Verifica si es una declaracion
        //1. Verifica que el primer elemento en el vector de tokens sea un type
        //2. Verifica los identificadores  que siguen luego del tipo de declaracion
        //Formato <Declaration> ::= <Type> <Identifiers>';'

        if !self.declaration_type(){    
            return false;
        }
        
        if !self.identifiers(){
            self.end_process();
            return false;
        }
        if !self.compare_to_top(";"){
            self.end_process();
            return false;
        }
        self.update_line();
        self.consume();
        true

    }

    fn declaration_type(&mut self) -> bool{

        //Verifica si el elemento en el vector contiene un 'INT' o 'Boolean'
        //Formato <Type> ::= int | boolean


        if self.compare_to_top("int") || self.compare_to_top("boolean") {
            self.update_line();
            self.consume();
            return true
        }

        false
    }

    fn identifiers(&mut self) -> bool{
        //Verifica que tenga forma de identificadores
        //Formato <Identifiers> ::= <Identifier> {','<identifier>}*


        if !self.identifier(){ return false; }

        self.update_line();
        self.consume();


        if self.compare_to_top(","){
            self.update_line();
            self.consume();
            return self.identifiers();
        }


        true

    }
    fn identifier(&mut self) -> bool{
        //Verifica que el identificador inicie con  una letra
        //y tenga una secuencia de letras y numero despues del primer caracter.
        //Formato <Identifier> ::= <Letter> | <Identifier> <Letter> | <Identifier> <Digit>


        if let Some(value) = self.file_tokens.get(0){
            
            if self.is_of_type(&value.get_value(),"KEYWORD"){ 
                return false 
            }
            let first_char = value.get_value().chars().next().unwrap().to_string();
            if self.is_of_type(&first_char,"LETTER"){
                let sliced_string = &value.get_value()[1..];
                for chr in sliced_string.chars(){
                    if self.is_of_type(&chr.to_string(),"LETTER") { continue }
                    else if self.is_of_type(&chr.to_string(),"NUMBER") { continue }
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
            return self.statements();
        }

        false

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
            self.update_line();
            self.consume();
        }
        else { return false }



        if self.statements() || self.compare_to_top("}") {
            self.update_line();
            self.consume();

            return true;
        }

        self.end_process();

        false

    }

    pub fn assignment(&mut self) -> bool{
        //Verifica si se trata de una asignacion
        //Primero verifica que sea un identificador valido
        //Que contengan el signo = que indica asignacion
        //y por ultimo verifica la expresion y concluya con ';'
        //Formato <Assignment> ::= <Identifier> '=' <Expression> ';'

        if self.identifier(){
            self.update_line();
            self.consume();
        } else { return false }



        if self.compare_to_top("="){
            self.update_line();
            self.consume();
        } else { self.end_process(); return false }



        if self.expression() && self.compare_to_top(";"){
            self.update_line();
            self.consume();
            return true
        }

        self.end_process();

        false
    }

    pub fn if_statement(&mut self) -> bool{
        //Verifica la sintaxis de un IF
        //Formato EBNF <IfStatement> ::= if (<Expression>) <Statement> [else <Statement>]

        if self.compare_to_top("if"){
            self.update_line();
            self.consume();
        } else { return false }

        if self.compare_to_top("("){
            self.update_line();
            self.consume();
        }else { self.end_process(); return false; }
        if self.expression() && self.compare_to_top(")"){
            self.update_line();
            self.consume();
        } else { self.end_process(); return false }
        
        if self.statement(){

            if self.compare_to_top("else"){
                self.update_line();
                self.consume();
                if self.statement(){ return true }
                self.end_process();
            }

            return true;
        }

        self.end_process();



        false
    }


    pub fn while_statement(&mut self) -> bool {
        //Verifica si es un While
        //Formato EBNF <WhileStatement> ::= while (<Expression>) <Statement>

        if self.compare_to_top("while"){
            self.update_line();
            self.consume();
        } else { return false }


        if self.compare_to_top("("){
            self.update_line();
            self.consume();
        } else { self.end_process(); return false }

        if self.expression() && self.compare_to_top(")"){
            self.update_line();
            self.consume();
        } else { self.end_process(); return false }

        if self.statement(){
            return true;
        }

        self.end_process();
        false 
    }


    pub fn expression(&mut self) -> bool{
        //Verifica si es una expresion
        //Formato EBNF <Expression> ::= <Conjunction> {'||' <Conjunction> }*

        //caso critico, si luego del OR no hay una conjunction es invalido y debe enviar un PANIC.
        if self.conjunction() {
            if self.compare_to_top("||"){
                self.update_line();
                self.consume();
                if !self.conjunction(){ self.end_process(); return false }
            }
            return true;
        }

        false
    }

    pub fn conjunction(&mut self) -> bool{

        //Verifica la sintaxis de una relacion.
        //Formato EBNF <Conjunction> ::= <Relation> {'&&' <Relation>}*
        if self.relation(){
            if self.compare_to_top("&&"){
                self.update_line();
                self.consume();
                if !self.relation() { self.end_process(); return false }
            }
            return true;
        }

        false
    }


    pub fn relation(&mut self) -> bool{
        //Verifica la sintaxis de una addition
        //Formato EBNF <Relation> ::= <Addition> {['>','>=','==','!=','<','<='] <Addition>}*
        if self.addition(){
            if self.compare_to_top(">") || self.compare_to_top(">=") || self.compare_to_top("==") ||
                self.compare_to_top("!=")  || self.compare_to_top("<") || self.compare_to_top("<="){
                    self.update_line();
                    self.consume();
                    if !self.addition() { self.end_process(); return  false }
                }
                return true;
        }

        false
    }


    pub fn addition(&mut self) -> bool{
        //Verifica la sintaxis de una addition

        //Formato <Addition> ::= <Term> {['+','-'] <Term>}*
        if self.term(){ 
            if self.compare_to_top("+") || self.compare_to_top("-"){
                self.update_line();
                self.consume();
                if !self.term() { self.end_process(); return false }
            }
            return true;
        }
        false
    }

    pub fn term(&mut self) -> bool{
        //verifica la sintaxis de un term

        //Formato EBNF <Term>::= <Negation> {['*','/'] <Negation>}*
        if self.negation(){
            if self.compare_to_top("*") || self.compare_to_top("/"){
                self.update_line();
                self.consume();
                if !self.negation() { self.end_process(); return false }
            }
            return true;
        }
        false
    }

    pub fn negation(&mut self) -> bool {
        //Validando la negation
        //Formato en ebnf <Negation> ::= [!] <Factor>
        if self.compare_to_top("!"){
            self.update_line();
            self.consume();
            if self.factor() { return true;}
            self.end_process();
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
            self.update_line();
            self.consume();
            return true;
        }
        else if self.compare_to_top("("){
            self.update_line();
            self.consume();
            if self.expression() && self.compare_to_top(")"){
                self.update_line();
                self.consume();
                return true;
            }
            self.end_process();
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
            for chr in value.get_value().chars(){
                if self.is_of_type(&chr.to_string(),"NUMBER") { continue }
                return false;
            }

            return true;
        }

        false
    }


    pub fn show(&self){

        for token in &self.file_tokens{
            print!("{:?} ",token.get_value().to_string().as_str());
        }
    }
    
}