use ast::Expr;
use crate::codegen::append::*;
use crate::codegen::context::Context;
use crate::codegen::{Inst, InstVec};

pub(crate) fn codegen(program: &ast::Program) -> InstVec {
    let mut context = Context::new();
    program.gen(&mut context)
}

macro_rules! concat {
    ($l:expr, $r:expr) => {
        $l.append($r)
    };

    ($l:expr, $r:expr, $($es:expr),+) => {
        {
            let a = concat!($l, $r);
            concat!(a, $($es),+)
        }
    }
}

fn wrap<T>(inst: T) -> Inst
// TODO replace wrap with .into()
where
    T: pvm::Instruction + 'static,
{
    Box::new(inst)
}

trait AstCodeGen {
    fn gen(&self, context: &mut Context) -> InstVec;
}

impl AstCodeGen for ast::Program {
    fn gen(&self, context: &mut Context) -> InstVec {
        let main_call_inst = wrap(pvm::CallInst::new(pvm::Operand::Label("main".to_string())));
        // TODO popr if main returns int

        let end_label = pvm::Operand::Label("end".to_string());
        let jump_end_inst = wrap(pvm::JumpInst::new(end_label.clone()));

        let mut insts = concat!(main_call_inst, jump_end_inst);

        for f in &self.funs {
            let f_insts = f.gen(context);
            insts = concat!(insts, f_insts);
        }

        let end_label_inst = wrap(pvm::LabelInst::new(end_label.clone()));
        concat!(insts, end_label_inst)
    }
}

impl AstCodeGen for ast::Fun {
    fn gen(&self, context: &mut Context) -> InstVec {
        let mut insts = Vec::new();

        let fun_l = pvm::Operand::Label(self.ident.symbol.borrow().name.clone());
        let fun_i = wrap(pvm::FunInst::new(fun_l));
        insts = concat!(insts, fun_i);

        for p in &self.params {
            let p_insts = p.gen(context);
            insts = concat!(insts, p_insts)
        }

        let b_insts = self.block.gen(context);
        concat!(insts, b_insts)
    }
}

impl AstCodeGen for ast::Param {
    fn gen(&self, context: &mut Context) -> InstVec {
        let dest = self.ident.dest();
        let pop_inst = wrap(pvm::PopaInst::new(dest));
        vec![pop_inst]
    }
}

impl AstCodeGen for ast::LetStmt {
    fn gen(&self, context: &mut Context) -> InstVec {
        let e_insts = self.expr.gen(context);
        let src = self.expr.dest();
        let dest = self.ident.dest();
        let move_inst = wrap(pvm::MoveInst::new(dest, src));
        concat!(e_insts, move_inst)
    }
}

impl AstCodeGen for ast::SetStmt {
    fn gen(&self, context: &mut Context) -> InstVec {
        let e_insts = self.expr.gen(context);
        let src = self.expr.dest();
        let dest = self.ident.dest();
        let move_inst = wrap(pvm::MoveInst::new(dest, src));
        concat!(e_insts, move_inst)
    }
}

impl AstCodeGen for ast::IfStmt {
    fn gen(&self, context: &mut Context) -> InstVec {
        let mut insts = Vec::new();
        let lab_prefix = context.label_gen.if_prefix();
        let then_labels: Vec<pvm::Operand> = (0..self.conds.len())
            .into_iter()
            .map(|i| format!("{}_then{}", lab_prefix, i))
            .map(|l| pvm::Operand::Label(l))
            .collect();

        let end_label = pvm::Operand::Label(format!("{}_end", lab_prefix));
        for ((c, b), l) in self.conds.iter().zip(&self.then_blocks).zip(&then_labels) {
            let c_insts = c.gen(context);
            let b_insts = b.gen(context);

            let jump_inst = wrap(pvm::JumpZeroInst::new(c.dest(), l.clone()));
            let label_inst = wrap(pvm::LabelInst::new(l.clone()));
            let jump_end_inst = wrap(pvm::JumpInst::new(end_label.clone()));
            insts = concat!(
                insts,
                c_insts,
                jump_inst,
                b_insts,
                jump_end_inst,
                label_inst
            );
        }

        if let Some(b) = &self.else_block {
            let b_insts = b.gen(context);
            insts = concat!(insts, b_insts);
        }

        let end_label_inst = wrap(pvm::LabelInst::new(end_label));
        concat!(insts, end_label_inst)
    }
}

impl AstCodeGen for ast::WhileStmt {
    fn gen(&self, context: &mut Context) -> InstVec {
        let lab_prefix = context.label_gen.while_prefix();
        let top_label = pvm::Operand::Label(format!("{}_top", lab_prefix));
        let end_label = pvm::Operand::Label(format!("{}_end", lab_prefix));
        let top_label_inst = wrap(pvm::LabelInst::new(top_label.clone()));
        let end_label_inst = wrap(pvm::LabelInst::new(end_label.clone()));
        let top_jump_inst = wrap(pvm::JumpInst::new(top_label));
        let c_insts = self.cond.gen(context);
        let b_insts = self.block.gen(context);
        let end_jump_inst = wrap(pvm::JumpZeroInst::new(self.cond.dest(), end_label));
        concat!(
            top_label_inst,
            c_insts,
            end_jump_inst,
            b_insts,
            top_jump_inst,
            end_label_inst
        )
    }
}

