//===--- UseUsingCheck.cpp - clang-tidy------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "UseUsingCheck.h"
#include "clang/AST/ASTContext.h"
#include "clang/Lex/Lexer.h"

using namespace clang::ast_matchers;

namespace clang {
namespace tidy {
namespace modernize {

UseUsingCheck::UseUsingCheck(StringRef Name, ClangTidyContext *Context)
    : ClangTidyCheck(Name, Context),
      IgnoreMacros(Options.getLocalOrGlobal("IgnoreMacros", true)) {}

void UseUsingCheck::registerMatchers(MatchFinder *Finder) {
  if (!getLangOpts().CPlusPlus11)
    return;
  Finder->addMatcher(typedefDecl(unless(isInstantiated())).bind("typedef"),
                     this);
  // This matcher used to find structs defined in source code within typedefs.
  // They appear in the AST just *prior* to the typedefs.
  Finder->addMatcher(cxxRecordDecl(unless(isImplicit())).bind("struct"), this);
}

void UseUsingCheck::check(const MatchFinder::MatchResult &Result) {
  // Match CXXRecordDecl only to store the range of the last non-implicit full
  // declaration, to later check whether it's within the typdef itself.
  const auto *MatchedCxxRecordDecl =
      Result.Nodes.getNodeAs<CXXRecordDecl>("struct");
  if (MatchedCxxRecordDecl) {
    LastCxxDeclRange = MatchedCxxRecordDecl->getSourceRange();
    return;
  }

  const auto *MatchedDecl = Result.Nodes.getNodeAs<TypedefDecl>("typedef");
  if (MatchedDecl->getLocation().isInvalid())
    return;

  SourceLocation StartLoc = MatchedDecl->getBeginLoc();

  if (StartLoc.isMacroID() && IgnoreMacros)
    return;

  static const char *UseUsingWarning = "use 'using' instead of 'typedef'";

  // Warn at StartLoc but do not fix if there is macro or array.
  if (MatchedDecl->getUnderlyingType()->isArrayType() || StartLoc.isMacroID()) {
    diag(StartLoc, UseUsingWarning);
    return;
  }

  auto printPolicy = PrintingPolicy(getLangOpts());
  printPolicy.SuppressScope = true;
  printPolicy.ConstantArraySizeAsWritten = true;
  printPolicy.UseVoidForZeroParams = false;

  std::string Type = MatchedDecl->getUnderlyingType().getAsString(printPolicy);
  std::string Name = MatchedDecl->getNameAsString();
  SourceRange ReplaceRange = MatchedDecl->getSourceRange();

  // typedefs with multiple comma-separated definitions produce multiple
  // consecutive TypedefDecl nodes whose SourceRanges overlap. Each range starts
  // at the "typedef" and then continues *across* previous definitions through
  // the end of the current TypedefDecl definition.
  std::string Using = "using ";
  if (ReplaceRange.getBegin().isMacroID() ||
      ReplaceRange.getBegin() >= LastReplacementEnd) {
    // This is the first (and possibly the only) TypedefDecl in a typedef. Save
    // Type and Name in case we find subsequent TypedefDecl's in this typedef.
    FirstTypedefType = Type;
    FirstTypedefName = Name;
  } else {
    // This is additional TypedefDecl in a comma-separated typedef declaration.
    // Start replacement *after* prior replacement and separate with semicolon.
    ReplaceRange.setBegin(LastReplacementEnd);
    Using = ";\nusing ";

    // If this additional TypedefDecl's Type starts with the first TypedefDecl's
    // type, make this using statement refer back to the first type, e.g. make
    // "typedef int Foo, *Foo_p;" -> "using Foo = int;\nusing Foo_p = Foo*;"
    if (Type.size() > FirstTypedefType.size() &&
        Type.substr(0, FirstTypedefType.size()) == FirstTypedefType)
      Type = FirstTypedefName + Type.substr(FirstTypedefType.size() + 1);
  }
  if (!ReplaceRange.getEnd().isMacroID())
    LastReplacementEnd = ReplaceRange.getEnd().getLocWithOffset(Name.size());

  auto Diag = diag(ReplaceRange.getBegin(), UseUsingWarning);

  // If typedef contains a full struct/class declaration, extract its full text.
  if (LastCxxDeclRange.isValid() && ReplaceRange.fullyContains(LastCxxDeclRange)) {
    bool Invalid;
    Type =
        Lexer::getSourceText(CharSourceRange::getTokenRange(LastCxxDeclRange),
                             *Result.SourceManager, getLangOpts(), &Invalid);
    if (Invalid)
      return;
  }

  std::string Replacement = Using + Name + " = " + Type;
  Diag << FixItHint::CreateReplacement(ReplaceRange, Replacement);
}

} // namespace modernize
} // namespace tidy
} // namespace clang
