use super::field::Field;

pub struct Sudoku {
    pub field: Field,
    pub answer: Option<Field>,
}

impl Sudoku {
    /// ## check
    /// - 問題が適正であるかを判定する
    pub fn check(&self) -> bool {
        // 行の数字の個数
        let mut cnt_row = vec![vec![0; 10]; 10];
        // 列の数字の個数
        let mut cnt_col = vec![vec![0; 10]; 10];
        // ブロックの数字の個数
        let mut cnt_block = vec![vec![0; 10]; 10];

        // カウント
        for i in 0..81 {
            let (r, c) = (i / 9, i % 9);
            let num = self.field.field[r][c] as usize; // セルの値
            cnt_row[r][num] += 1;
            cnt_col[c][num] += 1;
            // ブロックの位置
            let (br, bc) = (r / 3, c / 3);
            let b = 3 * br + bc;
            cnt_block[b][num] += 1;
        }

        // 重複がないかチェック
        let has_no_duplicate = |array: Vec<Vec<usize>>| -> bool {
            array
                .iter()
                .map(|vec| vec[1..].iter().all(|&val| val <= 1))
                .all(|val| val)
        };

        has_no_duplicate(cnt_row) && has_no_duplicate(cnt_col) && has_no_duplicate(cnt_block)
    }

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
