%{
#include <iostream>
#include <vector>
#include <string>

#include "ast.hpp"
#include "parser.hpp"

extern int yylex();
void yyerror(YYLTYPE* loc, const char* err);
bool stobool(std::string* str);

/*
 * Here, programBlock is a global value that will be used to point to the
 * final root node of the AST (which must be a block).
 */
ASTBlock* programBlock = NULL;
%}

/* Enable location tracking. */
%locations

/*
 * We'll need several different types to represent the various source program
 * constructs.  The first three types below are all straightforward.  The last
 * type, str, is for tokens coming in from the scanner.
 */
%union {
  ASTExpression* expr;
  ASTStatement* stmt;
  ASTBlock* block;
  std::string* str;
}

/*
 * Because the lexer can generate more than one token at a time (i.e. DEDENT
 * tokens), we'll use a push parser.
 */
%define api.pure full
%define api.push-pull push

/*
 * These are all of the terminals in our grammar, i.e. the syntactic
 * categories that can be recognized by the lexer.  We'll return each one
 * as a string from the scanner (if we return anything at all), so they all
 * get the string type from our union above.
 */
%token <str> IDENTIFIER
%token <str> FLOAT INTEGER BOOLEAN
%token <str> INDENT DEDENT NEWLINE
%token <str> AND BREAK DEF ELIF ELSE FOR IF NOT OR RETURN WHILE
%token <str> ASSIGN PLUS MINUS TIMES DIVIDEDBY
%token <str> EQ NEQ GT GTE LT LTE
%token <str> LPAREN RPAREN COMMA COLON

/*
 * Here we're assigning types to the nonterminals.  Each type assignment
 * should be straightforward.
 */
%type <expr> expression primary_expression condition
%type <stmt> statement assign_statement if_statement while_statement break_statement
%type <block> statements block else_block

/*
 * Here, we're defining the precedence of the operators.  The ones that appear
 * later have higher precedence.  All of the operators are left-associative
 * except the "not" operator, which is right-associative.
 */
%left OR
%left AND
%left PLUS MINUS
%left TIMES DIVIDEDBY
%left EQ NEQ GT GTE LT LTE
%right NOT

/* This is our goal/start symbol. */
%start program

%%

/*
 * Each of the CFG rules below generates the relevant AST node and returns
 * it as the semantic value of the rule's left-hand side.  Since each of the
 * various nodes becomes incorporated into its parent node in the AST, we
 * don't free the nodes.  The only things we free below are strings from the
 * scanner that we no longer need (i.e. floats, booleans, and integers).
 */
program
  : statements { programBlock = $1; }
  ;

statements
  : statement { $$ = new ASTBlock($1); }
  | statements statement { $$ = $1; $$->statements.push_back($2); }
  ;

statement
  : assign_statement { $$ = $1; }
  | if_statement { $$ = $1; }
  | while_statement { $$ = $1; }
  | break_statement { $$ = $1; }
  ;

primary_expression
  : IDENTIFIER { $$ = new ASTIdentifier($1); }
  | FLOAT { $$ = new ASTFloat(std::stof(*$1)); delete $1; }
  | INTEGER { $$ = new ASTInteger(std::stoi(*$1)); delete $1; }
  | BOOLEAN { $$ = new ASTBoolean(stobool($1)); delete $1; }
  | LPAREN expression RPAREN { $$ = $2; }
  ;

expression
  : primary_expression { $$ = $1; }
  | expression PLUS expression { $$ = new ASTBinaryOperatorExpression(PLUS, $1, $3); }
  | expression MINUS expression { $$ = new ASTBinaryOperatorExpression(MINUS, $1, $3); }
  | expression TIMES expression { $$ = new ASTBinaryOperatorExpression(TIMES, $1, $3); }
  | expression DIVIDEDBY expression { $$ = new ASTBinaryOperatorExpression(DIVIDEDBY, $1, $3); }
  | expression EQ expression { $$ = new ASTBinaryOperatorExpression(EQ, $1, $3); }
  | expression NEQ expression { $$ = new ASTBinaryOperatorExpression(NEQ, $1, $3); }
  | expression GT expression { $$ = new ASTBinaryOperatorExpression(GT, $1, $3); }
  | expression GTE expression { $$ = new ASTBinaryOperatorExpression(GTE, $1, $3); }
  | expression LT expression { $$ = new ASTBinaryOperatorExpression(LT, $1, $3); }
  | expression LTE expression { $$ = new ASTBinaryOperatorExpression(LTE, $1, $3); }
  ;

assign_statement
  : IDENTIFIER ASSIGN expression NEWLINE { $$ = new ASTAssignmentStatement(new ASTIdentifier($1), $3); }
  ;

block
  : INDENT statements DEDENT { $$ = $2; }
  ;

condition
  : expression { $$ = $1; }
  ;

if_statement
  : IF condition COLON NEWLINE block else_block { $$ = new ASTIfStatement($2, $5, $6); }
  ;

else_block
  : %empty { $$ = NULL; }
  | ELSE COLON NEWLINE block { $$ = $4; }


while_statement
  : WHILE condition COLON NEWLINE block { $$ = new ASTWhileStatement($2, $5); }
  ;

break_statement
  : BREAK NEWLINE { $$ = new ASTBreakStatement(); }
  ;

%%

void yyerror(YYLTYPE* loc, const char* err) {
  std::cerr << "Error (line " << loc->first_line << "): " << err << std::endl;
}

/*
 * This function translates a string containing a Python boolean value into
 * the corresponding C++ bool value.
 */
bool stobool(std::string* str) {
  if (*str == "True") {
    return true;
  } else {
    return false;
  }
}
