/*
 * This file contains definitions for nodes in an AST representation.  Each
 * of the non-abstract classes defined below has at least one constructor,
 * whose usage is documented with the specific class below.
 *
 * Each non-abstract class below also has a method called generateGVSpec().
 * This function generates GraphView specification code for a single AST node
 * of from the corresponding class.  For all classes, this method takes the
 * following two arguments:
 *
 * @param nodeName A string identifier for this node to be used in the
 *   GraphView specification.  Should be unique with respect to the identifiers
 *   for all other nodes in the specification.
 * @param gvSpec A reference to a running string in which the GraphView
 *   specification for the entire AST is being stored.  The specification for
 *   this node will be appended to the end of this string along with the
 *   specification for any descendants of this node.
 */

#ifndef _AST_HPP
#define _AST_HPP

#include <vector>

/*
 * Abstract class representing a generic node in an AST.
 */
class ASTNode {
public:
  virtual ~ASTNode() {}
  virtual void generateGVSpec(std::string nodeName, std::string& gvSpec) const = 0;
};


/*
 * Abstract class representing a node in an AST specifically corresponding to
 * an expression.
 */
class ASTExpression : public ASTNode {};


/*
 * Class representing a node in an AST corresponding to an identifier.  This
 * is a subclass of the node representing expressions, so identifiers can be
 * treated as expressions.
 *
 * The constructor of this class takes the following argument:
 *
 * @param name A pointer to a string containing the text of the identifier.
 *   This class takes ownership of its argument and frees it in the class
 *   destructor.
 */
class ASTIdentifier : public ASTExpression {
public:
  const std::string* name;
  ASTIdentifier(const std::string* name) : name(name) {}
  virtual ~ASTIdentifier() { delete name; }
  virtual void generateGVSpec(std::string nodeName, std::string& gvSpec) const;
};


/*
 * Class representing a node in an AST corresponding to a floating-point value.
 * This is a subclass of the node representing expressions, so floating-point
 * values can be treated as expressions.
 *
 * The constructor of this class takes the following argument:
 *
 * @param value The numerical value of the node.
 */
class ASTFloat : public ASTExpression {
public:
  float value;
  ASTFloat(float value) : value(value) {}
  virtual void generateGVSpec(std::string nodeName, std::string& gvSpec) const;
};


/*
 * Class representing a node in an AST corresponding to an integer value.  This
 * is a subclass of the node representing expressions, so integer values can
 * be treated as expressions.
 *
 * The constructor of this class takes the following argument:
 *
 * @param value The numerical value of the node.
 */
class ASTInteger : public ASTExpression {
public:
  int value;
  ASTInteger(int value) : value(value) {}
  virtual void generateGVSpec(std::string nodeName, std::string& gvSpec) const;
};


/*
 * Class representing a node in an AST corresponding to a boolean value.  This
 * is a subclass of the node representing expressions, so boolean values can
 * be treated as expressions.
 *
 * The constructor of this class takes the following argument:
 *
 * @param value The boolean value of the node.
 */
class ASTBoolean : public ASTExpression {
public:
  bool value;
  ASTBoolean(bool value) : value(value) {}
  virtual void generateGVSpec(std::string nodeName, std::string& gvSpec) const;
};


/*
 * Class representing a node in an AST corresponding to a binary operation
 * expression.  This is a subclass of the node representing expressions.
 *
 * The constructor of this class takes the following arguments:
 *
 * @param op An integer value representing the type of operation being
 *   performed in this expression.  This value should come from parser.hpp
 *   (e.g. PLUS, MINUS, GTE, etc.).
 * @param lhs The AST node representing the left-hand side of this expression.
 *   This class takes ownership of this argument and frees it in the class
 *   destructor.
 * @param rhs The AST node representing the right-hand side of this expression.
 *   This class takes ownership of this argument and frees it in the class
 *   destructor.
 */
class ASTBinaryOperatorExpression : public ASTExpression {
public:
  int op;
  const ASTExpression* lhs;
  const ASTExpression* rhs;
  ASTBinaryOperatorExpression(int op, const ASTExpression* lhs,
    const ASTExpression* rhs) : op(op), lhs(lhs), rhs(rhs) {}
  virtual ~ASTBinaryOperatorExpression() { delete lhs; delete rhs; }
  virtual void generateGVSpec(std::string nodeName, std::string& gvSpec) const;
};


/*
 * Abstract class representing a node in an AST specifically corresponding to
 * a statement.
 */
class ASTStatement : public ASTNode {};