impl AstCodeGen for ast::ReturnStmt {
    fn gen(&self, context: &mut Context) -> InstVec {
        let mut insts = Vec::new();
        if let Some(expr) = &self.expr {
            let e_insts = expr.gen(context);
            // TODO test print code
            let print_inst = wrap(pvm::PrintiInst::new(expr.dest()));
            let println_inst = wrap(pvm::PrintlnInst::new());

            let pushr_inst = wrap(pvm::PushrInst::new(expr.dest()));
            insts = concat!(insts, e_insts, print_inst, println_inst, pushr_inst);
        }

        let ret_inst = wrap(pvm::RetInst::new());
        concat!(insts, ret_inst)
    }
}

impl AstCodeGen for ast::ExprStmt {
    fn gen(&self, context: &mut Context) -> InstVec {
        self.expr.gen(context)
    }
}

impl AstCodeGen for ast::Block {
    fn gen(&self, context: &mut Context) -> InstVec {
        self.stmts.iter()
            .map(|stmt| stmt.gen(context))
            .flatten()
            .collect()

    }
}

impl AstCodeGen for ast::Stmt {
    fn gen(&self, context: &mut Context) -> InstVec {
        match self {
            ast::Stmt::Let(s) => s.gen(context),
            ast::Stmt::Set(s) => s.gen(context),
            ast::Stmt::If(s) => s.gen(context),
            ast::Stmt::While(s) => s.gen(context),
            ast::Stmt::Return(s) => s.gen(context),
            ast::Stmt::Expr(s) => s.gen(context),
            ast::Stmt::Block(s) => s.gen(context),
        }
    }
}

impl AstCodeGen for ast::IntLitExpr {
    fn gen(&self, _context: &mut Context) -> InstVec {
        let src = pvm::Operand::Constant(pvm::to_u64!(self.value));
        let move_inst = wrap(pvm::MoveInst::new(self.dest.clone(), src));
        vec![move_inst]
    }
}

impl AstCodeGen for ast::FloatLitExpr {
    fn gen(&self, _context: &mut Context) -> InstVec {
        let src = pvm::Operand::Constant(pvm::to_u64!(self.value));
        let move_inst = wrap(pvm::MoveInst::new(self.dest.clone(), src));
        vec![move_inst]
    }
}

impl AstCodeGen for ast::BoolLitExpr {
    fn gen(&self, _context: &mut Context) -> InstVec {
        let src = pvm::Operand::Constant(pvm::to_u64!(self.value as u8));
        let move_inst = wrap(pvm::MoveInst::new(self.dest.clone(), src));
        vec![move_inst]
    }
}

impl AstCodeGen for ast::StringLitExpr {
    fn gen(&self, context: &mut Context) -> InstVec {
        todo!()
    }
}

impl AstCodeGen for ast::IdentExpr {
    fn gen(&self, _context: &mut Context) -> InstVec {
        vec![]
    }
}

impl AstCodeGen for ast::CallExpr {
    fn gen(&self, context: &mut Context) -> InstVec {
        let mut insts = Vec::new();
        let f_insts = self.fun.gen(context);
        insts = concat!(insts, f_insts);
        let mut push_insts = Vec::new();
        for a in &self.args {
            let a_insts = a.gen(context);
            let pusha_inst = wrap(pvm::PushaInst::new(a.dest()));
            push_insts.push(pusha_inst);
            insts = concat!(insts, a_insts);
        }


        let call_inst = wrap(pvm::CallInst::new(self.fun.dest())); // TODO will this work for lambdas?

        // TODO if function has ret val, need popr instruction

        concat!(insts, push_insts, call_inst)
    }
}

impl AstCodeGen for ast::UnaryExpr {
    fn gen(&self, context: &mut Context) -> InstVec {
        let e_insts = self.expr.gen(context);
        // TODO gen instruction from op

        todo!()
    }
}

impl AstCodeGen for ast::BinaryExpr {
    fn gen(&self, context: &mut Context) -> InstVec {
        let l_insts = self.left.gen(context);
        let r_insts = self.right.gen(context);
        // TODO gen instruction from op

        todo!()
    }
}

impl AstCodeGen for ast::Expr {
    fn gen(&self, context: &mut Context) -> InstVec {
        match self {
            Expr::IntLit(e) => e.gen(context),
            Expr::FloatLit(e) => e.gen(context),
            Expr::BoolLit(e) => e.gen(context),
            Expr::StringLit(e) => e.gen(context),
            Expr::Ident(e) => e.gen(context),
            Expr::Call(e) => e.gen(context),
            Expr::Unary(e) => e.gen(context),
            Expr::Binary(e) => e.gen(context),
        }
    }
}
