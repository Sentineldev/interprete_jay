//Alumno : Jesús Figuera
//Cedula : 29.660.012
use std::fs;
use std::process;
use std::collections::HashMap;

//deteccion de errores implementada, pero aun falta los ifs,  whiles y elses.
//tabla de simbolos construida.


mod lexer;
mod parser;
//Estructura Lexica de JAY en EBNF

    //<InputElement> ::= <WhiteSpace> | <Comment> | <Token>
    //<WhiteSpace> ::= space | \t | \n | \f
    //<Comment> ::= //Cualquier cadena que finalice en  \r o \n
    //<Token> ::= <Identifier> | <KeyWord> | <Literal> | <Identifier> <Digit>
    //<Letter> ::= a | b | ... | z | A | B | ... | Z
    //<Digit> ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
    //<KeyWord> ::= boolean | else | if | int | main | void | while
    //<Literal> ::= <Boolean> | <Integer>
    //<Boolean> ::= true | false
    //<Integer> ::= <Digit> | <Integer> <Digit>
    //<Separator> ::= ( | ) | { | } | ; | ,
    //<Operador> ::= = | + | - | * | / | > | >= | == | != | < | <= | && | || | !
    

//Estructura sintactica de  Jay en EBNF
/*

<Program> --> void main()  ́{ ́<Declarations> <Statements>  ́} ́
<Declarations> -->{ <Declaration> }*
<Declaration> --> <Type> <Identifiers>;
<Type> --> int | boolean
<Identifiers> --> <Identifier> { , <Identifier> }*
<Statements> --> { <Statement> }*
<Statement> --> <Block> | <Assignment> | <IfStatement> | <WhileStatement>
<Block> -->  ́{ ́ <Statements>  ́} ́
<Assignment> --> <Identifier> = <Expression>;
<IfStatemenmt> -->if (<Expression>) <Statement> [else <Statement>]
<WhileStatement> --> while ( <Expression> )<Statement>
<Expression> --> <Conjunction> { || <Conjunction> }*
<Conjunction> --> <Relation> { && <Relation> }*
<Relation> --> <Addition> { [ > | >= | == | != | <| <= ] <Addition> }*
<Addition> --> <Term> { [ + | - ] <Term> }*
<Term> --> <Negation> { [ * | / ] <Negation> }*
<Negation> --> [ ! ] <Factor>
Factor --> <Identifier> | <Literal> | (<Expression>)

*/


/*


Se aplico el algoritmo 'Recursive Descent' para hacer la validacion de  sintaxis
Tras varias pruebas aplicadas el programa dio resultados positivos.

Para el analisis lexico no se uso ningun algoritmo en particular, mas que una tabla de hash
con los distintos elementos terminales identificados.

La tabla de simbolos se muestra usando la libreria de rust Tabled para que esta tenga un estilo guapo.

El programa solo evalua asignaciones e if's anidados sin else
es decir.

// esto es evaluado correctamente.
if(expression){
    if(<Statement>){

    }
}else{
    <Statement>
}

Esto no es evaluado correctamente.
if(expression){
    if(<Statement>){

    }else{
        <statement>
    }
}else{
    <Statement>
}

tampoco evalua y hace la iteracion del ciclo while

Nota: Seguire trabajando en este proyecto luego de entregarselo. Intentare implementar ese ciclo iterativo y la evaluacion
correcta de los ifs anidados y demas.

Nota 2: Solo hice uso de dos librerias Para la tabla y la evaluacion de expresiones...
Si esto no estaba permitido por favor hagame saber.
Aplique la mayoria del proyecto solo con herramientas prederterminadas del lenguaje.
Tanto analisis lexico, como sintactico fueron implementados con librerias prederterminadas.
*/







