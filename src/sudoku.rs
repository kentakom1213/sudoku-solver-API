use super::field::Field;

pub struct Sudoku {
    pub field: Field,
    pub answer: Option<Field>,
}

impl Sudoku {
    /// ## solve
    /// 数独の解を求める（`Sudoku::dfs`のwrapper）
    pub fn solve(&mut self) {
        let field = self.field.clone();

        // DFSで解を求める
        self.dfs(0);
        self.field = field;
    }

    /// ## dfs
    /// 深さ優先探索により解を求める
    fn dfs(&mut self, cur: usize) {
        // すでに探索が終了している場合
        if self.answer.is_some() {
            return;
        }

        // 解がもとまった場合
        if cur == 81 {
            self.answer = Some(self.field.clone());
            return;
        }

        let (i, j) = (cur / 9, cur % 9);
        if self.field.field[i][j] == 0 {
            // 順に代入する
            for n in 1..=9 {
                // 代入した際に条件を満たすかどうか
                let mut is_ok = true;

                for t in 0..9 {
                    // --- 行 ---
                    is_ok &= self.field.field[i][t] != n;

                    // --- 列 ---
                    is_ok &= self.field.field[t][j] != n;

                    // --- ブロック ---
                    let (r, c) = (i / 3 * 3 + t / 3, j / 3 * 3 + t % 3);
                    is_ok &= self.field.field[r][c] != n;
                }

                // 条件を満たしているとき
                if is_ok {
                    self.field.field[i][j] = n;
                    self.dfs(cur + 1);
                    self.field.field[i][j] = 0;
                }
            }
        } else {
            self.dfs(cur + 1);
        }
    }
}
