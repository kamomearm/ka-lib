#include<bits/stdc++.h>
using namespace std;

// 上限 (ここでは 10^12 に)
const long long LIM = 1000000000000LL;

// 素因数一覧
const vector<int> prs = {2,  3,  5,  7,  11, 13, 17, 19,

                         23, 29, 31, 37, 41, 43, 47,

                         53, 59, 61, 67, 71, 73, 79,

                         83, 89, 97};

// 値, 約数の個数, 指数ベクトル
using Node = tuple<long long, long long, vector<int>>;

int main() {

    // 答え
    vector<Node> res;

    // ヒープ
    priority_queue<Node, vector<Node>, greater<Node>> que;

    // 素因数 prs[i] に対する指数を 1 増やして、キューに push
    auto pushup = [&](long long &val, long long &num, vector<int> &exp, int i) -> bool {
        if (val > LIM / prs[i])
            return false;

        while (i >= exp.size())
            exp.push_back(0);

        val *= prs[i];

        num = num / (exp[i] + 1) * (exp[i] + 2);

        ++exp[i];

        que.push(Node(val, num, exp));

        return true;
    };

    // 初期条件 (一段目を積んでいく)
    long long val = 1, num = 1;
    vector<int> exp;

    que.push(Node(val, num, exp));

    for (int i = 0; i < prs.size(); ++i) {
        if (!pushup(val, num, exp, i))
            break;
    }

    // ループ
    long long maxnum = 0;
    while (!que.empty()) {

        // ヒープから最低値を取り出す
        auto cur = que.top();

        auto [val, num, exp] = cur;

        que.pop();

        // 約数の個数が最高記録更新なら答えに格納する
        if (maxnum < num) {
            maxnum = num;
            res.push_back(cur);
        }

        // 最上段の上に積んでいく
        if (!exp.empty()) {
            // 2 に対する指数
            int e0 = exp[0];

            // 2, 3, 5, ... と指数が e0 である限り、上に積んでいく
            for (int i = 0; i < exp.size(); ++i) {
                if (exp[i] < e0)
                    break;
                if (!pushup(val, num, exp, i))
                    break;
            }
        }
    }

    // 出力
    for (int i = 0; i < res.size(); ++i) {
        auto [val, num, exp] = res[i];
        cout << "|$" << i + 1 << "$|$" << val << "$|$" << num << "$|";
        if (!exp.empty()) cout << "$";

        for (int j = 0; j < exp.size(); ++j) {
            if (j) cout << "*";

            cout << prs[j];

            if (exp[j] > 1) cout << "^" << exp[j];
        }

        if (!exp.empty())
            cout << "$";

        cout << "|" << endl;
    }
}