fn generate_tokens() -> HashMap<&'static str,&'static str>{
    //Tokens o lexemas que pertenecen a la gramatica.
    
    let tokens : HashMap<&str,&str> = HashMap::from([
        ("=","ASSIGN_OPERATOR"),("+","PLUS_OPERATOR"),("-","MINUS_OPERATOR"),
        ("*","MULT_OPERATOR"),("/","DIVISION_OPERATOR"),(">","GREATER_THAN_OPERATOR"),
        (">=","GREATER_OR_EQUAL_OPERATOR"),("==","EQUAL_OPERATOR"),("!=","DIFFERENT_OPERATOR"),
        ("<","LESS_THAN_OPERATOR"),("<=","LESS_OR_EQUAL_OPERATOR"),("&&","AND_OPERATOR"),("&","AND_SOLO_OPERATOR"),
        ("||","OR_OPERATOR"),("|","VERTICAL_LINE_OPERATOR"),("!","NOT_OPERATOR"),("(","OPEN_PARENTHESIS_SEPARATOR"),
        (")","CLOSE_PARENTHESIS_SEPARATOR"),("{","OPEN_BRACKET_SEPARATOR"),("}","CLOSE_BRACKET_SEPARATOR"),
        (";","SEMICOLON_SEPARATOR"),(",","COMMA_SEPARATOR"),
        ("false","FALSE_KEYWORD"),("true","TRUE_KEYWORD"),
        ("boolean","BOOLEAN_KEYWORD"),("int","INT_KEYWORD"),("void","VOID_KEYWORD"),
        ("if","IF_KEYWORD"),("else","ELSE_KEYWORD"),("main","MAIN_KEYWORD"),
        ("while","WHILE_KEYWORD"),
        ("a","LETTER"),("b","LETTER"),("c","LETTER"),
        ("d","LETTER"),("e","LETTER"),("f","LETTER"),
        ("g","LETTER"),("h","LETTER"),("i","LETTER"),
        ("j","LETTER"),("k","LETTER"),("l","LETTER"),
        ("m","LETTER"),("n","LETTER"),("ñ","LETTER"),
        ("o","LETTER"),("p","LETTER"),("q","LETTER"),
        ("r","LETTER"),("s","LETTER"),("t","LETTER"),
        ("u","LETTER"),("v","LETTER"),("w","LETTER"),
        ("x","LETTER"),("y","LETTER"),("z","LETTER"),
        ("A","LETTER"),("B","LETTER"),("C","LETTER"),
        ("D","LETTER"),("E","LETTER"),("F","LETTER"),
        ("G","LETTER"),("H","LETTER"),("I","LETTER"),
        ("J","LETTER"),("K","LETTER"),("L","LETTER"),
        ("M","LETTER"),("N","LETTER"),("Ñ","LETTER"),
        ("O","LETTER"),("P","LETTER"),("Q","LETTER"),
        ("R","LETTER"),("S","LETTER"),("T","LETTER"),
        ("U","LETTER"),("V","LETTER"),("W","LETTER"),
        ("X","LETTER"),("Y","LETTER"),("Z","LETTER"),
        ("0","NUMBER"),("1","NUMBER"),("2","NUMBER"),
        ("3","NUMBER"),("4","NUMBER"),("5","NUMBER"),
        ("6","NUMBER"),("7","NUMBER"),("8","NUMBER"),
        ("9","NUMBER"),
    ]);

    tokens

}


fn read_file(file_content :  &String) -> Result<bool,String>{

    //Lee linea por line el contenido del archivo
    //1. Comprueba y extrae los tokens del archivo
    //2. Comprueba que la sintaxis este correcta.


    let tokens = generate_tokens();



    let lexer = lexer::Lexer::build(&tokens);

    if let Ok(value) = lexer.read_file_lines(&file_content){

        for el in &value{
            print!("{} ",el.get_value());
        }
        
        let mut parser = parser::Parser::build(&tokens,&value);
        parser.run();
        parser.show();
    }
    

    Ok(true)
}
    
    

fn main() {

    


    match fs::read_to_string(format!("{}/FUENTE.jay",std::env::current_dir().unwrap().display())){
        Ok(content) => {
            if let Err(value) = read_file(&content){
                eprintln!("{value}");
            }
            
        },
        Err(_error) => {
            eprintln!("File not found or missing permissions. {_error}");
            process::exit(1);
        }
    };
}
