use crate::feature::{Meta, Subtest};
use crate::features::Es2023HashbangGrammar;
impl Meta for Es2023HashbangGrammar {
    fn name(&self) -> &'static str {
        "Hashbang Grammar"
    }
    fn key(&self) -> &'static str {
        "es2023_hashbang_grammar"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2023 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-hashbang/"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#Hashbang_comments"
    }
    fn exec(&self) -> &'static str {
        "try {\n  return !eval('#!/wash/your/hands');\n} catch (e) {\n  return false\n}"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
