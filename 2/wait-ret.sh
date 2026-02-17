#!/bin/bash
# $!で前のプロセスのIDを取得
# $?で終了したプロセスの返り値を取得
false &
wait $!
echo "falseコマンドが終了しました: $?" 

true &
wait $!
echo "trueコマンドが終了しました: $?"
