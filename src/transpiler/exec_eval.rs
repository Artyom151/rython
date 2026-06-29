impl super::Transpiler {
    pub(crate) fn exec_string_literal(&mut self, code: &str) -> String {
        let tokens = crate::lexer::tokenize(code);
        let program = crate::parser::Parser::new(tokens).parse();
        let mut sub_tr = super::Transpiler::new();
        let (defs, stmts) = sub_tr.transpile(&program);
        for var in sub_tr.used_vars {
            self.used_vars.insert(var);
        }
        for (var, ty) in sub_tr.type_map {
            self.type_map.entry(var).or_insert(ty);
        }
        let mut result = String::new();
        if !defs.trim().is_empty() {
            result.push_str(defs.trim());
            result.push('\n');
        }
        result.push_str(stmts.trim());
        result
    }
    pub(crate) fn eval_string_literal(&mut self, code: &str) -> String {
        let tokens = crate::lexer::tokenize(code);
        let program = crate::parser::Parser::new(tokens).parse();
        let mut sub_tr = super::Transpiler::new();
        let (_, stmts) = sub_tr.transpile(&program);
        if stmts.trim().is_empty() {
            "Value::None".to_string()
        } else {
            let trimmed = stmts.trim().trim_end_matches(';');
            format!("({})", trimmed)
        }
    }
}