/*
 * Class representing a node in an AST corresponding to an assignment
 * statement.  This is a subclass of the node representing statements.
 *
 * The constructor of this class takes the following arguments:
 *
 * @param lhs The AST node representing the identifier on the left-hand side
 *   of this assignment statement.  This class takes ownership of this argument
 *   and frees it in the class destructor.
 * @param rhs The AST node representing the expression on the right-hand side
 *   of this assignment statement.  This class takes ownership of this argument
 *   and frees it in the class destructor.
 */
class ASTAssignmentStatement : public ASTStatement {
public:
  const ASTIdentifier* lhs;
  const ASTExpression* rhs;
  ASTAssignmentStatement(const ASTIdentifier* lhs, const ASTExpression* rhs)
    : lhs(lhs), rhs(rhs) {}
  virtual ~ASTAssignmentStatement() { delete lhs; delete rhs; }
  virtual void generateGVSpec(std::string nodeName, std::string& gvSpec) const;
};


/*
 * Class representing a node in an AST corresponding to a block of statements.
 * This is a subclass of the generic AST node.
 *
 * The constructor of this class takes the following argument:
 *
 * @param statement The AST node corresponding to the first statement in this
 *   block.  Additional statements can be added by using
 *   ASTBlock::statements.push_back().  This class takes ownership of all
 *   statements added to the block and frees them in the class destructor.
 */
class ASTBlock : public ASTNode {
public:
  std::vector<const ASTStatement*> statements;
  ASTBlock(const ASTStatement* statement) {
    this->statements.push_back(statement);
  }
  virtual ~ASTBlock() {
    for (int i = 0; i < statements.size(); i++) {
      delete statements[i];
    }
  }
  virtual void generateGVSpec(std::string nodeName, std::string& gvSpec) const;
};





/*
 * Class representing a node in an AST corresponding to an if statement.  This
 * is a subclass of the node representing statements.
 *
 * The constructor for this class takes three arguments:
 *
 * @param condition The AST node representing the expression used as the
 *   condition of this if statement.  This class takes ownership of this
 *   argument and frees it in the class destructor.
 * @param ifBlock The AST node representing the block of statements to be
 *   executed if the condition evaluates as true.  This class takes ownership
 *   of this argument and frees it in the class destructor.
 * @param elseBlock The AST node representing the block of statements to be
 *   executed as the else block for this if statement.  This class takes
 *   ownership of this argument and frees it in the class destructor.
 */
class ASTIfStatement : public ASTStatement {
public:
  const ASTExpression* condition;
  const ASTBlock* ifBlock;
  const ASTBlock* elseBlock;
  ASTIfStatement(const ASTExpression* condition, const ASTBlock* ifBlock,
    const ASTBlock* elseBlock)
    : condition(condition), ifBlock(ifBlock), elseBlock(elseBlock) {}
  virtual ~ASTIfStatement() {
    delete condition;
    delete ifBlock;
    delete elseBlock;
  }
  virtual void generateGVSpec(std::string nodeName, std::string& gvSpec) const;
};


/*
 * Class representing a node in an AST corresponding to a while statement.
 * This is a subclass of the node representing statements.
 *
 * The constructor for this class takes two arguments:
 *
 * @param condition The AST node representing the expression used as the
 *   condition of this while statement.  This class takes ownership of this
 *   argument and frees it in the class destructor.
 * @param ifBlock The AST node representing the block of statements to be
 *   executed at each iteration of this while loop.  This class takes ownership
 *   of this argument and frees it in the class destructor.
 */
class ASTWhileStatement : public ASTStatement {
public:
  const ASTExpression* condition;
  const ASTBlock* whileBlock;
  ASTWhileStatement(const ASTExpression* condition, const ASTBlock* whileBlock)
    : condition(condition), whileBlock(whileBlock) {}
  virtual ~ASTWhileStatement() {
    delete condition;
    delete whileBlock;
  }
  virtual void generateGVSpec(std::string nodeName, std::string& gvSpec) const;
};


/*
 * Class representing a node in an AST corresponding to an break statement.
 * This is a subclass of the node representing statements.
 */
class ASTBreakStatement : public ASTStatement {
  virtual void generateGVSpec(std::string nodeName, std::string& gvSpec) const;
};


/*
 * Function to generate GraphView spec for the subtree rooted at any node in
 * an AST.
 *
 * @param node An AST node.
 *
 * @return Returns a string containing a complete GraphView specification to
 *   visualize the AST subtree rooted at node.
 */
std::string generateGVSpec(ASTNode* node);

#endif
