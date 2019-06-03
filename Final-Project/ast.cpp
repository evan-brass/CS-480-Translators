#include <sstream>

#include "ast.hpp"
#include "parser.hpp"

/*
 * Simple template function to convert a value of any type to a string
 * representation.  The type must have an insertion operator (i.e. operator<<).
 */
template <class T>
std::string toString(const T& val) {
  std::ostringstream out;
  out << val;
  return out.str();
}


/*
 * Function to generate GraphView spec for the subtree rooted at any node in
 * an AST.
 *
 * @param node An AST node.
 *
 * @return Returns a string containing a complete GraphView specification to
 *   visualize the AST subtree rooted at node.
 */
std::string generateGVSpec(ASTNode* node) {
  std::string gvSpec = "digraph G {\n";
  std::string nodeName = "n0";
  node->generateGVSpec(nodeName, gvSpec);
  gvSpec += "}\n";
  return gvSpec;
}

/****************************************************************************
 **
 ** Below is the implementation of generateGVSpec() for each class defined in
 ** ast.hpp.  Please see the documentation in ast.hpp for a desription of
 ** this function.
 **
 ****************************************************************************/

void ASTIdentifier::generateGVSpec(std::string nodeName, std::string& gvSpec) const {
  gvSpec += "  " + nodeName
    + " [shape=box,label=\"Identifier: " + *this->name + "\"];\n";
}


void ASTFloat::generateGVSpec(std::string nodeName, std::string& gvSpec) const {
  gvSpec += "  " + nodeName
    + " [shape=box,label=\"Float: " + toString(this->value) + "\"];\n";
}


void ASTInteger::generateGVSpec(std::string nodeName, std::string& gvSpec) const {
  gvSpec += "  " + nodeName
    + " [shape=box,label=\"Integer: " + toString(this->value) + "\"];\n";
}


void ASTBoolean::generateGVSpec(std::string nodeName, std::string& gvSpec) const {
  gvSpec += "  " + nodeName
    + " [shape=box,label=\"Boolean: " + toString(this->value) + "\"];\n";
}


void ASTBinaryOperatorExpression::generateGVSpec(std::string nodeName, std::string& gvSpec) const {
  std::string lhsNodeName = nodeName + "_lhs";
  std::string rhsNodeName = nodeName + "_rhs";
  std::string opStr;
  switch (this->op) {
    case PLUS:
      opStr = "PLUS";
      break;
    case MINUS:
      opStr = "MINUS";
      break;
    case TIMES:
      opStr = "TIMES";
      break;
    case DIVIDEDBY:
      opStr = "DIVIDEDBY";
      break;
    case EQ:
      opStr = "EQ";
      break;
    case NEQ:
      opStr = "NEQ";
      break;
    case GT:
      opStr = "GT";
      break;
    case GTE:
      opStr = "GTE";
      break;
    case LT:
      opStr = "LT";
      break;
    case LTE:
      opStr = "LTE";
      break;
  }

  gvSpec += "  " + nodeName + " [label=\"" + opStr + "\"];\n";
  gvSpec += "  " + nodeName + " -> " + lhsNodeName + ";\n";
  this->lhs->generateGVSpec(lhsNodeName, gvSpec);
  gvSpec += "  " + nodeName + " -> " + rhsNodeName + ";\n";
  this->rhs->generateGVSpec(rhsNodeName, gvSpec);
}


void ASTAssignmentStatement::generateGVSpec(std::string nodeName, std::string& gvSpec) const {
  std::string lhsNodeName = nodeName + "_lhs";
  std::string rhsNodeName = nodeName + "_rhs";
  gvSpec += "  " + nodeName + " [label=\"Assignment\"];\n";
  gvSpec += "  " + nodeName + " -> " + lhsNodeName + ";\n";
  this->lhs->generateGVSpec(lhsNodeName, gvSpec);
  gvSpec += "  " + nodeName + " -> " + rhsNodeName + ";\n";
  this->rhs->generateGVSpec(rhsNodeName, gvSpec);
}


void ASTBlock::generateGVSpec(std::string nodeName, std::string& gvSpec) const {
  gvSpec += "  " + nodeName + " [label=\"Block\"];\n";
  for (int i = 0; i < this->statements.size(); i++) {
    std::string childNodeName = nodeName + "_" + toString(i);
    gvSpec += "  " + nodeName + " -> " + childNodeName + ";\n";
    this->statements[i]->generateGVSpec(childNodeName, gvSpec);
  }
}


void ASTIfStatement::generateGVSpec(std::string nodeName, std::string& gvSpec) const {
  std::string conditionNodeName = nodeName + "_cond";
  std::string ifBlockNodeName = nodeName + "_if";
  gvSpec += "  " + nodeName + " [label=\"If\"];\n";
  gvSpec += "  " + nodeName + " -> " + conditionNodeName + ";\n";
  this->condition->generateGVSpec(conditionNodeName, gvSpec);
  gvSpec += "  " + nodeName + " -> " + ifBlockNodeName + ";\n";
  this->ifBlock->generateGVSpec(ifBlockNodeName, gvSpec);

  if (this->elseBlock) {
    std::string elseBlockNodeName = nodeName + "_else";
    gvSpec += "  " + nodeName + " -> " + elseBlockNodeName + ";\n";
    this->elseBlock->generateGVSpec(elseBlockNodeName, gvSpec);
  }
}


void ASTWhileStatement::generateGVSpec(std::string nodeName, std::string& gvSpec) const {
  std::string conditionNodeName = nodeName + "_cond";
  std::string whileBlockNodeName = nodeName + "_while";
  gvSpec += "  " + nodeName + " [label=\"While\"];\n";
  gvSpec += "  " + nodeName + " -> " + conditionNodeName + ";\n";
  this->condition->generateGVSpec(conditionNodeName, gvSpec);
  gvSpec += "  " + nodeName + " -> " + whileBlockNodeName + ";\n";
  this->whileBlock->generateGVSpec(whileBlockNodeName, gvSpec);
}


void ASTBreakStatement::generateGVSpec(std::string nodeName, std::string& gvSpec) const {
  gvSpec += "  " + nodeName + " [label=\"Break\"];\n";
}
